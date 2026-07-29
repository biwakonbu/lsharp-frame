use crate::{EffectId, ProcessId, PtySessionId, UiNodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent {
    Invoked {
        node: UiNodeId,
    },
    TextChanged {
        node: UiNodeId,
        text: String,
    },
    FocusChanged {
        node: Option<UiNodeId>,
    },
    Custom {
        node: UiNodeId,
        name: String,
        payload: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyEvent {
    Spawned {
        session: PtySessionId,
    },
    Output {
        session: PtySessionId,
        sequence: u64,
        data: Vec<u8>,
    },
    Exited {
        session: PtySessionId,
        exit_code: Option<i32>,
    },
    Failed {
        session: Option<PtySessionId>,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessEvent {
    Spawned {
        process: ProcessId,
    },
    Stdout {
        process: ProcessId,
        sequence: u64,
        data: Vec<u8>,
    },
    Stderr {
        process: ProcessId,
        sequence: u64,
        data: Vec<u8>,
    },
    Exited {
        process: ProcessId,
        exit_code: Option<i32>,
    },
    Failed {
        process: Option<ProcessId>,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEvent {
    pub agent: String,
    pub session: String,
    pub kind: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectCompletedEvent {
    pub effect: EffectId,
    pub success: bool,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrameEvent {
    Ui(UiEvent),
    CommandInvoked {
        command: String,
    },
    Pty(PtyEvent),
    Process(ProcessEvent),
    Agent(AgentEvent),
    EffectCompleted(EffectCompletedEvent),
    Timer {
        timer: String,
        tick: u64,
    },
    Custom {
        namespace: String,
        name: String,
        payload: Vec<u8>,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventBatch {
    pub sequence: u64,
    pub events: Vec<FrameEvent>,
}
