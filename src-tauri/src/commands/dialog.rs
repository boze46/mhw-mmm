use std::sync::mpsc;
use tauri::AppHandle;
use tauri_plugin_dialog::{DialogExt, FilePath};

/// 选择游戏目录
#[tauri::command]
pub fn select_game_directory(app: AppHandle) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_title("选择 Monster Hunter World 游戏目录")
        .pick_folder(move |result| {
            let _ = tx.send(result);
        });

    match rx.recv() {
        Ok(Some(FilePath::Path(path))) => Ok(path.to_string_lossy().to_string()),
        Ok(Some(FilePath::Url(url))) => Ok(url.to_string()),
        Ok(None) => Err("未选择目录".to_string()),
        Err(_) => Err("对话框通信失败".to_string()),
    }
}

/// 选择压缩包文件
#[tauri::command]
pub fn select_archive_file(app: AppHandle) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .set_title("选择 MOD 压缩包")
        .add_filter("压缩包", &["zip", "rar", "7z", "tar.gz"])
        .pick_file(move |result| {
            let _ = tx.send(result);
        });

    match rx.recv() {
        Ok(Some(FilePath::Path(path))) => Ok(path.to_string_lossy().to_string()),
        Ok(Some(FilePath::Url(url))) => Ok(url.to_string()),
        Ok(None) => Err("未选择文件".to_string()),
        Err(_) => Err("对话框通信失败".to_string()),
    }
}
