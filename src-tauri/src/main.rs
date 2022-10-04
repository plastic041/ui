#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::__cmd__make_posts;
use app::__cmd__show_posts;
use app::establish_connection;
use app::posts::make_posts::make_posts;
use app::posts::show_posts::show_posts;

fn main() {
    let connection = establish_connection();
    tauri::Builder::default()
        .manage(connection)
        .invoke_handler(tauri::generate_handler![show_posts, make_posts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
