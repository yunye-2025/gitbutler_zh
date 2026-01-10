use anyhow::Context as _;
use but_api::json::Error;
use but_error::Code;
use but_settings::AppSettingsWithDiskSync;
#[cfg(target_os = "macos")]
use tauri::menu::AboutMetadata;
use tauri::{
    AppHandle, Emitter, Manager, Runtime, WebviewWindow,
    menu::{Menu, MenuEvent, MenuItemBuilder, PredefinedMenuItem, Submenu, SubmenuBuilder},
};
use tracing::instrument;

static SHORTCUT_EVENT: &str = "menu://shortcut";

#[tauri::command(async)]
#[instrument(skip(handle), err(Debug))]
pub fn menu_item_set_enabled(handle: AppHandle, id: &str, enabled: bool) -> Result<(), Error> {
    let window = handle
        .get_window("main")
        .expect("main window always present");

    let menu_item = window
        .menu()
        .context("menu not found")?
        .get(id)
        .with_context(|| but_error::Context::new(format!("menu item not found: {id}")))?;

    menu_item
        .as_menuitem()
        .context(Code::Unknown)?
        .set_enabled(enabled)
        .context(Code::Unknown)?;

    Ok(())
}

pub fn build<R: Runtime>(
    handle: &AppHandle<R>,
    #[cfg_attr(target_os = "linux", allow(unused_variables))] settings: &AppSettingsWithDiskSync,
) -> tauri::Result<Menu<R>> {
    #[cfg(not(feature = "disable-auto-updates"))]
    let check_for_updates =
        MenuItemBuilder::with_id("global/update", "检查更新…").build(handle)?;

    #[cfg(target_os = "macos")]
    let app_name = handle
        .config()
        .product_name
        .clone()
        .context("App name not defined.")?;

    #[cfg(target_os = "macos")]
    let settings_menu = MenuItemBuilder::with_id("global/settings", "设置")
        .accelerator("CmdOrCtrl+,")
        .build(handle)?;

    #[cfg(target_os = "macos")]
    let mac_menu = {
        #[cfg_attr(feature = "disable-auto-updates", allow(unused_mut))]
        let mut menu = SubmenuBuilder::new(handle, app_name)
            .about(Some(AboutMetadata::default()))
            .separator()
            .item(&settings_menu);

        #[cfg(not(feature = "disable-auto-updates"))]
        {
            menu = menu.item(&check_for_updates);
        }
        menu.separator()
            .services()
            .separator()
            .hide()
            .hide_others()
            .show_all()
            .separator()
            .quit()
            .build()?
    };

    let file_menu = &SubmenuBuilder::new(handle, "文件")
        .items(&[
            &MenuItemBuilder::with_id("file/add-local-repo", "添加本地仓库…")
                .accelerator("CmdOrCtrl+O")
                .build(handle)?,
            &MenuItemBuilder::with_id("file/clone-repo", "克隆仓库…")
                .accelerator("CmdOrCtrl+Shift+O")
                .build(handle)?,
            &PredefinedMenuItem::separator(handle)?,
            &MenuItemBuilder::with_id("file/create-branch", "创建分支…")
                .accelerator("CmdOrCtrl+B")
                .build(handle)?,
            &MenuItemBuilder::with_id("file/create-dependent-branch", "创建依赖分支…")
                .accelerator("CmdOrCtrl+Shift+B")
                .build(handle)?,
            &PredefinedMenuItem::separator(handle)?,
        ])
        .build()?;

    #[cfg(target_os = "macos")]
    file_menu.append(&PredefinedMenuItem::close_window(handle, None)?)?;

    if cfg!(not(target_os = "macos")) {
        file_menu.append_items(&[&PredefinedMenuItem::quit(handle, None)?])?;
        #[cfg(not(feature = "disable-auto-updates"))]
        file_menu.append_items(&[&check_for_updates])?;
    }

    #[cfg(not(target_os = "linux"))]
    let edit_menu = &Submenu::new(handle, "编辑", true)?;

    // For now, only on MacOS. Once mainstream, we'd have to set the accelerators correctly and test it more.
    #[cfg(not(target_os = "linux"))]
    if settings.get()?.feature_flags.undo && cfg!(target_os = "macos") {
        edit_menu.append_items(&[
            &MenuItemBuilder::with_id("edit/undo", "撤销")
                .accelerator("CmdOrCtrl+Z")
                .build(handle)?,
            &MenuItemBuilder::with_id("edit/redo", "重做")
                .accelerator("CmdOrCtrl+Shift+Z")
                .build(handle)?,
            &PredefinedMenuItem::separator(handle)?,
        ])?;
    }

    #[cfg(not(target_os = "linux"))]
    {
        edit_menu.append_items(&[
            &PredefinedMenuItem::cut(handle, None)?,
            &PredefinedMenuItem::copy(handle, None)?,
            &PredefinedMenuItem::paste(handle, None)?,
        ])?;
    }

    let view_menu = &Submenu::new(handle, "视图", true)?;

    #[cfg(target_os = "macos")]
    view_menu.append(&PredefinedMenuItem::fullscreen(handle, None)?)?;
    view_menu.append_items(&[
        &MenuItemBuilder::with_id("view/switch-theme", "切换主题")
            .accelerator("CmdOrCtrl+T")
            .build(handle)?,
        &MenuItemBuilder::with_id("view/toggle-sidebar", "切换未分配面板")
            .accelerator("CmdOrCtrl+\\")
            .build(handle)?,
        &PredefinedMenuItem::separator(handle)?,
        &MenuItemBuilder::with_id("view/zoom-in", "放大")
            .accelerator("CmdOrCtrl+=")
            .build(handle)?,
        &MenuItemBuilder::with_id("view/zoom-out", "缩小")
            .accelerator("CmdOrCtrl+-")
            .build(handle)?,
        &MenuItemBuilder::with_id("view/zoom-reset", "重置缩放")
            .accelerator("CmdOrCtrl+0")
            .build(handle)?,
        &PredefinedMenuItem::separator(handle)?,
    ])?;

    #[cfg(any(debug_assertions, feature = "devtools"))]
    view_menu.append_items(&[
        &MenuItemBuilder::with_id("view/devtools", "开发者工具")
            .accelerator("CmdOrCtrl+Shift+C")
            .build(handle)?,
        &MenuItemBuilder::with_id("view/reload", "重新加载视图")
            .accelerator("CmdOrCtrl+R")
            .build(handle)?,
    ])?;

    let mut project_menu_builder = SubmenuBuilder::new(handle, "项目")
        .item(
            &MenuItemBuilder::with_id("project/history", "操作历史")
                .accelerator("CmdOrCtrl+Shift+H")
                .build(handle)?,
        )
        .separator()
        .text("project/open-in-vscode", "在编辑器中打开");

    #[cfg(target_os = "macos")]
    {
        project_menu_builder =
            project_menu_builder.text("project/show-in-finder", "在 Finder 中显示");
    }

    #[cfg(target_os = "windows")]
    {
        project_menu_builder =
            project_menu_builder.text("project/show-in-finder", "在资源管理器中显示");
    }

    #[cfg(target_os = "linux")]
    {
        project_menu_builder =
            project_menu_builder.text("project/show-in-finder", "在文件管理器中显示");
    }

    let project_menu = &project_menu_builder
        .separator()
        .text("project/settings", "项目设置")
        .build()?;

    #[cfg(target_os = "macos")]
    let window_menu = &SubmenuBuilder::new(handle, "窗口")
        .items(&[
            &PredefinedMenuItem::minimize(handle, None)?,
            &PredefinedMenuItem::maximize(handle, None)?,
            &PredefinedMenuItem::separator(handle)?,
            &PredefinedMenuItem::close_window(handle, None)?,
        ])
        .build()?;

    let help_menu = SubmenuBuilder::new(handle, "帮助")
        .text("help/documentation", "文档")
        .text("help/debugging-guide", "调试指南")
        .text("help/github", "源代码")
        .text("help/release-notes", "发布说明")
        .separator()
        .text("help/share-debug-info", "分享调试信息…")
        .text("help/report-issue", "创建问题反馈")
        .separator()
        .text("help/open-logs-folder", "打开日志文件夹")
        .text("help/open-config-folder", "打开配置文件夹")
        .separator()
        .text("help/discord", "Discord")
        .text("help/youtube", "YouTube")
        .text("help/bluesky", "Bluesky")
        .text("help/x", "X")
        .separator()
        .item(
            &MenuItemBuilder::with_id(
                "help/version",
                format!("Version {}", handle.package_info().version),
            )
            .enabled(false)
            .build(handle)?,
        )
        .build()?;

    Menu::with_items(
        handle,
        &[
            #[cfg(target_os = "macos")]
            &mac_menu,
            file_menu,
            #[cfg(not(target_os = "linux"))]
            edit_menu,
            view_menu,
            project_menu,
            #[cfg(target_os = "macos")]
            window_menu,
            &help_menu,
        ],
    )
}

