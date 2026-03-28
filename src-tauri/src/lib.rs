use notify::{Event, RecursiveMode, Watcher, EventKind,};
use std::{path::Path, sync::mpsc, fs};
use tauri::{AppHandle, Emitter, Manager};
use serde::Deserialize;

#[derive(Deserialize)]
struct Rule {
    file: String,
    patterns: Vec<String>,
}

#[derive(Deserialize)]
struct Config {
    rules: Vec<Rule>,
}

fn watch_file(app: AppHandle, path: String, patterns: Vec<String>) -> Result<(), String> {
    println!("[DEBUG] Watching {}", path);

    if let Ok(content) = fs::read_to_string(&path) {
        if is_dangerous(&content, &patterns) {
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
                Ok(event) => handle_event(&app, event, &patterns),
                Err(e) => println!("[DEBUG] watch error: {:?}", e),
            }
        }
    });

    Ok(())
}

fn handle_event(app: &AppHandle, event: Event, patterns: &[String]) {
    match event.kind {
        EventKind::Modify(..) => {
            for file in event.paths {
                let file_content_res = fs::read_to_string(&file);
                match file_content_res {
                    Ok(file_content) => {
                        if is_dangerous(&file_content, patterns) {
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

fn is_dangerous(content: &str, patterns: &[String]) -> bool {
    content.lines().any(|line| {
        let trimmed_line = line.trim();
        return patterns.iter().any(|pattern| trimmed_line.contains(pattern)) && !trimmed_line.starts_with("#");
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_ignore_cursor_events(true).unwrap();
            load_config().rules.iter().for_each(|rule| {
                watch_file(app.handle().clone(), rule.file.clone(), rule.patterns.clone()).unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn load_config() -> Config {
    if let Ok(content) = fs::read_to_string("config.json") {
        return serde_json::from_str(&content).unwrap();
    }
    panic!("Failed to load config.json");
}
