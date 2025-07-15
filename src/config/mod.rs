use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use pumpkin_util::text::color::NamedColor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChatColorStyle {
    Simple(NamedColor),
    Rainbow,
    Fire,
    CustomGradient(String), // Nom du gradient personnalisé
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub uuid: String,
    pub chat_color: Option<ChatColorStyle>,
    pub name_color: Option<ChatColorStyle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginData {
    pub players: HashMap<String, PlayerData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GradientMethod {
    #[serde(rename = "hsv")]
    Hsv,
    #[serde(rename = "rgb")]
    RgbInterpolation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradientConfig {
    #[serde(rename = "type")]
    pub method: GradientMethod,
    #[serde(default)]
    pub start_hue: Option<f32>,
    #[serde(default)]
    pub end_hue: Option<f32>,
    #[serde(default)]
    pub saturation: Option<f32>,
    #[serde(default)]
    pub value: Option<f32>,
    #[serde(default)]
    pub colors: Option<Vec<[u8; 3]>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub save_interval: u64, // en secondes
    pub auto_save: bool,
    pub simple_colors: HashMap<String, String>,
    pub gradients: HashMap<String, GradientConfig>,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub default_chat_color: String,
    pub default_name_color: String,
} 