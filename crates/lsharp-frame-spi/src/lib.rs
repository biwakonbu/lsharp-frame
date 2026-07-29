//! Ports implemented by concrete desktop, PTY, and process adapters.

use std::fmt;

use lsharp_frame_contract::{
    EventBatch, FrameUpdate, ProcessEvent, ProcessId, ProcessSpawnRequest, PtyEvent,
    PtyResizeRequest, PtySessionId, PtySpawnRequest,
};

pub trait DesktopDelegate {
    fn initialize(&mut self) -> Result<FrameUpdate, DesktopError>;
    fn dispatch(&mut self, events: EventBatch) -> Result<FrameUpdate, DesktopError>;
}

pub trait DesktopBackend {
    fn run(&mut self, delegate: &mut dyn DesktopDelegate) -> Result<(), DesktopError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopError {
    Kernel(String),
    Backend(String),
}

impl fmt::Display for DesktopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(message) => write!(formatter, "kernel error: {message}"),
            Self::Backend(message) => write!(formatter, "desktop backend error: {message}"),
        }
    }
}

impl std::error::Error for DesktopError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyCommand {
    Write {
        session: PtySessionId,
        data: Vec<u8>,
    },
    Resize(PtyResizeRequest),
    Interrupt {
        session: PtySessionId,
    },
    Terminate {
        session: PtySessionId,
    },
    Close {
        session: PtySessionId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PtyBackendError {
    Unsupported,
    NotFound(PtySessionId),
    SpawnFailed(String),
    Io(String),
}

pub trait PtyBackend: Send + Sync {
    fn spawn(&self, request: PtySpawnRequest) -> Result<PtySessionId, PtyBackendError>;
    fn submit(&self, command: PtyCommand) -> Result<(), PtyBackendError>;
    fn drain_events(&self, limit: usize) -> Vec<PtyEvent>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessCommand {
    Write { process: ProcessId, data: Vec<u8> },
    Interrupt { process: ProcessId },
    Terminate { process: ProcessId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessBackendError {
    Unsupported,
    NotFound(ProcessId),
    SpawnFailed(String),
    Io(String),
}

pub trait ProcessBackend: Send + Sync {
    fn spawn(&self, request: ProcessSpawnRequest) -> Result<ProcessId, ProcessBackendError>;
    fn submit(&self, command: ProcessCommand) -> Result<(), ProcessBackendError>;
    fn drain_events(&self, limit: usize) -> Vec<ProcessEvent>;
}
