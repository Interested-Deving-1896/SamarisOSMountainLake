use std::collections::HashSet;
use crate::resources::resource_id::GpuResourceId;

#[derive(Debug, Clone)]
pub struct FrameGraph {
    frame_index: u64,
    resources: HashSet<GpuResourceId>,
    read_set: HashSet<GpuResourceId>,
    write_set: HashSet<GpuResourceId>,
    pass_count: u32,
    barrier_count: u32,
}

impl FrameGraph {
    pub fn new(frame_index: u64) -> Self {
        Self {
            frame_index,
            resources: HashSet::new(),
            read_set: HashSet::new(),
            write_set: HashSet::new(),
            pass_count: 0,
            barrier_count: 0,
        }
    }

    pub fn track_resource(&mut self, id: GpuResourceId) {
        self.resources.insert(id);
    }

    pub fn mark_read(&mut self, id: GpuResourceId) {
        self.resources.insert(id);
        self.read_set.insert(id);
    }

    pub fn mark_write(&mut self, id: GpuResourceId) {
        self.resources.insert(id);
        self.write_set.insert(id);
    }

    pub fn add_pass(&mut self) {
        self.pass_count += 1;
    }

    pub fn add_barrier(&mut self) {
        self.barrier_count += 1;
    }

    pub fn reset(&mut self, next_frame: u64) {
        self.frame_index = next_frame;
        self.resources.clear();
        self.read_set.clear();
        self.write_set.clear();
        self.pass_count = 0;
        self.barrier_count = 0;
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    pub fn pass_count(&self) -> u32 {
        self.pass_count
    }

    pub fn barrier_count(&self) -> u32 {
        self.barrier_count
    }

    pub fn frame_index(&self) -> u64 {
        self.frame_index
    }

    pub fn has_resource(&self, id: &GpuResourceId) -> bool {
        self.resources.contains(id)
    }

    pub fn is_read(&self, id: &GpuResourceId) -> bool {
        self.read_set.contains(id)
    }

    pub fn is_written(&self, id: &GpuResourceId) -> bool {
        self.write_set.contains(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_frame_graph() {
        let fg = FrameGraph::new(42);
        assert_eq!(fg.frame_index(), 42);
        assert_eq!(fg.resource_count(), 0);
    }

    #[test]
    fn test_track_resource() {
        let mut fg = FrameGraph::new(0);
        let id = GpuResourceId::new();
        fg.track_resource(id);
        assert!(fg.has_resource(&id));
        assert_eq!(fg.resource_count(), 1);
    }

    #[test]
    fn test_mark_read_write() {
        let mut fg = FrameGraph::new(0);
        let id = GpuResourceId::new();
        fg.mark_read(id);
        assert!(fg.is_read(&id));
        fg.mark_write(id);
        assert!(fg.is_written(&id));
    }

    #[test]
    fn test_pass_and_barrier_count() {
        let mut fg = FrameGraph::new(0);
        assert_eq!(fg.pass_count(), 0);
        fg.add_pass();
        fg.add_pass();
        assert_eq!(fg.pass_count(), 2);
        fg.add_barrier();
        assert_eq!(fg.barrier_count(), 1);
    }

    #[test]
    fn test_reset() {
        let mut fg = FrameGraph::new(0);
        fg.track_resource(GpuResourceId::new());
        fg.add_pass();
        fg.add_barrier();
        fg.reset(100);
        assert_eq!(fg.frame_index(), 100);
        assert_eq!(fg.resource_count(), 0);
        assert_eq!(fg.pass_count(), 0);
        assert_eq!(fg.barrier_count(), 0);
    }
}
