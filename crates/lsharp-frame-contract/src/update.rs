use crate::{EffectRequest, UiTransaction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRegistration {
    pub id: String,
    pub title: String,
    pub category: String,
    pub default_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FrameUpdate {
    pub ui: Option<UiTransaction>,
    pub effects: Vec<EffectRequest>,
    pub commands: Vec<CommandRegistration>,
    pub notifications: Vec<Notification>,
    pub subscriptions: Vec<String>,
}
