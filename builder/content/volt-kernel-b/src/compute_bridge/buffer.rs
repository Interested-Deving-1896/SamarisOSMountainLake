use std::collections::HashMap;
use uuid::Uuid;

use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone)]
pub struct BufferHandle {
    pub id: Uuid,
    pub app_id: u32,
    pub size: u64,
}

pub struct BufferManager {
    buffers: HashMap<Uuid, BufferEntry>,
    total_allocated: u64,
    max_total: u64,
}

struct BufferEntry {
    handle: BufferHandle,
    data: Vec<u8>,
}

impl BufferManager {
    pub fn new(max_total_bytes: u64) -> Self {
        Self {
            buffers: HashMap::new(),
            total_allocated: 0,
            max_total: max_total_bytes,
        }
    }

    pub fn allocate(&mut self, app_id: u32, size: u64) -> Result<BufferHandle> {
        if size == 0 {
            return Err(TesseractError::Compute("cannot allocate zero-size buffer".into()));
        }

        if self.total_allocated + size > self.max_total {
            return Err(TesseractError::Compute(format!(
                "out of buffer memory: {}/{}",
                self.total_allocated, self.max_total
            )));
        }

        let handle = BufferHandle {
            id: Uuid::new_v4(),
            app_id,
            size,
        };

        let data = vec![0u8; size as usize];
        self.buffers.insert(handle.id, BufferEntry {
            handle: handle.clone(),
            data,
        });
        self.total_allocated += size;

        Ok(handle)
    }

    pub fn free(&mut self, handle_id: &Uuid) -> Result<()> {
        let entry = self.buffers.remove(handle_id).ok_or_else(|| {
            TesseractError::NotFound(format!("buffer {} not found", handle_id))
        })?;
        self.total_allocated = self.total_allocated.saturating_sub(entry.handle.size);
        Ok(())
    }

    pub fn write(&mut self, handle_id: &Uuid, data: &[u8], offset: u64) -> Result<()> {
        let entry = self.buffers.get_mut(handle_id).ok_or_else(|| {
            TesseractError::NotFound(format!("buffer {} not found", handle_id))
        })?;

        let end = offset.saturating_add(data.len() as u64);
        if end > entry.handle.size {
            return Err(TesseractError::Compute("write past buffer end".into()));
        }

        entry.data[offset as usize..end as usize].copy_from_slice(data);
        Ok(())
    }

    pub fn read(&self, handle_id: &Uuid, offset: u64, len: u64) -> Result<Vec<u8>> {
        let entry = self.buffers.get(handle_id).ok_or_else(|| {
            TesseractError::NotFound(format!("buffer {} not found", handle_id))
        })?;

        let end = offset.saturating_add(len);
        if end > entry.handle.size {
            return Err(TesseractError::Compute("read past buffer end".into()));
        }

        Ok(entry.data[offset as usize..end as usize].to_vec())
    }

    pub fn total_allocated(&self) -> u64 {
        self.total_allocated
    }

    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    pub fn cleanup_app(&mut self, app_id: u32) {
        let ids: Vec<Uuid> = self.buffers
            .iter()
            .filter(|(_, e)| e.handle.app_id == app_id)
            .map(|(id, _)| *id)
            .collect();

        for id in ids {
            self.free(&id).ok();
        }
    }

    pub fn reset(&mut self) {
        self.buffers.clear();
        self.total_allocated = 0;
    }
}
