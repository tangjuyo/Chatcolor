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
use crate::{PLAYER_COLORS, PLAYER_NAME_COLORS, ChatColorStyle};

const NAMES: [&str; 1] = ["chatcolor"];
const DESCRIPTION: &str = "Set your default chat color or gradient (ex: red, blue, rainbow, fire).";
const ARG_COLOR: &str = "color";

struct ChatColorExecutor;

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
        let color_str = match args.get(ARG_COLOR) {
            Some(Arg::Simple(s)) => *s,
            _ => "",
        };
        let style = match color_str.to_lowercase().as_str() {
            "rainbow" => ChatColorStyle::Rainbow,
            "fire" => ChatColorStyle::Fire,
            "black" => ChatColorStyle::Simple(NamedColor::Black),
            "dark_blue" | "darkblue" | "blue1" => ChatColorStyle::Simple(NamedColor::DarkBlue),
            "dark_green" | "darkgreen" | "green1" => ChatColorStyle::Simple(NamedColor::DarkGreen),
            "dark_aqua" | "darkaqua" | "aqua1" => ChatColorStyle::Simple(NamedColor::DarkAqua),
            "dark_red" | "darkred" | "red1" => ChatColorStyle::Simple(NamedColor::DarkRed),
            "dark_purple" | "darkpurple" | "purple1" => ChatColorStyle::Simple(NamedColor::DarkPurple),
            "gold" => ChatColorStyle::Simple(NamedColor::Gold),
            "gray" => ChatColorStyle::Simple(NamedColor::Gray),
            "dark_gray" | "darkgray" | "gray1" => ChatColorStyle::Simple(NamedColor::DarkGray),
            "blue" => ChatColorStyle::Simple(NamedColor::Blue),
            "green" => ChatColorStyle::Simple(NamedColor::Green),
            "aqua" => ChatColorStyle::Simple(NamedColor::Aqua),
            "red" => ChatColorStyle::Simple(NamedColor::Red),
            "light_purple" | "lightpurple" | "pink" => ChatColorStyle::Simple(NamedColor::LightPurple),
            "yellow" => ChatColorStyle::Simple(NamedColor::Yellow),
            "white" => ChatColorStyle::Simple(NamedColor::White),
            _ => {
                p.send_system_message(&pumpkin_util::text::TextComponent::text(
                    "Unknown color or style. Available: black, dark_blue, dark_green, dark_aqua, dark_red, dark_purple, gold, gray, dark_gray, blue, green, aqua, red, light_purple, yellow, white, rainbow, fire"
                )).await;
                return Ok(());
            }
        };
        {
            let mut map = PLAYER_COLORS.lock().await;
            map.insert(p.gameprofile.id, style);
        }
        let feedback = match style {
            ChatColorStyle::Simple(color) => format!("Your chat color is now set to {:?}", color),
            ChatColorStyle::Rainbow => "Your chat color is now set to RAINBOW! 🌈".to_string(),
            ChatColorStyle::Fire => "Your chat color is now set to FIRE! 🔥".to_string(),
        };
        p.send_system_message(&pumpkin_util::text::TextComponent::text(feedback)).await;
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

// Commande /namecolor <color|style>
struct NameColorExecutor;

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
        let color_str = match args.get(ARG_COLOR) {
            Some(Arg::Simple(s)) => *s,
            _ => "",
        };
        let style = match color_str.to_lowercase().as_str() {
            "rainbow" => ChatColorStyle::Rainbow,
            "fire" => ChatColorStyle::Fire,
            "black" => ChatColorStyle::Simple(NamedColor::Black),
            "dark_blue" | "darkblue" | "blue1" => ChatColorStyle::Simple(NamedColor::DarkBlue),
            "dark_green" | "darkgreen" | "green1" => ChatColorStyle::Simple(NamedColor::DarkGreen),
            "dark_aqua" | "darkaqua" | "aqua1" => ChatColorStyle::Simple(NamedColor::DarkAqua),
            "dark_red" | "darkred" | "red1" => ChatColorStyle::Simple(NamedColor::DarkRed),
            "dark_purple" | "darkpurple" | "purple1" => ChatColorStyle::Simple(NamedColor::DarkPurple),
            "gold" => ChatColorStyle::Simple(NamedColor::Gold),
            "gray" => ChatColorStyle::Simple(NamedColor::Gray),
            "dark_gray" | "darkgray" | "gray1" => ChatColorStyle::Simple(NamedColor::DarkGray),
            "blue" => ChatColorStyle::Simple(NamedColor::Blue),
            "green" => ChatColorStyle::Simple(NamedColor::Green),
            "aqua" => ChatColorStyle::Simple(NamedColor::Aqua),
            "red" => ChatColorStyle::Simple(NamedColor::Red),
            "light_purple" | "lightpurple" | "pink" => ChatColorStyle::Simple(NamedColor::LightPurple),
            "yellow" => ChatColorStyle::Simple(NamedColor::Yellow),
            "white" => ChatColorStyle::Simple(NamedColor::White),
            _ => {
                p.send_system_message(&pumpkin_util::text::TextComponent::text(
                    "Unknown color or style. Available: black, dark_blue, dark_green, dark_aqua, dark_red, dark_purple, gold, gray, dark_gray, blue, green, aqua, red, light_purple, yellow, white, rainbow, fire"
                )).await;
                return Ok(());
            }
        };
        {
            let mut map = PLAYER_NAME_COLORS.lock().await;
            map.insert(p.gameprofile.id, style);
        }
        let feedback = match style {
            ChatColorStyle::Simple(color) => format!("Your name color is now set to {:?}", color),
            ChatColorStyle::Rainbow => "Your name color is now set to RAINBOW! 🌈".to_string(),
            ChatColorStyle::Fire => "Your name color is now set to FIRE! 🔥".to_string(),
        };
        p.send_system_message(&pumpkin_util::text::TextComponent::text(feedback)).await;
        Ok(())
    }
}

pub fn init_namecolor_command_tree() -> CommandTree {
    CommandTree::new(["namecolor"], "Set your name color or gradient (ex: red, blue, rainbow, fire).")
        .then(argument(ARG_COLOR, SimpleArgConsumer).execute(NameColorExecutor))
} 