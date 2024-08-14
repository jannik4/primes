use super::{GameTime, Zoom};
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        query::QueryItem,
        system::{lifetimeless::*, SystemParamItem},
    },
    math::FloatOrd,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        mesh::{GpuBufferInfo, GpuMesh, MeshVertexBufferLayoutRef},
        render_asset::RenderAssets,
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
            RenderCommandResult, SetItemPipeline, TrackedRenderPass, ViewSortedRenderPhases,
        },
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        view::ExtractedView,
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::{
        Mesh2dPipeline, Mesh2dPipelineKey, RenderMesh2dInstances, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
};
use bytemuck::{Pod, Zeroable};
use std::sync::{Arc, OnceLock};

#[derive(Component)]
pub struct InstanceMaterialData {
    data: Arc<Vec<InstanceData>>,
    buffer: Arc<OnceLock<InstanceBuffer>>,
}

impl InstanceMaterialData {
    pub fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        Self {
            data: Arc::new(iter.into_iter().map(InstanceData).collect()),
            buffer: Arc::new(OnceLock::new()),
        }
    }
}

impl ExtractComponent for InstanceMaterialData {
    type QueryData = &'static InstanceMaterialData;
    type QueryFilter = ();
    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::QueryData>) -> Option<Self> {
        Some(InstanceMaterialData {
            data: Arc::clone(&item.data),
            buffer: Arc::clone(&item.buffer),
        })
    }
}

pub struct InstancedPlugin;

impl Plugin for InstancedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent2d, DrawCustom>()
            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
            .add_systems(ExtractSchedule, extract_globals)
            .add_systems(
                Render,
                (
                    queue_custom.in_set(RenderSet::QueueMeshes),
                    prepare_instance_buffers.in_set(RenderSet::PrepareResources),
                    prepare_gpu_data.in_set(RenderSet::PrepareResources),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<CustomPipeline>()
            .init_resource::<GlobalsGpuData>();
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData(u32);

fn queue_custom(
    transparent_2d_draw_functions: Res<DrawFunctions<Transparent2d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    meshes: Res<RenderAssets<GpuMesh>>,
    render_mesh_instances: ResMut<RenderMesh2dInstances>,
    material_meshes: Query<Entity, With<InstanceMaterialData>>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    mut views: Query<(Entity, &ExtractedView)>,
) {
    let draw_custom = transparent_2d_draw_functions.read().id::<DrawCustom>();

    let msaa_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples());

    for (view_entity, view) in &mut views {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view_entity) else {
            continue;
        };

        let view_key = msaa_key | Mesh2dPipelineKey::from_hdr(view.hdr);
        for entity in &material_meshes {
            let Some(mesh_instance) = render_mesh_instances.get(&entity) else {
                continue;
            };
            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id) else {
                continue;
            };
            let key =
                view_key | Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());
            let pipeline = pipelines
                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
                .unwrap();
            transparent_phase.add(Transparent2d {
                sort_key: FloatOrd(mesh_instance.transforms.world_from_local.translation.z),
                entity,
                pipeline,
                draw_function: draw_custom,
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::NONE,
            });
        }
    }
}

struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

fn prepare_instance_buffers(
    query: Query<(Entity, &InstanceMaterialData)>,
    render_device: Res<RenderDevice>,
) {
    for (_entity, instance_data) in &query {
        instance_data.buffer.get_or_init(|| InstanceBuffer {
            buffer: render_device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("instance data buffer"),
                contents: bytemuck::cast_slice(instance_data.data.as_slice()),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            }),
            length: instance_data.data.len(),
        });
    }
}

#[derive(Resource, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Globals {
    elapsed_seconds: f32,
    zoom: f32,
}

fn extract_globals(
    mut commands: Commands,
    game_time: Extract<Option<Res<GameTime>>>,
    zoom: Extract<Option<Res<Zoom>>>,
) {
    commands.insert_resource(Globals {
        elapsed_seconds: match game_time.as_ref() {
            Some(game_time) => game_time.elapsed.as_secs_f32(),
            None => 0.0,
        },
        zoom: match zoom.as_ref() {
            Some(zoom) => zoom.current,
            None => 1.0,
        },
    });
}

#[derive(Resource)]
struct GlobalsGpuData {
    buffer: Buffer,
    bind_group: BindGroup,
}

impl FromWorld for GlobalsGpuData {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let custom_pipeline = world.resource::<CustomPipeline>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("globals buffer"),
            size: std::mem::size_of::<Globals>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bind_group = render_device.create_bind_group(
            "globals bind group",
            &custom_pipeline.globals_layout,
            &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(buffer.as_entire_buffer_binding()),
            }],
        );

        Self { buffer, bind_group }
    }
}

fn prepare_gpu_data(
    globals: Res<Globals>,
    globals_gpu_data: Res<GlobalsGpuData>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(
        &globals_gpu_data.buffer,
        0,
        bytemuck::cast_slice(&[*globals]),
    );
}

#[derive(Resource)]
struct CustomPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: Mesh2dPipeline,
    globals_layout: BindGroupLayout,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let mesh_pipeline = world.resource::<Mesh2dPipeline>();

        CustomPipeline {
            shader: world.load_asset("shader.wgsl"),
            mesh_pipeline: mesh_pipeline.clone(),
            globals_layout: render_device.create_bind_group_layout(
                "globals layout",
                &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            ),
        }
    }
}

impl SpecializedMeshPipeline for CustomPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();

        descriptor.layout.insert(2, self.globals_layout.clone());

        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![VertexAttribute {
                format: VertexFormat::Uint32,
                offset: 0,
                shader_location: 3, // shader locations 0-2 are taken up by Position, Normal and UV attributes
            }],
        });

        Ok(descriptor)
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawMeshInstanced,
);

struct DrawMeshInstanced;

impl<P: PhaseItem> RenderCommand<P> for DrawMeshInstanced {
    type Param = (
        SRes<GlobalsGpuData>,
        SRes<RenderAssets<GpuMesh>>,
        SRes<RenderMesh2dInstances>,
    );
    type ViewQuery = ();
    type ItemQuery = Read<InstanceMaterialData>;

    #[inline]
    fn render<'w>(
        item: &P,
        _view: (),
        instance_material_data: Option<&'w InstanceMaterialData>,
        (globals, meshes, render_mesh_instances): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let globals = globals.into_inner();
        let Some(mesh_instance) = render_mesh_instances.get(&item.entity()) else {
            return RenderCommandResult::Failure;
        };
        let Some(gpu_mesh) = meshes.into_inner().get(mesh_instance.mesh_asset_id) else {
            return RenderCommandResult::Failure;
        };
        let Some(instance_material_data) = instance_material_data else {
            return RenderCommandResult::Failure;
        };
        let instance_buffer = instance_material_data.buffer.get().unwrap();

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));

        pass.set_bind_group(2, &globals.bind_group, &[]);

        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                index_format,
                count,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed => {
                pass.draw(0..gpu_mesh.vertex_count, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}
