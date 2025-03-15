mod command;
mod utils;

use command::{get_app_dir, greet};
use tauri::{
    menu::{CheckMenuItem, Menu, MenuBuilder, MenuItem, SubmenuBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview::PageLoadPayload,
    App, AppHandle, Manager, Runtime, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

const APP_NAME: &str = "Hacker News App";

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

    setup_menu(handler)?;
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

fn setup_menu<R: Runtime>(app: &AppHandle<R>) -> Result<(), tauri::Error> {
    let icon = app.default_window_icon().unwrap().clone();

    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        .item(&MenuItem::with_id(
            app,
            "open",
            "Open",
            true,
            Some("Ctrl+O"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "save",
            "Save",
            true,
            Some("Ctrl+S"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "saveas",
            "Save As",
            true,
            Some("Ctrl+Q"),
        )?)
        .separator()
        .quit()
        .build()?;

    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        .item(&MenuItem::with_id(
            app,
            "process",
            "process",
            true,
            Some("Ctrl+P"),
        )?)
        .separator()
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .select_all()
        .item(&CheckMenuItem::with_id(
            app,
            "checkme",
            "Check Me",
            true,
            true,
            None::<&str>,
        )?)
        .build()?;

    // let tray_menu = SubmenuBuilder::with_id(app, "tray", "Tray")
    //     .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>)?)
    //     .item(&MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?)
    //     .separator()
    //     .quit()
    //     .build()?;

    let tray_menu1 = MenuBuilder::with_id(app, "tray")
        .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>)?)
        .item(&MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?)
        .separator()
        .quit()
        .build()?;

    TrayIconBuilder::with_id(format!("{}-tray", APP_NAME))
        .tooltip("Hacker news")
        .icon(icon)
        .menu(&tray_menu1)
        .show_menu_on_left_click(true)
        .on_tray_icon_event(|tray, event| {
            debug!("Tray icon event: {:?}", event);
            if let TrayIconEvent::Click {
                button: MouseButton::Right,
                ..
            } = event
            {
                open_main(tray.app_handle()).unwrap();
            }
        })
        .build(app)?;

    let menu = Menu::with_items(app, &[&file_menu, &edit_menu])?;
    app.set_menu(menu)?;
    app.on_menu_event(|app, event| {
        info!("menu event: {:?}", event);
        match event.id.as_ref() {
            "open" => open_main(app).unwrap(),
            "save" => {},
            "saveas" => {},
            "process" => {},
            "checkme" => {},
            _ => {},
        }
    });

    Ok(())
}

fn open_main<R: Runtime>(handler: &AppHandle<R>) -> Result<(), tauri::Error> {
    handler
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WindowNotFound)?
        .show()
        .unwrap();
    Ok(())
}
