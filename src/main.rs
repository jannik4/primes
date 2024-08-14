#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::app::AppExit;

fn main() -> AppExit {
    primes::build_app().run()
}
