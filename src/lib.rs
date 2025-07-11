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

mod color_parser;
mod commands;

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");

// Stockage global de la couleur par défaut de chaque joueur
static PLAYER_COLORS: Lazy<Mutex<HashMap<Uuid, NamedColor>>> = Lazy::new(|| Mutex::new(HashMap::new()));

async fn register_permissions(context: &Context) -> Result<(), String> {
    let permission = Permission::new(
        &format!("{PLUGIN_NAME}:command.chatcolor"),
        "Use the /chatcolor command",
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

        // Si le message ne contient pas de code couleur, appliquer la couleur par défaut
        let formatted_message = if event.message.contains('&') {
            color_parser::parse_color_codes(&event.message)
        } else {
            // Chercher la couleur du joueur
            let color = {
                let map = PLAYER_COLORS.lock().await;
                map.get(&event.player.gameprofile.id).copied().unwrap_or(NamedColor::White)
            };
            color_parser::parse_color_codes(&format!("&{}{}",
                color_to_code(color), event.message))
        };
        
        log::info!("Original message: '{}'", event.message);
        log::info!("Formatted message text: '{}'", formatted_message.clone().get_text());
        
        // Créer le message décoré en gardant la couleur
        let decorated_message = formatted_message.clone();
        
        log::info!("Decorated message (structure): {:?}", decorated_message);

        // Cancel the original event
        event.set_cancelled(true);

        // Broadcast the formatted message à tous les joueurs du monde
        let world = event.player.living_entity.entity.world.read().await;
        world
            .broadcast_message(
                &decorated_message,
                &pumpkin_util::text::TextComponent::text(event.player.gameprofile.name.clone()),
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