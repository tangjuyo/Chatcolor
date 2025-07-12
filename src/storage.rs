use crate::config::{ChatColorStyle, PluginConfig, Settings};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

// Stockage global de la couleur/style par défaut de chaque joueur
pub static PLAYER_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Stockage global de la couleur/style du pseudo de chaque joueur
pub static PLAYER_NAME_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Configuration du plugin
pub static PLUGIN_CONFIG: Lazy<Mutex<PluginConfig>> = Lazy::new(|| Mutex::new(PluginConfig {
    save_interval: 300, // 5 minutes par défaut
    auto_save: true,
    simple_colors: HashMap::new(),
    gradients: HashMap::new(),
    settings: Settings {
        default_chat_color: "white".to_string(),
        default_name_color: "white".to_string(),
    },
})); 