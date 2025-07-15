use async_trait::async_trait;
use pumpkin::command::{
    args::{Arg, ConsumedArgs, simple::SimpleArgConsumer},
    dispatcher::CommandError,
    dispatcher::CommandError::InvalidRequirement,
    tree::CommandTree,
    tree::builder::{argument, require},
    CommandExecutor, CommandSender,
};
use pumpkin_util::text::color::NamedColor;
use pumpkin_util::permission::PermissionLvl;
use crate::{storage::PLAYER_NAME_COLORS, config::ChatColorStyle};

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

// Fonction pour obtenir le style depuis la configuration (pour l'instant, valeurs par défaut)
async fn get_style_from_config(color_name: &str) -> Option<ChatColorStyle> {
    let color_name = color_name.to_lowercase();
    
    // Pour l'instant, utiliser des valeurs par défaut
    match color_name.as_str() {
        "red" => Some(ChatColorStyle::Simple(NamedColor::Red)),
        "blue" => Some(ChatColorStyle::Simple(NamedColor::Blue)),
        "green" => Some(ChatColorStyle::Simple(NamedColor::Green)),
        "yellow" => Some(ChatColorStyle::Simple(NamedColor::Yellow)),
        "purple" => Some(ChatColorStyle::Simple(NamedColor::LightPurple)),
        "aqua" => Some(ChatColorStyle::Simple(NamedColor::Aqua)),
        "white" => Some(ChatColorStyle::Simple(NamedColor::White)),
        "gray" => Some(ChatColorStyle::Simple(NamedColor::Gray)),
        _ => None,
    }
}

// Fonction pour obtenir la liste des couleurs disponibles (pour l'instant, valeurs par défaut)
async fn get_available_colors() -> Vec<String> {
    vec!["red", "blue", "green", "yellow", "purple", "aqua", "white", "gray", "rainbow", "fire"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

const NAMES: [&str; 1] = ["namecolor"];
const DESCRIPTION: &str = "Set your name color or gradient (ex: red, blue, rainbow, fire).";
const ARG_COLOR: &str = "color";

pub struct NameColorExecutor;

#[async_trait]
impl CommandExecutor for NameColorExecutor {
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
                let mut map = PLAYER_NAME_COLORS.lock().await;
                map.insert(p.gameprofile.id, style.clone());
            }
            let feedback = match &style {
                ChatColorStyle::Simple(color) => format!("Your name color is now set to {:?}", color),
                ChatColorStyle::Rainbow => "Your name color is now set to RAINBOW! 🌈".to_string(),
                ChatColorStyle::Fire => "Your name color is now set to FIRE! 🔥".to_string(),
                ChatColorStyle::CustomGradient(gradient_name) => format!("Your name color is now set to {} gradient!", gradient_name),
            };
            p.send_system_message(&pumpkin_util::text::TextComponent::text(feedback)).await;
            
            // TODO: Sauvegarder les données après le changement
            // Note: La sauvegarde se fait maintenant dans on_unload
            // Pour une sauvegarde immédiate, il faudrait passer le DataManager en paramètre
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
            .execute(NameColorExecutor)
            .then(argument(ARG_COLOR, SimpleArgConsumer).execute(NameColorExecutor))
    )
} 