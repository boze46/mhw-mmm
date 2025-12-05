use crate::models::{AppConfig, ModInfo, OperationResult};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// 获取数据目录路径
pub fn get_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    let data_dir = app_data_dir.join("data");

    // 确保数据目录存在
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("无法创建数据目录: {}", e))?;
    }

    Ok(data_dir)
}

/// 获取配置文件路径
pub fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let data_dir = get_data_dir(app)?;
    Ok(data_dir.join("config.json"))
}

/// 加载配置文件
#[tauri::command]
pub fn load_config(app: AppHandle) -> Result<AppConfig, String> {
    let config_path = get_config_path(&app)?;

    if !config_path.exists() {
        // 如果配置文件不存在，创建默认配置
        let default_config = AppConfig::default();
        save_config(app.clone(), default_config.clone())?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("无法读取配置文件: {}", e))?;

    let config: AppConfig = serde_json::from_str(&content)
        .map_err(|e| format!("无法解析配置文件: {}", e))?;

    Ok(config)
}

/// 保存配置文件
#[tauri::command]
pub fn save_config(app: AppHandle, config: AppConfig) -> Result<OperationResult, String> {
    let config_path = get_config_path(&app)?;

    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("无法序列化配置: {}", e))?;

    fs::write(&config_path, content)
        .map_err(|e| format!("无法写入配置文件: {}", e))?;

    Ok(OperationResult::success("配置已保存"))
}

/// 加载单个 MOD 的信息
#[tauri::command]
pub fn load_mod_info(app: AppHandle, mod_name: String) -> Result<ModInfo, String> {
    let data_dir = get_data_dir(&app)?;
    let mod_dir = data_dir.join(&mod_name);
    let mod_info_path = mod_dir.join("mod-info.json");

    if !mod_info_path.exists() {
        return Err(format!("MOD 信息文件不存在: {}", mod_name));
    }

    let content = fs::read_to_string(&mod_info_path)
        .map_err(|e| format!("无法读取 MOD 信息: {}", e))?;

    let mod_info: ModInfo = serde_json::from_str(&content)
        .map_err(|e| format!("无法解析 MOD 信息: {}", e))?;

    Ok(mod_info)
}

/// 保存单个 MOD 的信息
#[tauri::command]
pub fn save_mod_info(app: AppHandle, mod_name: String, mod_info: ModInfo) -> Result<OperationResult, String> {
    let data_dir = get_data_dir(&app)?;
    let mod_dir = data_dir.join(&mod_name);

    // 确保 MOD 目录存在
    if !mod_dir.exists() {
        fs::create_dir_all(&mod_dir)
            .map_err(|e| format!("无法创建 MOD 目录: {}", e))?;
    }

    let mod_info_path = mod_dir.join("mod-info.json");
    let content = serde_json::to_string_pretty(&mod_info)
        .map_err(|e| format!("无法序列化 MOD 信息: {}", e))?;

    fs::write(&mod_info_path, content)
        .map_err(|e| format!("无法写入 MOD 信息: {}", e))?;

    Ok(OperationResult::success("MOD 信息已保存"))
}

/// 获取所有 MOD 的信息
#[tauri::command]
pub fn load_all_mods(app: AppHandle) -> Result<Vec<ModInfo>, String> {
    let config = load_config(app.clone())?;
    let mut mods = Vec::new();

    for mod_item in config.mods {
        match load_mod_info(app.clone(), mod_item.name.clone()) {
            Ok(mod_info) => mods.push(mod_info),
            Err(e) => {
                eprintln!("加载 MOD {} 失败: {}", mod_item.name, e);
                // 继续加载其他 MOD
            }
        }
    }

    Ok(mods)
}
