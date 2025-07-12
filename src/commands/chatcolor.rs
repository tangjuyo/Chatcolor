use async_trait::async_trait;
use pumpkin::command::{
    args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
    dispatcher::CommandError,
    dispatcher::CommandError::InvalidRequirement,
    tree::CommandTree,
    tree::builder::{argument, require},
    CommandExecutor, CommandSender,
};
use pumpkin_util::permission::PermissionLvl;
use pumpkin_util::text::color::NamedColor;
use crate::{storage::{PLAYER_COLORS, PLUGIN_CONFIG}, config::ChatColorStyle};
use crate::utils::save_data;

// Fonction pour parser un code couleur Minecraft en NamedColor
fn parse_color_code_section(code: &str) -> Option<NamedColor> {
    if code.chars().count() != 2 || !code.starts_with('§') {
        return None;
    }
    
    match code.chars().nth(1)? {
        '0' => Some(NamedColor::Black),
        '1' => Some(NamedColor::DarkBlue),
        '2' => Some(NamedColor::DarkGreen),
        '3' => Some(NamedColor::DarkAqua),
        '4' => Some(NamedColor::DarkRed),
        '5' => Some(NamedColor::DarkPurple),
        '6' => Some(NamedColor::Gold),
        '7' => Some(NamedColor::Gray),
        '8' => Some(NamedColor::DarkGray),
        '9' => Some(NamedColor::Blue),
        'a' => Some(NamedColor::Green),
        'b' => Some(NamedColor::Aqua),
        'c' => Some(NamedColor::Red),
        'd' => Some(NamedColor::LightPurple),
        'e' => Some(NamedColor::Yellow),
        'f' => Some(NamedColor::White),
        _ => None,
    }
}

// Fonction pour obtenir le style depuis la configuration
pub async fn get_style_from_config(color_name: &str) -> Option<ChatColorStyle> {
    let config = PLUGIN_CONFIG.lock().await;
    let color_name = color_name.to_lowercase();
    
    // Vérifier d'abord les couleurs simples
    if let Some(color_code) = config.simple_colors.get(&color_name) {
        if let Some(named_color) = parse_color_code_section(color_code) {
            return Some(ChatColorStyle::Simple(named_color));
        }
    }
    
    // Vérifier ensuite les gradients
    if config.gradients.contains_key(&color_name) {
        return Some(ChatColorStyle::CustomGradient(color_name));
    }
    
    None
}

// Fonction pour obtenir la liste des couleurs disponibles
async fn get_available_colors() -> Vec<String> {
    let config = PLUGIN_CONFIG.lock().await;
    let mut colors = Vec::new();
    
    // Ajouter les couleurs simples
    colors.extend(config.simple_colors.keys().cloned());
    
    // Ajouter les gradients
    colors.extend(config.gradients.keys().cloned());
    
    colors
}

const NAMES: [&str; 1] = ["chatcolor"];
const DESCRIPTION: &str = "Set your default chat color or gradient (ex: red, blue, rainbow, fire).";
const ARG_COLOR: &str = "color";

pub struct ChatColorExecutor;

#[async_trait]
impl CommandExecutor for ChatColorExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _server: &pumpkin::server::Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(p) = sender.as_player() else {
            return Err(InvalidRequirement);
        };
        
        // Seuls les OP peuvent utiliser la commande
        if sender.permission_lvl() < PermissionLvl::One {
            p.send_system_message(&pumpkin_util::text::TextComponent::text(
                "You must be OP to use this command."
            )).await;
            return Ok(());
        }

        let color_str = match args.get(ARG_COLOR) {
            Some(Arg::Simple(s)) => *s,
            _ => "",
        };
        
        // Essayer d'abord les styles spéciaux (pour la compatibilité)
        let style = if color_str.to_lowercase() == "rainbow" {
            Some(ChatColorStyle::Rainbow)
        } else if color_str.to_lowercase() == "fire" {
            Some(ChatColorStyle::Fire)
        } else {
            // Sinon, chercher dans la configuration
            get_style_from_config(&color_str.to_lowercase()).await
        };
        
        if let Some(style) = style {
            {
                let mut map = PLAYER_COLORS.lock().await;
                map.insert(p.gameprofile.id, style.clone());
            }
            let feedback = match &style {
                ChatColorStyle::Simple(color) => format!("Your chat color is now set to {:?}", color),
                ChatColorStyle::Rainbow => "Your chat color is now set to RAINBOW! 🌈".to_string(),
                ChatColorStyle::Fire => "Your chat color is now set to FIRE! 🔥".to_string(),
                ChatColorStyle::CustomGradient(gradient_name) => format!("Your chat color is now set to {} gradient!", gradient_name),
            };
            p.send_system_message(&pumpkin_util::text::TextComponent::text(feedback)).await;
            
            // Sauvegarder les données après le changement
            if let Err(e) = save_data().await {
                log::error!("[ChatColor] Failed to save player data: {}", e);
            }
        } else {
            // Afficher la liste des couleurs disponibles
            let available_colors = get_available_colors().await;
            let color_list = available_colors.join(", ");
            p.send_system_message(&pumpkin_util::text::TextComponent::text(
                format!("Unknown color or style. Available: {}", color_list)
            )).await;
        }
        
        Ok(())
    }
}

#[allow(clippy::redundant_closure_for_method_calls)]
pub fn init_command_tree() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        require(|sender| sender.is_player())
            .execute(ChatColorExecutor)
            .then(argument(ARG_COLOR, SimpleArgConsumer).execute(ChatColorExecutor))
    )
} 