use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Capability {
    UiCore,
    UiCanvas,
    UiNativeSurface,
    ProcessSpawn,
    PtySpawn,
    FileSystemRead,
    FileSystemWrite,
    NetworkConnect,
    ClipboardRead,
    ClipboardWrite,
    NotificationsShow,
    AgentControl,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CapabilitySet {
    values: BTreeSet<Capability>,
}

impl CapabilitySet {
    pub fn new(values: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            values: values.into_iter().collect(),
        }
    }

    pub fn contains(&self, capability: Capability) -> bool {
        self.values.contains(&capability)
    }

    pub fn insert(&mut self, capability: Capability) -> bool {
        self.values.insert(capability)
    }

    pub fn iter(&self) -> impl Iterator<Item = Capability> + '_ {
        self.values.iter().copied()
    }
}
