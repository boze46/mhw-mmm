use serde::{Deserialize, Serialize};

/// 分类定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub color: String,
}

/// MOD 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModFiles {
    pub nativepc: Vec<String>,
    pub root: Vec<String>,
}

/// MOD 元数据（存储在 mod-info.json 中）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nexus_id: Option<String>,
    pub categories: Vec<String>,
    pub enabled: bool,
    pub install_date: String,
    pub file_size: u64,
    pub files: ModFiles,
}

/// MOD 配置项（存储在 config.json 的 mods 数组中）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModConfigItem {
    pub name: String,
    pub order: usize,
    pub enabled: bool,
}

/// 全局配置（config.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub version: String,
    pub game_directory: String,
    pub data_directory: String,
    pub mods: Vec<ModConfigItem>,
    pub categories: Vec<Category>,
    pub settings: AppSettings,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub auto_detect_conflicts: bool,
    pub show_conflict_warnings: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            game_directory: String::new(),
            data_directory: "./data".to_string(),
            mods: Vec::new(),
            categories: vec![
                Category {
                    name: "武器".to_string(),
                    color: "#FF5733".to_string(),
                },
                Category {
                    name: "装备".to_string(),
                    color: "#33FF57".to_string(),
                },
                Category {
                    name: "美化".to_string(),
                    color: "#3357FF".to_string(),
                },
            ],
            settings: AppSettings {
                auto_detect_conflicts: true,
                show_conflict_warnings: true,
            },
        }
    }
}

/// 压缩包文件树节点
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveFileNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ArchiveFileNode>>,
}

/// 压缩包内容预览
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePreview {
    pub has_native_pc: bool,
    pub native_pc_path: String,
    pub files: Vec<ArchiveFileNode>,
}

/// 文件操作进度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileProgress {
    pub current: usize,
    pub total: usize,
    pub percentage: f32,
    pub current_file: Option<String>,
}

/// 操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl OperationResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            error: None,
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self {
            success: false,
            message: None,
            error: Some(error.into()),
        }
    }
}
