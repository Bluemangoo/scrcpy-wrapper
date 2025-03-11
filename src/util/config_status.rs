use crate::config::Config;

#[derive(Debug, Clone, Default)]
pub struct ConfigStatus{
    pub chosen: String,
    pub input: String,
}

pub fn select_config_valid(chosen: &String, config: &Config) -> bool {
    if chosen.trim().is_empty() {
        return false;
    }
    config.saved.contains_key(chosen)
}