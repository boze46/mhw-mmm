use crate::models::OperationResult;
use std::fs;
use std::path::Path;
use std::io;

/// 递归复制目录
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // 创建目标目录
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // 遍历源目录
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if src_path.is_dir() {
            // 递归复制子目录
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // 复制文件
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 递归删除目录中的特定文件
pub fn remove_mod_files(game_dir: &Path, mod_files: &[String]) -> io::Result<()> {
    for file_path in mod_files {
        let full_path = game_dir.join(file_path);

        if full_path.exists() {
            if full_path.is_dir() {
                fs::remove_dir_all(&full_path)?;
            } else {
                fs::remove_file(&full_path)?;
            }
        }
    }

    Ok(())
}

/// 完全删除目录
#[tauri::command]
pub fn delete_directory(path: String) -> Result<OperationResult, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err("目录不存在".to_string());
    }

    fs::remove_dir_all(dir_path)
        .map_err(|e| format!("删除目录失败: {}", e))?;

    Ok(OperationResult::success("目录已删除"))
}

/// 复制 MOD 文件到游戏目录
#[tauri::command]
pub fn copy_mod_to_game(
    mod_path: String,
    game_path: String,
) -> Result<OperationResult, String> {
    let mod_dir = Path::new(&mod_path);
    let game_dir = Path::new(&game_path);

    if !mod_dir.exists() {
        return Err("MOD 目录不存在".to_string());
    }

    if !game_dir.exists() {
        return Err("游戏目录不存在".to_string());
    }

    // 复制 nativepc 文件夹
    let nativepc_src = mod_dir.join("nativepc");
    if nativepc_src.exists() {
        let nativepc_dst = game_dir.join("nativepc");
        copy_dir_recursive(&nativepc_src, &nativepc_dst)
            .map_err(|e| format!("复制 nativepc 失败: {}", e))?;
    }

    // 复制根目录下的其他文件
    for entry in fs::read_dir(mod_dir).map_err(|e| format!("读取 MOD 目录失败: {}", e))? {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();
        let file_name = entry.file_name();

        // 跳过 nativepc 和 mod-info.json
        if file_name == "nativepc" || file_name == "mod-info.json" {
            continue;
        }

        let dst_path = game_dir.join(&file_name);

        if path.is_dir() {
            copy_dir_recursive(&path, &dst_path)
                .map_err(|e| format!("复制目录 {} 失败: {}", file_name.to_string_lossy(), e))?;
        } else {
            fs::copy(&path, &dst_path)
                .map_err(|e| format!("复制文件 {} 失败: {}", file_name.to_string_lossy(), e))?;
        }
    }

    Ok(OperationResult::success("MOD 文件已复制到游戏目录"))
}

/// 从游戏目录删除 MOD 文件
#[tauri::command]
pub fn remove_mod_from_game(
    game_path: String,
    nativepc_files: Vec<String>,
    root_files: Vec<String>,
) -> Result<OperationResult, String> {
    let game_dir = Path::new(&game_path);

    if !game_dir.exists() {
        return Err("游戏目录不存在".to_string());
    }

    // 删除 nativepc 中的文件
    for file_path in nativepc_files {
        let full_path = game_dir.join("nativepc").join(&file_path);
        if full_path.exists() {
            if full_path.is_dir() {
                fs::remove_dir_all(&full_path)
                    .map_err(|e| format!("删除目录 {} 失败: {}", file_path, e))?;
            } else {
                fs::remove_file(&full_path)
                    .map_err(|e| format!("删除文件 {} 失败: {}", file_path, e))?;
            }
        }
    }

    // 删除根目录中的文件
    for file_path in root_files {
        let full_path = game_dir.join(&file_path);
        if full_path.exists() {
            if full_path.is_dir() {
                fs::remove_dir_all(&full_path)
                    .map_err(|e| format!("删除目录 {} 失败: {}", file_path, e))?;
            } else {
                fs::remove_file(&full_path)
                    .map_err(|e| format!("删除文件 {} 失败: {}", file_path, e))?;
            }
        }
    }

    Ok(OperationResult::success("MOD 文件已从游戏目录删除"))
}

/// 获取目录大小
#[tauri::command]
pub fn get_directory_size(path: String) -> Result<u64, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err("目录不存在".to_string());
    }

    calculate_dir_size(dir_path)
        .map_err(|e| format!("计算目录大小失败: {}", e))
}

fn calculate_dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0u64;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                total_size += calculate_dir_size(&path)?;
            } else {
                total_size += entry.metadata()?.len();
            }
        }
    } else {
        total_size = fs::metadata(path)?.len();
    }

    Ok(total_size)
}
