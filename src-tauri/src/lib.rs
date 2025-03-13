mod command;
mod utils;
use command::{get_app_dir, greet};
use tauri::{
    webview::PageLoadPayload, App, Webview, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");
    let handler = app.handle();
    #[cfg(desktop)]
    {
        handler.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    // 在tauri.conf.json中app windows下配置的
    //   {
    //     "title": "hackernews",
    //     "width": 800,
    //     "height": 600
    //   }
    // 默认会创建lable为main的窗口，要先去掉，不然和代码里加的会冲突
    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app {}", std::env::consts::OS))
            .title("Hacker news")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    webview.open_devtools();

    Ok(())
}

fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page Load on {:?}", webview.label());
}

fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event {:?} on {:?}", event, window.label());
    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("Close requested on {:?}", window.label());
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: Some("tauri_hackernews_app".to_string()),
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
