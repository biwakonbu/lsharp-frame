use crate::{Capability, EffectId, ProcessId, PtySessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtySpawnRequest {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Vec<(String, String)>,
    pub columns: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtyWriteRequest {
    pub session: PtySessionId,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtyResizeRequest {
    pub session: PtySessionId,
    pub columns: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSpawnRequest {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessWriteRequest {
    pub process: ProcessId,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EffectKind {
    SpawnPty(PtySpawnRequest),
    WritePty(PtyWriteRequest),
    ResizePty(PtyResizeRequest),
    SpawnProcess(ProcessSpawnRequest),
    WriteProcess(ProcessWriteRequest),
    ReadFile {
        path: String,
    },
    WriteFile {
        path: String,
        contents: Vec<u8>,
    },
    ShowNotification {
        title: String,
        body: String,
    },
    AgentCommand {
        agent: String,
        payload: Vec<u8>,
    },
    Custom {
        namespace: String,
        name: String,
        payload: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectRequest {
    pub id: EffectId,
    pub required_capability: Capability,
    pub kind: EffectKind,
}
