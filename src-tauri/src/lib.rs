use notify::{Event, RecursiveMode, Watcher, EventKind,};
use std::{path::Path, sync::mpsc, fs};
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
fn watch_file(app: AppHandle, path: String) -> Result<(), String> {
    println!("[DEBUG] Watching {}", path);

    if let Ok(content) = fs::read_to_string(&path) {
        if is_dangerous(&content) {
            println!("DANGEROUS at startup !!");
            app.emit("dangerous", {}).unwrap();
        } else {
            println!("SAFE at startup");
            app.emit("safe", {}).unwrap();
        }
    }
    
    std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
        let mut watcher = notify::recommended_watcher(tx).expect("failed to create watcher");
        watcher.watch(Path::new(&path), RecursiveMode::NonRecursive).expect("failed to watch file");
        for res in rx {
            match res {
                Ok(event) => handle_event(&app, event),
                Err(e) => println!("[DEBUG] watch error: {:?}", e),
            }
        }
    });

    Ok(())
}

fn handle_event(app: &AppHandle, event: Event) {
    match event.kind {
        EventKind::Modify(..) => {
            for file in event.paths {
                let file_content_res = fs::read_to_string(&file);
                match file_content_res {
                    Ok(file_content) => {
                        if is_dangerous(&file_content) {
                            println!("DANGEROUS !!");
                            app.emit("dangerous", {}).unwrap();
                        } else {
                            println!("SAFE !!");
                            app.emit("safe", {}).unwrap();
                        }
                    },
                    Err(_e) => println!("[DEBUG] Error reading file {:?}", file)
                }
                
            }
        },
        _ => {}
    }
}

fn is_dangerous(content: &str) -> bool {
    content.lines().any(|line| {
        let trimmed_line = line.trim();
        return trimmed_line.contains("POSTGRES_HOST") && trimmed_line.contains("pgsql-prod-frc.postgres.database.azure.com") && !trimmed_line.starts_with("#");
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_ignore_cursor_events(true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![watch_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
