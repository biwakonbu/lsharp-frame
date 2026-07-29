//! Stable, crate-independent contracts used by the L#frame core, adapters,
//! and generated L# component bindings.

mod capability;
mod effect;
mod event;
mod id;
mod plugin;
mod ui;
mod update;

pub use capability::{Capability, CapabilitySet};
pub use effect::{
    EffectKind, EffectRequest, ProcessSpawnRequest, ProcessWriteRequest, PtyResizeRequest,
    PtySpawnRequest, PtyWriteRequest,
};
pub use event::{
    AgentEvent, EffectCompletedEvent, EventBatch, FrameEvent, ProcessEvent, PtyEvent, UiEvent,
};
pub use id::{ArtifactId, DisplayListId, EffectId, ProcessId, PtySessionId, SurfaceId, UiNodeId};
pub use plugin::PluginManifest;
pub use ui::{
    Axis, Insets, Length, Style, TextRole, UiDocument, UiError, UiNode, UiNodeKind, UiPatch,
    UiTransaction,
};
pub use update::{CommandRegistration, FrameUpdate, Notification};
