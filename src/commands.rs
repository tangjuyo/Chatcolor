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
use crate::PLAYER_COLORS;

const NAMES: [&str; 1] = ["chatcolor"];
const DESCRIPTION: &str = "Set your default chat color.";
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
        let color = match color_str.to_lowercase().as_str() {
            "black" => NamedColor::Black,
            "dark_blue" | "darkblue" | "blue1" => NamedColor::DarkBlue,
            "dark_green" | "darkgreen" | "green1" => NamedColor::DarkGreen,
            "dark_aqua" | "darkaqua" | "aqua1" => NamedColor::DarkAqua,
            "dark_red" | "darkred" | "red1" => NamedColor::DarkRed,
            "dark_purple" | "darkpurple" | "purple1" => NamedColor::DarkPurple,
            "gold" => NamedColor::Gold,
            "gray" => NamedColor::Gray,
            "dark_gray" | "darkgray" | "gray1" => NamedColor::DarkGray,
            "blue" => NamedColor::Blue,
            "green" => NamedColor::Green,
            "aqua" => NamedColor::Aqua,
            "red" => NamedColor::Red,
            "light_purple" | "lightpurple" | "pink" => NamedColor::LightPurple,
            "yellow" => NamedColor::Yellow,
            "white" => NamedColor::White,
            _ => {
                p.send_system_message(&pumpkin_util::text::TextComponent::text(
                    "Unknown color. Available: black, dark_blue, dark_green, dark_aqua, dark_red, dark_purple, gold, gray, dark_gray, blue, green, aqua, red, light_purple, yellow, white"
                )).await;
                return Ok(());
            }
        };
        {
            let mut map = PLAYER_COLORS.lock().await;
            map.insert(p.gameprofile.id, color);
        }
        p.send_system_message(&pumpkin_util::text::TextComponent::text(
            format!("Your chat color is now set to {:?}", color)
        )).await;
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