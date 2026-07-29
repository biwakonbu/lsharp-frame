//! L#frame orchestration independent of Wasmtime and concrete desktop crates.

use std::fmt;

use lsharp_frame_contract::{Capability, CapabilitySet, EventBatch, FrameUpdate, UiDocument};
use lsharp_frame_spi::{DesktopDelegate, DesktopError};

pub trait KernelExecutor {
    fn initialize(&mut self) -> Result<FrameUpdate, KernelError>;
    fn dispatch(&mut self, events: EventBatch) -> Result<FrameUpdate, KernelError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelError {
    Execution(String),
    Contract(String),
}

impl fmt::Display for KernelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Execution(message) => write!(formatter, "kernel execution failed: {message}"),
            Self::Contract(message) => write!(formatter, "kernel contract failed: {message}"),
        }
    }
}

impl std::error::Error for KernelError {}

#[derive(Debug)]
pub struct FrameCore<K> {
    kernel: K,
    granted_capabilities: CapabilitySet,
    ui_document: UiDocument,
}

impl<K> FrameCore<K>
where
    K: KernelExecutor,
{
    pub fn new(kernel: K, granted_capabilities: CapabilitySet) -> Self {
        Self {
            kernel,
            granted_capabilities,
            ui_document: UiDocument::default(),
        }
    }

    pub fn ui_document(&self) -> &UiDocument {
        &self.ui_document
    }

    fn accept_update(&mut self, update: FrameUpdate) -> Result<FrameUpdate, KernelError> {
        for effect in &update.effects {
            if !self
                .granted_capabilities
                .contains(effect.required_capability)
            {
                return Err(KernelError::Contract(format!(
                    "effect {} requested denied capability {:?}",
                    effect.id, effect.required_capability
                )));
            }
        }

        if let Some(transaction) = &update.ui {
            self.ui_document
                .apply(transaction)
                .map_err(|error| KernelError::Contract(error.to_string()))?;
        }
        Ok(update)
    }

    pub fn grant(&mut self, capability: Capability) {
        self.granted_capabilities.insert(capability);
    }
}

impl<K> DesktopDelegate for FrameCore<K>
where
    K: KernelExecutor,
{
    fn initialize(&mut self) -> Result<FrameUpdate, DesktopError> {
        let update = self
            .kernel
            .initialize()
            .map_err(|error| DesktopError::Kernel(error.to_string()))?;
        self.accept_update(update)
            .map_err(|error| DesktopError::Kernel(error.to_string()))
    }

    fn dispatch(&mut self, events: EventBatch) -> Result<FrameUpdate, DesktopError> {
        let update = self
            .kernel
            .dispatch(events)
            .map_err(|error| DesktopError::Kernel(error.to_string()))?;
        self.accept_update(update)
            .map_err(|error| DesktopError::Kernel(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use lsharp_frame_contract::{
        Axis, Capability, EffectId, EffectKind, EffectRequest, FrameUpdate, Style, UiNode,
        UiNodeId, UiNodeKind, UiPatch, UiTransaction,
    };

    use super::*;

    #[derive(Debug)]
    struct StaticKernel {
        update: FrameUpdate,
    }

    impl KernelExecutor for StaticKernel {
        fn initialize(&mut self) -> Result<FrameUpdate, KernelError> {
            Ok(self.update.clone())
        }

        fn dispatch(&mut self, _events: EventBatch) -> Result<FrameUpdate, KernelError> {
            Ok(FrameUpdate::default())
        }
    }

    #[test]
    fn applies_kernel_ui_transaction() {
        let update = FrameUpdate {
            ui: Some(UiTransaction {
                revision: 1,
                patches: vec![
                    UiPatch::Upsert(UiNode {
                        id: UiNodeId(1),
                        kind: UiNodeKind::Split {
                            axis: Axis::Horizontal,
                            ratio: 0.25,
                        },
                        style: Style {
                            visible: true,
                            ..Style::default()
                        },
                        children: vec![],
                    }),
                    UiPatch::SetRoot(Some(UiNodeId(1))),
                ],
            }),
            ..FrameUpdate::default()
        };
        let mut core = FrameCore::new(
            StaticKernel { update },
            CapabilitySet::new([Capability::UiCore]),
        );

        DesktopDelegate::initialize(&mut core).expect("kernel update should be accepted");
        assert_eq!(core.ui_document().root(), Some(UiNodeId(1)));
    }

    #[test]
    fn denied_effect_fails_closed() {
        let update = FrameUpdate {
            effects: vec![EffectRequest {
                id: EffectId(9),
                required_capability: Capability::PtySpawn,
                kind: EffectKind::ShowNotification {
                    title: "not relevant".into(),
                    body: "capability is authoritative".into(),
                },
            }],
            ..FrameUpdate::default()
        };
        let mut core = FrameCore::new(
            StaticKernel { update },
            CapabilitySet::new([Capability::UiCore]),
        );

        let error = DesktopDelegate::initialize(&mut core).expect_err("capability must be denied");
        assert!(error.to_string().contains("PtySpawn"));
    }
}
