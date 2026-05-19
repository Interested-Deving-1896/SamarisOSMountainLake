use crate::resources::resource_id::GpuResourceId;
use crate::scheduler::priority::GpuPriority;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuCommandKind {
    Render,
    Compute,
    Transfer,
    Compress,
    Decompress,
    Barrier,
}

#[derive(Debug, Clone)]
pub struct GpuCommand {
    pub id: u64,
    pub kind: GpuCommandKind,
    pub priority: GpuPriority,
    pub resources: Vec<GpuResourceId>,
    pub label: String,
}

impl GpuCommand {
    pub fn new(kind: GpuCommandKind, priority: GpuPriority, label: &str) -> Self {
        GpuCommand {
            id: 0,
            kind,
            priority,
            resources: Vec::new(),
            label: label.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_command() {
        let cmd = GpuCommand::new(
            GpuCommandKind::Compute,
            GpuPriority::High,
            "test compute",
        );
        assert_eq!(cmd.kind, GpuCommandKind::Compute);
        assert_eq!(cmd.priority, GpuPriority::High);
        assert_eq!(cmd.label, "test compute");
        assert!(cmd.resources.is_empty());
    }

    #[test]
    fn test_render_command() {
        let cmd = GpuCommand::new(GpuCommandKind::Render, GpuPriority::Critical, "frame");
        assert_eq!(cmd.kind, GpuCommandKind::Render);
    }

    #[test]
    fn test_transfer_command() {
        let cmd = GpuCommand::new(GpuCommandKind::Transfer, GpuPriority::Normal, "upload");
        assert_eq!(cmd.kind, GpuCommandKind::Transfer);
    }

    #[test]
    fn test_clone() {
        let cmd = GpuCommand::new(GpuCommandKind::Barrier, GpuPriority::Idle, "sync");
        let cloned = cmd.clone();
        assert_eq!(cmd.id, cloned.id);
        assert_eq!(cmd.kind, cloned.kind);
    }

    #[test]
    fn test_resources_vec() {
        let mut cmd = GpuCommand::new(GpuCommandKind::Compress, GpuPriority::Normal, "compress");
        let rid = GpuResourceId::new();
        cmd.resources.push(rid);
        assert_eq!(cmd.resources.len(), 1);
    }
}
