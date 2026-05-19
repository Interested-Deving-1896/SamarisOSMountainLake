use crate::resources::resource_id::GpuResourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuBufferUsage {
    Vertex,
    Index,
    Uniform,
    Storage,
    Indirect,
    MapRead,
    MapWrite,
}

impl GpuBufferUsage {
    pub fn name(&self) -> &'static str {
        match self {
            GpuBufferUsage::Vertex => "Vertex",
            GpuBufferUsage::Index => "Index",
            GpuBufferUsage::Uniform => "Uniform",
            GpuBufferUsage::Storage => "Storage",
            GpuBufferUsage::Indirect => "Indirect",
            GpuBufferUsage::MapRead => "MapRead",
            GpuBufferUsage::MapWrite => "MapWrite",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuBuffer {
    pub id: GpuResourceId,
    pub size: u64,
    pub usage: GpuBufferUsage,
    pub label: String,
}

impl GpuBuffer {
    pub fn new(id: GpuResourceId, size: u64, usage: GpuBufferUsage, label: &str) -> Self {
        Self {
            id,
            size,
            usage,
            label: label.to_string(),
        }
    }

    pub fn size_bytes(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let id = GpuResourceId::new();
        let buf = GpuBuffer::new(id, 4096, GpuBufferUsage::Vertex, "mesh_verts");
        assert_eq!(buf.id, id);
        assert_eq!(buf.size, 4096);
        assert_eq!(buf.usage, GpuBufferUsage::Vertex);
        assert_eq!(buf.label, "mesh_verts");
    }

    #[test]
    fn test_size_bytes() {
        let buf = GpuBuffer::new(GpuResourceId::new(), 8192, GpuBufferUsage::Index, "indices");
        assert_eq!(buf.size_bytes(), 8192);
    }

    #[test]
    fn test_clone() {
        let buf = GpuBuffer::new(GpuResourceId::new(), 256, GpuBufferUsage::Uniform, "ubo");
        let cloned = buf.clone();
        assert_eq!(buf.size, cloned.size);
        assert_eq!(buf.label, cloned.label);
    }

    #[test]
    fn test_usage_names() {
        assert_eq!(GpuBufferUsage::Vertex.name(), "Vertex");
        assert_eq!(GpuBufferUsage::Uniform.name(), "Uniform");
        assert_eq!(GpuBufferUsage::MapWrite.name(), "MapWrite");
    }

    #[test]
    fn test_all_buffer_usages() {
        let usages = vec![
            GpuBufferUsage::Vertex,
            GpuBufferUsage::Index,
            GpuBufferUsage::Uniform,
            GpuBufferUsage::Storage,
            GpuBufferUsage::Indirect,
            GpuBufferUsage::MapRead,
            GpuBufferUsage::MapWrite,
        ];
        for u in &usages {
            assert!(!u.name().is_empty());
        }
    }
}
