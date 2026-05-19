use crate::scheduler::command::GpuCommand;
use crate::scheduler::priority::GpuPriority;

pub struct GpuBatch {
    pub commands: Vec<GpuCommand>,
    pub total_estimate_us: u64,
    pub priority: GpuPriority,
}

impl GpuBatch {
    pub fn new(priority: GpuPriority) -> Self {
        GpuBatch {
            commands: Vec::new(),
            total_estimate_us: 0,
            priority,
        }
    }

    pub fn add(&mut self, cmd: GpuCommand) {
        self.total_estimate_us += match cmd.kind {
            crate::scheduler::command::GpuCommandKind::Render => 8000,
            crate::scheduler::command::GpuCommandKind::Compute => 4000,
            crate::scheduler::command::GpuCommandKind::Transfer => 1000,
            crate::scheduler::command::GpuCommandKind::Compress => 2000,
            crate::scheduler::command::GpuCommandKind::Decompress => 2000,
            crate::scheduler::command::GpuCommandKind::Barrier => 500,
        };
        self.commands.push(cmd);
    }

    pub fn can_add(&self, cmd: &GpuCommand) -> bool {
        if cmd.priority != self.priority {
            return false;
        }
        self.commands.len() < self.priority.batch_size()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::command::GpuCommandKind;

    fn make_cmd(kind: GpuCommandKind, priority: GpuPriority) -> GpuCommand {
        GpuCommand::new(kind, priority, "batch_test")
    }

    #[test]
    fn test_new_batch() {
        let b = GpuBatch::new(GpuPriority::High);
        assert!(b.is_empty());
        assert_eq!(b.total_estimate_us, 0);
        assert_eq!(b.priority, GpuPriority::High);
    }

    #[test]
    fn test_add_increases_estimate() {
        let mut b = GpuBatch::new(GpuPriority::Normal);
        b.add(make_cmd(GpuCommandKind::Render, GpuPriority::Normal));
        assert_eq!(b.total_estimate_us, 8000);
        assert_eq!(b.commands.len(), 1);
    }

    #[test]
    fn test_can_add_same_priority() {
        let mut b = GpuBatch::new(GpuPriority::Critical);
        let cmd = make_cmd(GpuCommandKind::Compute, GpuPriority::Critical);
        assert!(b.can_add(&cmd));
        b.add(cmd);
        let cmd2 = make_cmd(GpuCommandKind::Compute, GpuPriority::Critical);
        assert!(!b.can_add(&cmd2));
    }

    #[test]
    fn test_can_add_wrong_priority() {
        let b = GpuBatch::new(GpuPriority::High);
        let cmd = make_cmd(GpuCommandKind::Compute, GpuPriority::Normal);
        assert!(!b.can_add(&cmd));
    }

    #[test]
    fn test_is_empty() {
        let mut b = GpuBatch::new(GpuPriority::Idle);
        assert!(b.is_empty());
        b.add(make_cmd(GpuCommandKind::Transfer, GpuPriority::Idle));
        assert!(!b.is_empty());
    }

    #[test]
    fn test_batch_size_limit() {
        let mut b = GpuBatch::new(GpuPriority::Critical);
        for _ in 0..5 {
            b.add(make_cmd(GpuCommandKind::Barrier, GpuPriority::Critical));
        }
        assert_eq!(b.commands.len(), 5);
    }
}
