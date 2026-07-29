//! Deterministic desktop adapter for tests, replay, and benchmarks.

use std::collections::VecDeque;

use lsharp_frame_contract::{EventBatch, FrameUpdate};
use lsharp_frame_spi::{DesktopBackend, DesktopDelegate, DesktopError};

#[derive(Debug, Default)]
pub struct HeadlessDesktopBackend {
    queued_events: VecDeque<EventBatch>,
    updates: Vec<FrameUpdate>,
}

impl HeadlessDesktopBackend {
    pub fn with_events(events: impl IntoIterator<Item = EventBatch>) -> Self {
        Self {
            queued_events: events.into_iter().collect(),
            updates: Vec::new(),
        }
    }

    pub fn updates(&self) -> &[FrameUpdate] {
        &self.updates
    }
}

impl DesktopBackend for HeadlessDesktopBackend {
    fn run(&mut self, delegate: &mut dyn DesktopDelegate) -> Result<(), DesktopError> {
        self.updates.push(delegate.initialize()?);
        while let Some(events) = self.queued_events.pop_front() {
            self.updates.push(delegate.dispatch(events)?);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lsharp_frame_contract::FrameUpdate;

    use super::*;

    #[derive(Debug, Default)]
    struct CountingDelegate {
        dispatched: usize,
    }

    impl DesktopDelegate for CountingDelegate {
        fn initialize(&mut self) -> Result<FrameUpdate, DesktopError> {
            Ok(FrameUpdate::default())
        }

        fn dispatch(&mut self, _events: EventBatch) -> Result<FrameUpdate, DesktopError> {
            self.dispatched += 1;
            Ok(FrameUpdate::default())
        }
    }

    #[test]
    fn replays_batches_in_order() {
        let mut backend = HeadlessDesktopBackend::with_events([
            EventBatch {
                sequence: 1,
                events: vec![],
            },
            EventBatch {
                sequence: 2,
                events: vec![],
            },
        ]);
        let mut delegate = CountingDelegate::default();

        backend.run(&mut delegate).expect("headless replay");
        assert_eq!(delegate.dispatched, 2);
        assert_eq!(backend.updates().len(), 3);
    }
}
