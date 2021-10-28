#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rand::Rng;

#[tauri::command]
fn random_int() -> i32 {
  let mut rng = rand::thread_rng();

  rng.gen_range(-1000..1000)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![random_int])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
