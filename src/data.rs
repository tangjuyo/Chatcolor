use crate::config::{PlayerData, PluginData, ChatColorStyle};
use crate::storage::{PLAYER_COLORS, PLAYER_NAME_COLORS};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

pub struct DataManager {
    data_folder: PathBuf,
}

impl DataManager {
    pub fn new(data_folder: PathBuf) -> Self {
        Self { data_folder }
    }

    /// Sauvegarder les données des joueurs
    pub async fn save_data(&self) -> Result<(), String> {
        let data_path = self.data_folder.join("data.yml");
        
        // Créer le dossier si il n'existe pas
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directory: {}", e))?;
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
        let yaml_content = serde_yaml::to_string(&plugin_data)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        fs::write(&data_path, yaml_content)
            .await
            .map_err(|e| format!("Failed to write data file: {}", e))?;
        
        log::info!("[ChatColor] Data saved successfully");
        Ok(())
    }

    /// Charger les données des joueurs
    pub async fn load_data(&self) -> Result<(), String> {
        let data_path = self.data_folder.join("data.yml");
        
        if !data_path.exists() {
            log::info!("[ChatColor] No data file found, starting with empty data");
            return Ok(());
        }
        
        let yaml_content = fs::read_to_string(&data_path)
            .await
            .map_err(|e| format!("Failed to read data file: {}", e))?;
        
        let plugin_data: PluginData = serde_yaml::from_str(&yaml_content)
            .map_err(|e| format!("Failed to parse data file: {}", e))?;
        
        // Charger les données dans les maps globales
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
} 