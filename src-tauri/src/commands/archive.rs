use crate::models::{ArchiveFileNode, ArchivePreview, OperationResult};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// 检查目录中是否存在 nativepc 文件夹（大小写不敏感）
pub fn find_nativepc_folder(path: &Path) -> Option<PathBuf> {
    if !path.is_dir() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.to_lowercase() == "nativepc" && entry.path().is_dir() {
                    return Some(entry.path());
                }
            }
        }
    }

    None
}

/// 将 nativepc 文件夹统一重命名为小写
pub fn normalize_nativepc_folder(path: &Path) -> Result<(), String> {
    if let Some(nativepc_path) = find_nativepc_folder(path) {
        let file_name = nativepc_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("无法获取文件夹名称")?;

        // 如果已经是小写，则无需操作
        if file_name == "nativepc" {
            return Ok(());
        }

        // 重命名为小写
        let new_path = path.join("nativepc");
        fs::rename(&nativepc_path, &new_path)
            .map_err(|e| format!("无法重命名文件夹: {}", e))?;
    }

    Ok(())
}

/// 预览 zip 压缩包内容
#[tauri::command]
pub fn preview_zip_archive(archive_path: String) -> Result<ArchivePreview, String> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| format!("无法打开压缩包: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("无法读取压缩包: {}", e))?;

    let mut has_native_pc = false;
    let mut native_pc_path = String::new();
    let mut file_tree: Vec<ArchiveFileNode> = Vec::new();

    // 读取压缩包中的所有文件
    for i in 0..archive.len() {
        let file = archive.by_index(i)
            .map_err(|e| format!("无法读取文件: {}", e))?;

        let file_path = file.name().to_string();
        let is_directory = file.is_dir();

        // 检查是否包含 nativepc 文件夹
        if !has_native_pc {
            let path_lower = file_path.to_lowercase();
            if path_lower.contains("nativepc/") || path_lower.contains("nativepc\\") {
                has_native_pc = true;
                // 提取 nativepc 的路径部分
                if let Some(idx) = path_lower.find("nativepc") {
                    native_pc_path = file_path[..idx + 8].to_string();
                }
            }
        }

        // 构建文件树节点
        file_tree.push(ArchiveFileNode {
            name: Path::new(&file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&file_path)
                .to_string(),
            path: file_path,
            is_directory,
            children: None,
        });
    }

    Ok(ArchivePreview {
        has_native_pc,
        native_pc_path,
        files: file_tree,
    })
}

/// 解压 zip 文件到指定目录
#[tauri::command]
pub fn extract_zip_archive(
    archive_path: String,
    destination: String,
) -> Result<OperationResult, String> {
    let file = fs::File::open(&archive_path)
        .map_err(|e| format!("无法打开压缩包: {}", e))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| format!("无法读取压缩包: {}", e))?;

    let dest_path = Path::new(&destination);

    // 确保目标目录存在
    if !dest_path.exists() {
        fs::create_dir_all(dest_path)
            .map_err(|e| format!("无法创建目标目录: {}", e))?;
    }

    // 解压所有文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("无法读取文件 {}: {}", i, e))?;

        let file_path = file.name().to_string(); // 将借用的结果转换为 owned String
        let is_dir = file.is_dir();
        let output_path = dest_path.join(&file_path);

        if is_dir {
            // 创建目录
            fs::create_dir_all(&output_path)
                .map_err(|e| format!("无法创建目录 {}: {}", file_path, e))?;
        } else {
            // 确保父目录存在
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("无法创建父目录: {}", e))?;
            }

            // 解压文件
            let mut output_file = fs::File::create(&output_path)
                .map_err(|e| format!("无法创建文件 {}: {}", file_path, e))?;

            io::copy(&mut file, &mut output_file)
                .map_err(|e| format!("无法写入文件 {}: {}", file_path, e))?;
        }
    }

    // 规范化 nativepc 文件夹名称
    normalize_nativepc_folder(dest_path)?;

    Ok(OperationResult::success("解压成功"))
}

/// 计算目录大小
pub fn calculate_dir_size(path: &Path) -> Result<u64, io::Error> {
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
