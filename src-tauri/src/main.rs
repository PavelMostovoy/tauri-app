#![cfg_attr(
all(not(debug_assertions), target_os = "macos"),
windows_subsystem = "windows"
)]

use std::env;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    account_name: Option<String>,
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_to_db(name: &str) -> String {
    let connection = sqlite::open(get_db_file_location()).unwrap();
     connection
    .execute(
        format!("
        INSERT INTO users VALUES ('{name}', 42);
        "),
    )
    .unwrap();

    format!("Hello, {}! Name added to the DB ", name)
}

fn get_db_file_location() -> String {
    let file_name: String;
    match env::current_exe() {
        Ok(exe_path) => {
            let mut path = exe_path;
            path.pop();
            path.push("local.db");
            file_name = format!("{}", path.display());
            print!("{}", &file_name);
        }
        Err(_) => { file_name = "local.db".to_string() }
    };

    file_name
}


fn init_db_connect() {
    let db_name = get_db_file_location();

    let connection = sqlite::open(&db_name).unwrap();

   connection
    .execute(
        "
        CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
        ",
    )
    .unwrap();
}

fn main() {
    init_db_connect();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_to_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

