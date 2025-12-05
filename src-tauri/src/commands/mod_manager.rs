use crate::commands::{
    archive::{calculate_dir_size, extract_zip_archive},
    config::{get_data_dir, load_config, save_config},
};
use crate::models::{ModConfigItem, ModFiles, ModInfo, OperationResult};
use std::fs;
use std::path::Path;
use tauri::AppHandle;

/// 收集 MOD 文件列表
fn collect_mod_files(mod_dir: &Path) -> Result<ModFiles, String> {
    let mut nativepc_files = Vec::new();
    let mut root_files = Vec::new();

    // 收集 nativepc 文件夹中的文件
    let nativepc_path = mod_dir.join("nativepc");
    if nativepc_path.exists() {
        collect_files_recursive(&nativepc_path, &nativepc_path, &mut nativepc_files)?;
    }

    // 收集根目录中的其他文件
    for entry in fs::read_dir(mod_dir).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let file_name = entry.file_name();

        // 跳过 nativepc 和 mod-info.json
        if file_name == "nativepc" || file_name == "mod-info.json" {
            continue;
        }

        let path = entry.path();
        let relative_path = path
            .strip_prefix(mod_dir)
            .unwrap()
            .to_string_lossy()
            .to_string();

        root_files.push(relative_path);
    }

    Ok(ModFiles {
        nativepc: nativepc_files,
        root: root_files,
    })
}

/// 递归收集文件路径（相对于基础路径）
fn collect_files_recursive(
    dir: &Path,
    base: &Path,
    files: &mut Vec<String>,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("读取目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_recursive(&path, base, files)?;
        } else {
            let relative_path = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .to_string();
            files.push(relative_path);
        }
    }

    Ok(())
}

/// 完整的 MOD 安装流程
#[tauri::command]
pub fn install_mod(
    app: AppHandle,
    archive_path: String,
    mod_name: String,
    nexus_id: Option<String>,
    categories: Vec<String>,
) -> Result<OperationResult, String> {
    // 1. 验证 MOD 名称是否已存在
    let config = load_config(app.clone())?;
    if config.mods.iter().any(|m| m.name == mod_name) {
        return Err(format!("MOD \"{}\" 已存在", mod_name));
    }

    // 2. 获取数据目录并创建 MOD 目录
    let data_dir = get_data_dir(&app)?;
    let mod_dir = data_dir.join(&mod_name);

    if mod_dir.exists() {
        return Err(format!("MOD 目录已存在: {:?}", mod_dir));
    }

    fs::create_dir_all(&mod_dir).map_err(|e| format!("创建 MOD 目录失败: {}", e))?;

    // 3. 解压压缩包到 MOD 目录
    extract_zip_archive(archive_path, mod_dir.to_string_lossy().to_string())?;

    // 4. 收集 MOD 文件列表
    let mod_files = collect_mod_files(&mod_dir)?;

    // 5. 计算 MOD 大小
    let file_size = calculate_dir_size(&mod_dir).unwrap_or(0);

    // 6. 创建 mod-info.json
    let mod_info = ModInfo {
        name: mod_name.clone(),
        nexus_id,
        categories,
        enabled: false,
        install_date: chrono::Utc::now().to_rfc3339(),
        file_size,
        files: mod_files,
    };

    let mod_info_path = mod_dir.join("mod-info.json");
    let mod_info_content =
        serde_json::to_string_pretty(&mod_info).map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&mod_info_path, mod_info_content).map_err(|e| format!("写入文件失败: {}", e))?;

    // 7. 更新 config.json
    let mut new_config = config;
    let order = new_config.mods.len() + 1;
    new_config.mods.push(ModConfigItem {
        name: mod_name.clone(),
        order,
        enabled: false,
    });

    save_config(app, new_config)?;

    Ok(OperationResult::success(format!(
        "MOD \"{}\" 安装成功",
        mod_name
    )))
}

