#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use single_instance::SingleInstance;

fn main() {
    let instance = SingleInstance::new("whiteout-026d707e").unwrap();
    assert!(instance.is_single());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let window = tauri::WindowBuilder::new(
                app,
                "base", /* the unique window label */
                tauri::WindowUrl::App("index.html".into()),
            )
            .always_on_top(true)
            .decorations(false)
            .build()?;
            let monitors = window.available_monitors()?;
            let monitor = monitors.first().unwrap();
            let monitor_pos = monitor.position();
            window.set_position(*monitor_pos)?;
            window.set_fullscreen(true)?;

            for (i, monitor) in monitors.iter().skip(1).enumerate() {
                let window = tauri::WindowBuilder::new(
                    app,
                    i.to_string(), /* the unique window label */
                    tauri::WindowUrl::App("index.html".into()),
                )
                .always_on_top(true)
                .decorations(false)
                .build()?;
                let monitor_pos = monitor.position();
                window.set_position(*monitor_pos)?;
                window.set_fullscreen(true)?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
