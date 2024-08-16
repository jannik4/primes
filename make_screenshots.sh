#!/bin/bash

# Array of zoom levels
zoom_levels=(0 1 4 -2 -6)

# Array of resolutions
resolutions=(
  "1920 1080"
  "3440 1440"
  "1080 1920"
  "1080 2340"
)

# Time value
time=5600

# Loop through each resolution
for resolution in "${resolutions[@]}"; do
  width=$(echo $resolution | awk '{print $1}')
  height=$(echo $resolution | awk '{print $2}')
  
  # Loop through each zoom level
  for zoom in "${zoom_levels[@]}"; do
    cargo run -r -- screenshot --width $width --height $height --time $time --zoom $zoom
  done
done
