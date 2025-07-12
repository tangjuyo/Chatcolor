use std::sync::Arc;
use pumpkin::plugin::{
    Context, EventPriority
};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::permission::{Permission, PermissionDefault, PermissionLvl};


// Modules
pub mod config;
pub mod storage;
pub mod utils;
pub mod commands;
pub mod handlers;

// Imports
use crate::utils::*;
use crate::commands::*;
use crate::handlers::*;

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

    // Charger la configuration
    if let Err(e) = load_config().await {
        log::error!("[ChatColor] Failed to load config: {}", e);
    }

    // Charger les données des joueurs
    if let Err(e) = load_data().await {
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
    if let Err(e) = save_data().await {
        log::error!("[ChatColor] Failed to save data: {}", e);
    }
    
    log::info!("ChatColor Plugin has been unloaded.");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
} 