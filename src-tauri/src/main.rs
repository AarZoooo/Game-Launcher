// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // The desktop entry point stays small and forwards to the app setup in lib.rs.
    launcher_lib::run()
}
