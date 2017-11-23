//! Config loading stuff
//! TODO: Figure out what I am trying to do here.
use toml;

/// General game settings, should not be edited by player
/// TODO: Maybe use this at compile time? Or something?
#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub graphics: GraphicsSettings,
}

/// Graphics settings
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub vsync: bool,
    pub multisampling: u16,
    pub title: String,
}
