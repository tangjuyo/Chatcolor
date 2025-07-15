use std::sync::Arc;
use pumpkin::plugin::{
    player::player_chat::PlayerChatEvent,
    EventHandler, Cancellable
};
use crate::{storage::{PLAYER_COLORS, PLAYER_NAME_COLORS}, config::ChatColorStyle};
use crate::utils::{apply_rainbow_gradient, apply_fire_gradient, parse_color_codes, apply_custom_gradient, color_to_code};

pub struct ChatEventHandler;

#[async_trait::async_trait]
impl EventHandler<PlayerChatEvent> for ChatEventHandler {
    async fn handle_blocking(
        &self,
        _server: &Arc<pumpkin::server::Server>,
        event: &mut PlayerChatEvent,
    ) {
        log::info!("ChatColor: handle_blocking called for message: '{}'", event.message);

        // Récupère la couleur du joueur (pas de fallback)
        let player_style = {
            let map = PLAYER_COLORS.lock().await;
            map.get(&event.player.gameprofile.id).cloned()
        };
        let player_name_style = {
            let map = PLAYER_NAME_COLORS.lock().await;
            map.get(&event.player.gameprofile.id).cloned()
        };

        // Si aucune couleur n'est définie, ne rien faire (laisser le chat vanilla)
        if player_style.is_none() && player_name_style.is_none() {
            return;
        }

        // LOG: Afficher le style utilisé pour ce joueur
        log::info!("[ChatColor] Style pour {}: {:?}", event.player.gameprofile.name, player_style);
        // Formate le message
        let formatted_message = match player_style.as_ref() {
            Some(style) => match style {
                ChatColorStyle::Simple(color) => {
                    if event.message.contains('&') {
                        parse_color_codes(&event.message)
                    } else {
                        parse_color_codes(&format!("&{}{}", color_to_code(*color), event.message))
                    }
                },
                ChatColorStyle::Rainbow => apply_rainbow_gradient(&event.message),
                ChatColorStyle::Fire => apply_fire_gradient(&event.message),
                ChatColorStyle::CustomGradient(gradient_name) => apply_custom_gradient(&event.message, gradient_name).await,
            },
            None => parse_color_codes(&event.message),
        };

        // LOG: Afficher le style utilisé pour le pseudo
        log::info!("[ChatColor] Style PSEUDO pour {}: {:?}", event.player.gameprofile.name, player_name_style);
        // Formate le pseudo
        let formatted_name = match player_name_style.as_ref() {
            Some(style) => match style {
                ChatColorStyle::Simple(color) => {
                    parse_color_codes(&format!("&{}{}", color_to_code(*color), event.player.gameprofile.name))
                },
                ChatColorStyle::Rainbow => apply_rainbow_gradient(&event.player.gameprofile.name),
                ChatColorStyle::Fire => apply_fire_gradient(&event.player.gameprofile.name),
                ChatColorStyle::CustomGradient(gradient_name) => apply_custom_gradient(&event.player.gameprofile.name, gradient_name).await,
            },
            None => parse_color_codes(&event.player.gameprofile.name),
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