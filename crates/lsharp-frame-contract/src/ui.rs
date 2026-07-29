use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::{ArtifactId, DisplayListId, PtySessionId, SurfaceId, UiNodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    Fill,
    Fixed(f32),
    Fraction(f32),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Insets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    pub background_token: Option<String>,
    pub foreground_token: Option<String>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub padding: Insets,
    pub gap: f32,
    pub radius: f32,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRole {
    Body,
    Label,
    Heading,
    Monospace,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum UiNodeKind {
    Row,
    Column,
    Stack,
    Grid {
        columns: u16,
    },
    Split {
        axis: Axis,
        ratio: f32,
    },
    Tabs,
    Scroll,
    VirtualList {
        item_count: u64,
        first_visible: u64,
        visible_count: u32,
    },
    Text {
        text: String,
        role: TextRole,
    },
    Button {
        label: String,
        command: String,
    },
    TextInput {
        value: String,
        placeholder: String,
        multiline: bool,
    },
    Terminal {
        session: PtySessionId,
    },
    Diff {
        artifact: ArtifactId,
    },
    Canvas {
        display_list: DisplayListId,
    },
    NativeSurface {
        provider: String,
        surface: SurfaceId,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    pub id: UiNodeId,
    pub kind: UiNodeKind,
    pub style: Style,
    pub children: Vec<UiNodeId>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiPatch {
    Upsert(UiNode),
    Remove(UiNodeId),
    ReplaceChildren {
        parent: UiNodeId,
        children: Vec<UiNodeId>,
    },
    SetStyle {
        node: UiNodeId,
        style: Style,
    },
    SetRoot(Option<UiNodeId>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UiTransaction {
    pub revision: u64,
    pub patches: Vec<UiPatch>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UiDocument {
    revision: u64,
    root: Option<UiNodeId>,
    nodes: BTreeMap<UiNodeId, UiNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiError {
    RevisionDidNotAdvance { current: u64, proposed: u64 },
    MissingNode(UiNodeId),
    RootMissing(UiNodeId),
    ChildMissing { parent: UiNodeId, child: UiNodeId },
    CycleDetected(UiNodeId),
}

impl fmt::Display for UiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RevisionDidNotAdvance { current, proposed } => {
                write!(
                    formatter,
                    "UI revision must advance: current={current}, proposed={proposed}"
                )
            }
            Self::MissingNode(node) => write!(formatter, "UI node {node} does not exist"),
            Self::RootMissing(node) => write!(formatter, "UI root {node} does not exist"),
            Self::ChildMissing { parent, child } => {
                write!(
                    formatter,
                    "UI node {parent} references missing child {child}"
                )
            }
            Self::CycleDetected(node) => write!(formatter, "UI cycle detected at node {node}"),
        }
    }
}

impl std::error::Error for UiError {}

impl UiDocument {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn root(&self) -> Option<UiNodeId> {
        self.root
    }

    pub fn node(&self, id: UiNodeId) -> Option<&UiNode> {
        self.nodes.get(&id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn apply(&mut self, transaction: &UiTransaction) -> Result<(), UiError> {
        if transaction.revision <= self.revision {
            return Err(UiError::RevisionDidNotAdvance {
                current: self.revision,
                proposed: transaction.revision,
            });
        }

        let mut candidate = self.clone();
        for patch in &transaction.patches {
            candidate.apply_patch(patch)?;
        }
        candidate.validate()?;
        candidate.revision = transaction.revision;
        *self = candidate;
        Ok(())
    }

    fn apply_patch(&mut self, patch: &UiPatch) -> Result<(), UiError> {
        match patch {
            UiPatch::Upsert(node) => {
                self.nodes.insert(node.id, node.clone());
            }
            UiPatch::Remove(node) => {
                self.nodes.remove(node);
                if self.root == Some(*node) {
                    self.root = None;
                }
                for current in self.nodes.values_mut() {
                    current.children.retain(|child| child != node);
                }
            }
            UiPatch::ReplaceChildren { parent, children } => {
                let node = self
                    .nodes
                    .get_mut(parent)
                    .ok_or(UiError::MissingNode(*parent))?;
                node.children.clone_from(children);
            }
            UiPatch::SetStyle { node, style } => {
                let current = self
                    .nodes
                    .get_mut(node)
                    .ok_or(UiError::MissingNode(*node))?;
                current.style = style.clone();
            }
            UiPatch::SetRoot(root) => self.root = *root,
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), UiError> {
        if let Some(root) = self.root
            && !self.nodes.contains_key(&root)
        {
            return Err(UiError::RootMissing(root));
        }

        for node in self.nodes.values() {
            for child in &node.children {
                if !self.nodes.contains_key(child) {
                    return Err(UiError::ChildMissing {
                        parent: node.id,
                        child: *child,
                    });
                }
            }
        }

        if let Some(root) = self.root {
            let mut visiting = BTreeSet::new();
            let mut visited = BTreeSet::new();
            self.visit(root, &mut visiting, &mut visited)?;
        }
        Ok(())
    }

    fn visit(
        &self,
        node: UiNodeId,
        visiting: &mut BTreeSet<UiNodeId>,
        visited: &mut BTreeSet<UiNodeId>,
    ) -> Result<(), UiError> {
        if visited.contains(&node) {
            return Ok(());
        }
        if !visiting.insert(node) {
            return Err(UiError::CycleDetected(node));
        }

        let current = self.nodes.get(&node).ok_or(UiError::MissingNode(node))?;
        for child in &current.children {
            self.visit(*child, visiting, visited)?;
        }

        visiting.remove(&node);
        visited.insert(node);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: u64, children: Vec<UiNodeId>) -> UiNode {
        UiNode {
            id: UiNodeId(id),
            kind: UiNodeKind::Column,
            style: Style {
                visible: true,
                ..Style::default()
            },
            children,
        }
    }

    #[test]
    fn transaction_is_atomic_when_validation_fails() {
        let mut document = UiDocument::default();
        let invalid = UiTransaction {
            revision: 1,
            patches: vec![
                UiPatch::Upsert(node(1, vec![UiNodeId(2)])),
                UiPatch::SetRoot(Some(UiNodeId(1))),
            ],
        };

        assert_eq!(
            document.apply(&invalid),
            Err(UiError::ChildMissing {
                parent: UiNodeId(1),
                child: UiNodeId(2),
            })
        );
        assert_eq!(document.revision(), 0);
        assert_eq!(document.node_count(), 0);
    }

    #[test]
    fn valid_tree_is_committed() {
        let mut document = UiDocument::default();
        let transaction = UiTransaction {
            revision: 1,
            patches: vec![
                UiPatch::Upsert(node(2, vec![])),
                UiPatch::Upsert(node(1, vec![UiNodeId(2)])),
                UiPatch::SetRoot(Some(UiNodeId(1))),
            ],
        };

        document.apply(&transaction).expect("valid UI transaction");
        assert_eq!(document.revision(), 1);
        assert_eq!(document.root(), Some(UiNodeId(1)));
        assert_eq!(document.node_count(), 2);
    }

    #[test]
    fn cycles_are_rejected() {
        let mut document = UiDocument::default();
        let transaction = UiTransaction {
            revision: 1,
            patches: vec![
                UiPatch::Upsert(node(1, vec![UiNodeId(2)])),
                UiPatch::Upsert(node(2, vec![UiNodeId(1)])),
                UiPatch::SetRoot(Some(UiNodeId(1))),
            ],
        };

        assert!(matches!(
            document.apply(&transaction),
            Err(UiError::CycleDetected(_))
        ));
    }
}
