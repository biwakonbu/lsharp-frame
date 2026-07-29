use crate::{Capability, CapabilitySet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub required_capabilities: CapabilitySet,
    pub optional_capabilities: CapabilitySet,
}

impl PluginManifest {
    pub fn requires(&self, capability: Capability) -> bool {
        self.required_capabilities.contains(capability)
    }
}