/// 启用 MOD（复制到游戏目录）
#[tauri::command]
pub fn enable_mod(app: AppHandle, mod_name: String) -> Result<OperationResult, String> {
    use crate::commands::config::{get_data_dir, load_config, save_mod_info};
    use crate::commands::file_ops::copy_mod_to_game;

    // 1. 加载配置
    let config = load_config(app.clone())?;

    // 2. 获取 MOD 目录
    let data_dir = get_data_dir(&app)?;
    let mod_dir = data_dir.join(&mod_name);

    if !mod_dir.exists() {
        return Err(format!("MOD 目录不存在: {}", mod_name));
    }

    // 3. 复制文件到游戏目录
    copy_mod_to_game(
        mod_dir.to_string_lossy().to_string(),
        config.game_directory.clone(),
    )?;

    // 4. 更新 mod-info.json
    let mod_info_path = mod_dir.join("mod-info.json");
    let mut mod_info: ModInfo = serde_json::from_str(
        &fs::read_to_string(&mod_info_path).map_err(|e| format!("读取文件失败: {}", e))?,
    )
    .map_err(|e| format!("解析失败: {}", e))?;

    mod_info.enabled = true;
    save_mod_info(app.clone(), mod_name.clone(), mod_info)?;

    // 5. 更新 config.json
    let mut new_config = config;
    if let Some(mod_item) = new_config.mods.iter_mut().find(|m| m.name == mod_name) {
        mod_item.enabled = true;
    }
    save_config(app, new_config)?;

    Ok(OperationResult::success(format!(
        "MOD \"{}\" 已启用",
        mod_name
    )))
}

/// 禁用 MOD（从游戏目录删除）
#[tauri::command]
pub fn disable_mod(app: AppHandle, mod_name: String) -> Result<OperationResult, String> {
    use crate::commands::config::{get_data_dir, load_config, load_mod_info, save_mod_info};
    use crate::commands::file_ops::remove_mod_from_game;

    // 1. 加载配置和 MOD 信息
    let config = load_config(app.clone())?;
    let mut mod_info = load_mod_info(app.clone(), mod_name.clone())?;

    // 2. 从游戏目录删除文件
    remove_mod_from_game(
        config.game_directory.clone(),
        mod_info.files.nativepc.clone(),
        mod_info.files.root.clone(),
    )?;

    // 3. 更新 mod-info.json
    mod_info.enabled = false;
    save_mod_info(app.clone(), mod_name.clone(), mod_info)?;

    // 4. 更新 config.json
    let mut new_config = config;
    if let Some(mod_item) = new_config.mods.iter_mut().find(|m| m.name == mod_name) {
        mod_item.enabled = false;
    }
    save_config(app, new_config)?;

    Ok(OperationResult::success(format!(
        "MOD \"{}\" 已禁用",
        mod_name
    )))
}

/// 删除 MOD（完全删除）
#[tauri::command]
pub fn delete_mod(app: AppHandle, mod_name: String) -> Result<OperationResult, String> {
    use crate::commands::config::{get_data_dir, load_config, load_mod_info};
    use crate::commands::file_ops::{delete_directory, remove_mod_from_game};

    // 1. 加载配置和 MOD 信息
    let config = load_config(app.clone())?;
    let mod_info = load_mod_info(app.clone(), mod_name.clone())?;

    // 2. 如果 MOD 已启用，先从游戏目录删除
    if mod_info.enabled {
        remove_mod_from_game(
            config.game_directory.clone(),
            mod_info.files.nativepc.clone(),
            mod_info.files.root.clone(),
        )?;
    }

    // 3. 删除 MOD 目录
    let data_dir = get_data_dir(&app)?;
    let mod_dir = data_dir.join(&mod_name);
    delete_directory(mod_dir.to_string_lossy().to_string())?;

    // 4. 更新 config.json
    let mut new_config = config;
    new_config.mods.retain(|m| m.name != mod_name);
    save_config(app, new_config)?;

    Ok(OperationResult::success(format!(
        "MOD \"{}\" 已删除",
        mod_name
    )))
}