/// `handle` is needed to access the undo queue, and buttons for that are only available when the `undo` feature is enabled.
pub fn handle_event<R: Runtime>(handle: &AppHandle<R>, webview: &WebviewWindow, event: &MenuEvent) {
    if event.id() == "edit/undo" {
        eprintln!("use app undo queue to undo.");
        return;
    }
    if event.id() == "edit/redo" {
        eprintln!("use app undo queue to redo.");
        return;
    }
    if event.id() == "file/add-local-repo" {
        emit(webview, "menu://shortcut", "add-local-repo");
        return;
    }

    if event.id() == "file/clone-repo" {
        emit(webview, SHORTCUT_EVENT, "clone-repo");
        return;
    }

    if event.id() == "file/create-branch" {
        emit(webview, SHORTCUT_EVENT, "create-branch");
        return;
    }

    if event.id() == "file/create-dependent-branch" {
        emit(webview, SHORTCUT_EVENT, "create-dependent-branch");
        return;
    }

    #[cfg(any(debug_assertions, feature = "devtools"))]
    {
        if event.id() == "view/devtools" {
            if webview.is_devtools_open() {
                webview.close_devtools();
            } else {
                webview.open_devtools();
            }
            return;
        }
    }

    if event.id() == "view/switch-theme" {
        emit(webview, SHORTCUT_EVENT, "switch-theme");
        return;
    }

    if event.id() == "view/toggle-sidebar" {
        emit(webview, SHORTCUT_EVENT, "toggle-sidebar");
        return;
    }

    if event.id() == "view/reload" {
        emit(webview, SHORTCUT_EVENT, "reload");
        return;
    }

    if event.id() == "view/zoom-in" {
        emit(webview, SHORTCUT_EVENT, "zoom-in");
        return;
    }

    if event.id() == "view/zoom-out" {
        emit(webview, SHORTCUT_EVENT, "zoom-out");
        return;
    }

    if event.id() == "view/zoom-reset" {
        emit(webview, SHORTCUT_EVENT, "zoom-reset");
        return;
    }

    if event.id() == "help/share-debug-info" {
        emit(webview, SHORTCUT_EVENT, "share-debug-info");
        return;
    }

    if event.id() == "project/history" {
        emit(webview, SHORTCUT_EVENT, "history");
        return;
    }

    if event.id() == "project/open-in-vscode" {
        emit(webview, SHORTCUT_EVENT, "open-in-vscode");
        return;
    }

    if event.id() == "project/show-in-finder" {
        emit(webview, SHORTCUT_EVENT, "show-in-finder");
        return;
    }

    if event.id() == "project/settings" {
        emit(webview, SHORTCUT_EVENT, "project-settings");
        return;
    }

    if event.id() == "global/settings" {
        emit(webview, SHORTCUT_EVENT, "global-settings");
        return;
    }

    if event.id() == "global/update" {
        emit(webview, SHORTCUT_EVENT, "update");
        return;
    }

    if event.id() == "help/open-logs-folder" {
        if let Err(err) = crate::debug::open_logs_folder(handle) {
            tracing::error!(error = ?err, "failed to open logs folder");
        }
        return;
    }

    if event.id() == "help/open-config-folder" {
        if let Err(err) = crate::debug::open_config_folder(handle) {
            tracing::error!(error = ?err, "failed to open config folder");
        }
        return;
    }

    'open_link: {
        let result = match event.id().0.as_str() {
            "help/documentation" => open::that("https://docs.gitbutler.com"),
            "help/debugging-guide" => {
                open::that("https://docs.gitbutler.com/development/debugging")
            }
            "help/github" => open::that("https://github.com/gitbutlerapp/gitbutler"),
            "help/release-notes" => {
                open::that("https://github.com/gitbutlerapp/gitbutler/releases")
            }
            "help/report-issue" => {
                open::that("https://github.com/gitbutlerapp/gitbutler/issues/new/choose")
            }
            "help/discord" => open::that("https://discord.com/invite/MmFkmaJ42D"),
            "help/youtube" => open::that("https://www.youtube.com/@gitbutlerapp"),
            "help/bluesky" => open::that("https://bsky.app/profile/gitbutler.com"),
            "help/x" => open::that("https://x.com/gitbutler"),
            _ => break 'open_link,
        };

        if let Err(err) = result {
            tracing::error!(error = ?err, "failed to open url for {}", event.id().0);
        }

        return;
    }

    tracing::error!("unhandled 'help' menu event: {}", event.id().0);
}

fn emit<R: Runtime>(window: &WebviewWindow<R>, event: &str, shortcut: &str) {
    if let Err(err) = window.emit(event, shortcut) {
        tracing::error!(error = ?err, "failed to emit event");
    }
}
