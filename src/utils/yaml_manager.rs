use crate::config::{PluginConfig, PluginData, PlayerData};
use crate::storage::{PLAYER_COLORS, PLAYER_NAME_COLORS, PLUGIN_CONFIG};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new("plugins/ChatColor/config.yml");
    
    log::info!("[ChatColor] Looking for config at: {:?}", config_path);
    log::info!("[ChatColor] Config file exists: {}", config_path.exists());
    
    if !config_path.exists() {
        create_default_config(config_path)?;
    } else {
        load_existing_config(config_path).await?;
    }
    
    log::info!("[ChatColor] Config loaded successfully");
    Ok(())
}

fn create_default_config(config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut default_simple_colors = HashMap::new();
    let mut default_gradients = HashMap::new();
    
    // Couleurs simples
    let simple_colors = [
        ("red", "§c"), ("dark_red", "§4"), ("blue", "§9"), ("dark_blue", "§1"),
        ("green", "§a"), ("dark_green", "§2"), ("yellow", "§e"), ("gold", "§6"),
        ("purple", "§5"), ("light_purple", "§d"), ("aqua", "§b"), ("dark_aqua", "§3"),
        ("white", "§f"), ("gray", "§7"), ("dark_gray", "§8"), ("black", "§0"),
    ];
    
    for (name, code) in simple_colors {
        default_simple_colors.insert(name.to_string(), code.to_string());
    }
    
    // Gradients par défaut
    default_gradients.insert("rainbow".to_string(), crate::config::GradientConfig {
        method: crate::config::GradientMethod::Hsv,
        start_hue: Some(0.0),
        end_hue: Some(360.0),
        saturation: Some(1.0),
        value: Some(1.0),
        colors: None,
    });
    
    default_gradients.insert("fire".to_string(), crate::config::GradientConfig {
        method: crate::config::GradientMethod::RgbInterpolation,
        start_hue: None,
        end_hue: None,
        saturation: None,
        value: None,
        colors: Some(vec![[255, 0, 0], [255, 165, 0], [255, 255, 0]]),
    });
    
    default_gradients.insert("ocean".to_string(), crate::config::GradientConfig {
        method: crate::config::GradientMethod::RgbInterpolation,
        start_hue: None,
        end_hue: None,
        saturation: None,
        value: None,
        colors: Some(vec![[0, 119, 190], [0, 191, 255], [135, 206, 235]]),
    });
    
    default_gradients.insert("sunset".to_string(), crate::config::GradientConfig {
        method: crate::config::GradientMethod::RgbInterpolation,
        start_hue: None,
        end_hue: None,
        saturation: None,
        value: None,
        colors: Some(vec![[255, 69, 0], [255, 140, 0], [255, 215, 0], [255, 20, 147]]),
    });
    
    default_gradients.insert("forest".to_string(), crate::config::GradientConfig {
        method: crate::config::GradientMethod::RgbInterpolation,
        start_hue: None,
        end_hue: None,
        saturation: None,
        value: None,
        colors: Some(vec![[34, 139, 34], [50, 205, 50], [144, 238, 144]]),
    });
    
    let default_config = PluginConfig {
        save_interval: 300,
        auto_save: true,
        simple_colors: default_simple_colors,
        gradients: default_gradients,
        settings: crate::config::Settings {
            default_chat_color: "white".to_string(),
            default_name_color: "white".to_string(),
        },
    };
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let yaml_content = serde_yaml::to_string(&default_config)?;
    fs::write(config_path, yaml_content)?;
    log::info!("[ChatColor] Default config created");
    
    Ok(())
}

async fn load_existing_config(config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let yaml_content = match fs::read_to_string(config_path) {
        Ok(content) => {
            log::info!("[ChatColor] YAML file read successfully");
            content
        },
        Err(e) => {
            log::error!("[ChatColor] Failed to read config file: {}", e);
            return Err(Box::new(e));
        }
    };
    
    log::info!("[ChatColor] YAML content length: {} characters", yaml_content.len());
    
    let config: PluginConfig = match serde_yaml::from_str(&yaml_content) {
        Ok(config) => {
            log::info!("[ChatColor] YAML parsed successfully");
            config
        },
        Err(e) => {
            log::error!("[ChatColor] Failed to parse YAML: {}", e);
            return Err(Box::new(e));
        }
    };
    
    // Log pour debug avant le move
    log::info!("[ChatColor] Config loaded - Simple colors: {}, Gradients: {}", 
               config.simple_colors.len(), config.gradients.len());
    
    // Log des couleurs simples
    for (name, code) in &config.simple_colors {
        log::info!("[ChatColor] Simple color: {} = {}", name, code);
    }
    
    // Log des gradients
    for (name, gradient) in &config.gradients {
        log::info!("[ChatColor] Gradient: {} = {:?}", name, gradient);
    }
    
    let mut plugin_config = PLUGIN_CONFIG.lock().await;
    *plugin_config = config;
    
    Ok(())
}

pub async fn save_data() -> Result<(), Box<dyn std::error::Error>> {
    let data_path = Path::new("plugins/ChatColor/data.yml");
    
    // Créer le dossier si il n'existe pas
    if let Some(parent) = data_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut plugin_data = PluginData {
        players: HashMap::new(),
    };
    
    // Récupérer les données des joueurs
    {
        let chat_colors = PLAYER_COLORS.lock().await;
        let name_colors = PLAYER_NAME_COLORS.lock().await;
        
        for (uuid, chat_color) in chat_colors.iter() {
            let player_data = plugin_data.players.entry(uuid.to_string()).or_insert_with(|| PlayerData {
                uuid: uuid.to_string(),
                chat_color: None,
                name_color: None,
            });
            player_data.chat_color = Some(chat_color.clone());
        }
        
        for (uuid, name_color) in name_colors.iter() {
            let player_data = plugin_data.players.entry(uuid.to_string()).or_insert_with(|| PlayerData {
                uuid: uuid.to_string(),
                chat_color: None,
                name_color: None,
            });
            player_data.name_color = Some(name_color.clone());
        }
    }
    
    // Sauvegarder en YAML
    let yaml_content = serde_yaml::to_string(&plugin_data)?;
    fs::write(data_path, yaml_content)?;
    
    log::info!("[ChatColor] Data saved successfully");
    Ok(())
}

pub async fn load_data() -> Result<(), Box<dyn std::error::Error>> {
    let data_path = Path::new("plugins/ChatColor/data.yml");
    
    if !data_path.exists() {
        log::info!("[ChatColor] No data file found, starting with empty data");
        return Ok(());
    }
    
    let yaml_content = fs::read_to_string(data_path)?;
    let plugin_data: PluginData = serde_yaml::from_str(&yaml_content)?;
    
    // Charger les données des joueurs
    {
        let mut chat_colors = PLAYER_COLORS.lock().await;
        let mut name_colors = PLAYER_NAME_COLORS.lock().await;
        
        for (uuid_str, player_data) in plugin_data.players {
            if let Ok(uuid) = Uuid::parse_str(&uuid_str) {
                if let Some(chat_color) = player_data.chat_color {
                    chat_colors.insert(uuid, chat_color);
                }
                if let Some(name_color) = player_data.name_color {
                    name_colors.insert(uuid, name_color);
                }
            }
        }
    }
    
    log::info!("[ChatColor] Data loaded successfully");
    Ok(())
} 