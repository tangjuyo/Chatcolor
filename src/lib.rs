use std::sync::Arc;
use std::path::PathBuf;
use pumpkin::plugin::{
    Context, EventPriority
};
use pumpkin::plugin::config::{ConfigurablePlugin, PluginConfig};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::permission::{Permission, PermissionDefault, PermissionLvl};


// Modules
pub mod config;
pub mod storage;
pub mod utils;
pub mod commands;
pub mod handlers;
pub mod data;

// Imports
// use crate::utils::*;
use crate::commands::*;
use crate::handlers::*;
use crate::config::ChatColorConfig;

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");


async fn register_permissions(context: &Context) -> Result<(), String> {
    let chatcolor_perm = Permission::new(
        "chat-color:command.chatcolor",
        "Use the /chatcolor command",
        PermissionDefault::Op(PermissionLvl::One),
    );
    context.register_permission(chatcolor_perm).await?;

    let namecolor_perm = Permission::new(
        "chat-color:command.namecolor",
        "Use the /namecolor command",
        PermissionDefault::Op(PermissionLvl::One),
    );
    context.register_permission(namecolor_perm).await?;
    Ok(())
}

async fn register_events(context: &Context) {
    // Register the chat event handler
    context.register_event::<pumpkin::plugin::player::player_chat::PlayerChatEvent, ChatEventHandler>(
        Arc::new(ChatEventHandler),
        EventPriority::Normal,
        true, // Blocking handler
    ).await;

    // Enregistrer la commande /chatcolor
    context.register_command(
        init_chatcolor_command_tree(),
        "chat-color:command.chatcolor",
    ).await;

    // Enregistrer la commande /namecolor
    context.register_command(
        init_namecolor_command_tree(),
        "chat-color:command.namecolor",
    ).await;
}

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    // Example of using the new configuration system
    log::info!("[ChatColor] Loading plugin with new configuration system...");

    // Save default configuration files
    if let Err(e) = self.save_default_config("config.yml").await {
        log::error!("[ChatColor] Failed to save default config: {}", e);
    }
    
    if let Err(e) = self.save_resource("messages.yml", false).await {
        log::error!("[ChatColor] Failed to save messages: {}", e);
    }

    // Load configurations
    match self.get_config().await {
        Ok(pumpkin_config) => {
            match ChatColorConfig::from_pumpkin_config(&serde_yaml::Value::Mapping(pumpkin_config.data.into_iter().map(|(k, v)| (serde_yaml::Value::String(k), v)).collect())) {
                Ok(config) => {
                    self.config = Some(config);
                    log::info!("[ChatColor] Configuration loaded successfully");
                    
                    // Example: Access configuration values
                    if let Some(config) = &self.config {
                        let save_interval = config.save_interval;
                        let auto_save = config.auto_save;
                        let default_chat_color = &config.settings.default_chat_color;
                        
                        log::info!("[ChatColor] Config loaded - Save interval: {}, Auto save: {}, Default chat color: {}", 
                                  save_interval, auto_save, default_chat_color);
                    }
                }
                Err(e) => {
                    log::error!("[ChatColor] Failed to parse config: {}", e);
                }
            }
        }
        Err(e) => {
            log::error!("[ChatColor] Failed to load config: {}", e);
        }
    }
    
    match self.load_config("messages.yml").await {
        Ok(messages) => {
            self.messages = Some(messages);
            log::info!("[ChatColor] Messages loaded successfully");
            
            // Example: Access message values
            if let Some(messages) = &self.messages {
                let prefix = messages.get_string_or("general.prefix", "&8[&6ChatColor&8] &r");
                let reload_msg = messages.get_string_or("general.reload", "&aConfiguration reloaded successfully!");
                
                log::info!("[ChatColor] Messages loaded - Prefix: {}, Reload msg: {}", prefix, reload_msg);
            }
        }
        Err(e) => {
            log::error!("[ChatColor] Failed to load messages: {}", e);
        }
    }

    // Charger les données des joueurs
    if let Err(e) = self.data_manager.load_data().await {
        log::error!("[ChatColor] Failed to load data: {}", e);
    }

    register_permissions(context).await?;
    register_events(context).await;

    log::info!("ChatColor Plugin has been loaded.");
    Ok(())
}

#[plugin_method]
async fn on_unload(&mut self, _context: &Context) -> Result<(), String> {
    // Sauvegarder les données avant de décharger le plugin
    if let Err(e) = self.data_manager.save_data().await {
        log::error!("[ChatColor] Failed to save data: {}", e);
    }
    
    log::info!("ChatColor Plugin has been unloaded.");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {
    config: Option<ChatColorConfig>,
    messages: Option<PluginConfig>,
    data_manager: data::DataManager,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            config: None,
            messages: None,
            data_manager: data::DataManager::new(PathBuf::from("plugins/ChatColor")),
        }
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigurablePlugin for Plugin {
    fn get_data_folder(&self) -> PathBuf {
        PathBuf::from("plugins/ChatColor")
    }
    
    fn get_plugin_name(&self) -> &str {
        "ChatColor"
    }
    
    fn get_embedded_resource(&self, filename: &str) -> Option<Vec<u8>> {
        match filename {
            "config.yml" => Some(include_bytes!("resources/config.yml").to_vec()),
            "messages.yml" => Some(include_bytes!("resources/messages.yml").to_vec()),
            _ => None,
        }
    }
} 