use crate::config::{ChatColorStyle};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

// Stockage global de la couleur/style par défaut de chaque joueur
pub static PLAYER_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Stockage global de la couleur/style du pseudo de chaque joueur
pub static PLAYER_NAME_COLORS: Lazy<Mutex<HashMap<Uuid, ChatColorStyle>>> = Lazy::new(|| Mutex::new(HashMap::new())); 