use lsharp_frame_adapter_headless::HeadlessDesktopBackend;
use lsharp_frame_contract::{
    Capability, CapabilitySet, EventBatch, FrameEvent, FrameUpdate, Style, TextRole, UiNode,
    UiNodeId, UiNodeKind, UiPatch, UiTransaction,
};
use lsharp_frame_core::{FrameCore, KernelError, KernelExecutor};
use lsharp_frame_spi::DesktopBackend;

#[derive(Debug, Default)]
struct DemoKernel {
    revision: u64,
    command_count: u64,
}

impl DemoKernel {
    fn render(&mut self) -> FrameUpdate {
        self.revision += 1;
        let root = UiNodeId(1);
        let title = UiNodeId(2);
        FrameUpdate {
            ui: Some(UiTransaction {
                revision: self.revision,
                patches: vec![
                    UiPatch::Upsert(UiNode {
                        id: title,
                        kind: UiNodeKind::Text {
                            text: format!(
                                "L#frame headless kernel — commands: {}",
                                self.command_count
                            ),
                            role: TextRole::Heading,
                        },
                        style: Style {
                            visible: true,
                            ..Style::default()
                        },
                        children: vec![],
                    }),
                    UiPatch::Upsert(UiNode {
                        id: root,
                        kind: UiNodeKind::Column,
                        style: Style {
                            visible: true,
                            gap: 8.0,
                            ..Style::default()
                        },
                        children: vec![title],
                    }),
                    UiPatch::SetRoot(Some(root)),
                ],
            }),
            ..FrameUpdate::default()
        }
    }
}

impl KernelExecutor for DemoKernel {
    fn initialize(&mut self) -> Result<FrameUpdate, KernelError> {
        Ok(self.render())
    }

    fn dispatch(&mut self, events: EventBatch) -> Result<FrameUpdate, KernelError> {
        self.command_count += events
            .events
            .iter()
            .filter(|event| matches!(event, FrameEvent::CommandInvoked { .. }))
            .count() as u64;
        Ok(self.render())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let events = [EventBatch {
        sequence: 1,
        events: vec![FrameEvent::CommandInvoked {
            command: "agent.new-session".into(),
        }],
    }];
    let mut backend = HeadlessDesktopBackend::with_events(events);
    let mut core = FrameCore::new(
        DemoKernel::default(),
        CapabilitySet::new([Capability::UiCore]),
    );

    backend.run(&mut core)?;
    println!(
        "headless replay complete: updates={}, ui_revision={}, nodes={}",
        backend.updates().len(),
        core.ui_document().revision(),
        core.ui_document().node_count()
    );
    Ok(())
}
