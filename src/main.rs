#![cfg_attr(
    all(not(debug_assertions), not(feature = "dev")),
    windows_subsystem = "windows"
)]

use bevy::app::AppExit;

fn main() -> AppExit {
    primes::build_app().run()
}
