use std::sync::Arc;
use pumpkin::plugin::{
    player::player_chat::PlayerChatEvent,
    Context, EventHandler, EventPriority, Cancellable
};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::{
    permission::{Permission, PermissionDefault},
    text::TextComponent,
};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use uuid::Uuid;
use pumpkin_util::text::color::NamedColor;
use crate::color_parser::{apply_rainbow_gradient, apply_fire_gradient, parse_color_codes};

#[derive(Clone, Copy, Debug)]
pub enum ChatColorStyle {
    Simple(NamedColor),
    Rainbow,
    Fire,
}

mod color_parser;
mod commands;

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");

// Stockage global de la couleur/style par défaut de chaque joueur
static PLAYER_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Stockage global de la couleur/style du pseudo de chaque joueur
static PLAYER_NAME_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

async fn register_permissions(context: &Context) -> Result<(), String> {
    let permission = Permission::new(
        &format!("{PLUGIN_NAME}:command.chatcolor"),
        "Use the /chatcolor command",
        PermissionDefault::Op(pumpkin_util::PermissionLvl::Zero),
    );
    context.register_permission(permission).await?;
    let permission = Permission::new(
        &format!("{PLUGIN_NAME}:command.namecolor"),
        "Use the /namecolor command",
        PermissionDefault::Op(pumpkin_util::PermissionLvl::Zero),
    );
    context.register_permission(permission).await?;
    Ok(())
}

async fn register_events(context: &Context) {
    // Register the chat event handler
    context.register_event::<PlayerChatEvent, ChatEventHandler>(
        Arc::new(ChatEventHandler),
        EventPriority::Normal,
        true, // Blocking handler
    ).await;

    // Enregistrer la commande /chatcolor
    context.register_command(
        commands::init_command_tree(),
        &format!("{PLUGIN_NAME}:command.chatcolor"),
    ).await;
    // Enregistrer la commande /namecolor
    context.register_command(
        commands::init_namecolor_command_tree(),
        &format!("{PLUGIN_NAME}:command.namecolor"),
    ).await;
}

#[plugin_method]
async fn on_load(&mut self, context: &Context) -> Result<(), String> {
    pumpkin::init_log!();

    register_permissions(context).await?;
    register_events(context).await;

    log::info!("ChatColor Plugin has been loaded.");
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

struct ChatEventHandler;

#[async_trait::async_trait]
impl EventHandler<PlayerChatEvent> for ChatEventHandler {
    async fn handle_blocking(
        &self,
        server: &Arc<pumpkin::server::Server>,
        event: &mut PlayerChatEvent,
    ) {
        log::info!("ChatColor: handle_blocking called for message: '{}'", event.message);

        // Récupère le style du message
        let style = {
            let map = PLAYER_COLORS.lock().await;
            map.get(&event.player.gameprofile.id).copied().unwrap_or(ChatColorStyle::Simple(NamedColor::White))
        };
        // Récupère le style du pseudo
        let name_style = {
            let map = PLAYER_NAME_COLORS.lock().await;
            map.get(&event.player.gameprofile.id).copied().unwrap_or(ChatColorStyle::Simple(NamedColor::White))
        };

        // Formate le message
        let formatted_message = match style {
            ChatColorStyle::Simple(color) => {
                if event.message.contains('&') {
                    parse_color_codes(&event.message)
                } else {
                    parse_color_codes(&format!("&{}{}", color_to_code(color), event.message))
                }
            },
            ChatColorStyle::Rainbow => apply_rainbow_gradient(&event.message),
            ChatColorStyle::Fire => apply_fire_gradient(&event.message),
        };

        // Formate le pseudo
        let formatted_name = match name_style {
            ChatColorStyle::Simple(color) => {
                parse_color_codes(&format!("&{}{}", color_to_code(color), event.player.gameprofile.name))
            },
            ChatColorStyle::Rainbow => apply_rainbow_gradient(&event.player.gameprofile.name),
            ChatColorStyle::Fire => apply_fire_gradient(&event.player.gameprofile.name),
        };

        // Cancel the original event
        event.set_cancelled(true);

        // Broadcast the formatted message à tous les joueurs du monde
        let world = event.player.living_entity.entity.world.read().await;
        world
            .broadcast_message(
                &formatted_message,
                &formatted_name,
                0, // Chat type
                None,
            )
            .await;

        // Log le message final envoyé
        log::info!(
            "<chat> {}: {}",
            event.player.gameprofile.name,
            formatted_message.clone().get_text()
        );
    }
}

// Convertit NamedColor en code couleur Minecraft (ex: &a)
fn color_to_code(color: NamedColor) -> char {
    match color {
        NamedColor::Black => '0',
        NamedColor::DarkBlue => '1',
        NamedColor::DarkGreen => '2',
        NamedColor::DarkAqua => '3',
        NamedColor::DarkRed => '4',
        NamedColor::DarkPurple => '5',
        NamedColor::Gold => '6',
        NamedColor::Gray => '7',
        NamedColor::DarkGray => '8',
        NamedColor::Blue => '9',
        NamedColor::Green => 'a',
        NamedColor::Aqua => 'b',
        NamedColor::Red => 'c',
        NamedColor::LightPurple => 'd',
        NamedColor::Yellow => 'e',
        NamedColor::White => 'f',
    }
} 