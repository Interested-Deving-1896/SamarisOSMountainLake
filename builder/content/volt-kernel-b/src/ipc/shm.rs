use std::sync::Arc;

use crate::core::error::{Result, TesseractError};
use crate::protocol::ring_buffer::RingBuffer;
use crate::protocol::TesseractCommand;

pub struct SharedMemoryRing {
    name: String,
    ring: Arc<RingBuffer<Vec<u8>>>,
    fd: Option<std::fs::File>,
    mapped: Option<*mut u8>,
    size: usize,
}

impl SharedMemoryRing {
    pub fn new(name: &str, capacity: usize) -> Result<Self> {
        let ring = Arc::new(RingBuffer::new(capacity));
        Ok(Self {
            name: name.to_string(),
            ring,
            fd: None,
            mapped: None,
            size: 0,
        })
    }

    pub fn create_linux_shm(name: &str, size: usize) -> Result<Self> {
        #[cfg(target_os = "linux")]
        {
            use std::ffi::CString;
            use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};

            let shm_name = CString::new(format!("/tesseract-{name}"))
                .map_err(|e| TesseractError::System(format!("shm name: {e}")))?;

            let fd = unsafe {
                let fd_raw = libc::shm_open(
                    shm_name.as_ptr(),
                    libc::O_CREAT | libc::O_RDWR,
                    libc::S_IRWXU,
                );
                if fd_raw < 0 {
                    return Err(TesseractError::System(format!(
                        "shm_open failed: {}",
                        std::io::Error::last_os_error()
                    )));
                }
                OwnedFd::from_raw_fd(fd_raw)
            };

            if unsafe { libc::ftruncate(fd.as_raw_fd(), size as i64) } < 0 {
                return Err(TesseractError::System(format!(
                    "ftruncate failed: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let ptr = unsafe {
                libc::mmap(
                    std::ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd.as_raw_fd(),
                    0,
                )
            };

            if ptr == libc::MAP_FAILED {
                return Err(TesseractError::System(format!(
                    "mmap failed: {}",
                    std::io::Error::last_os_error()
                )));
            }

            let file = unsafe { std::fs::File::from_raw_fd(fd.into_raw_fd()) };
            let ring = Arc::new(RingBuffer::new(size));

            Ok(Self {
                name: shm_name.to_string_lossy().into_owned(),
                ring,
                fd: Some(file),
                mapped: Some(ptr as *mut u8),
                size,
            })
        }

        #[cfg(not(target_os = "linux"))]
        {
            let _ = name;
            let _ = size;
            Err(TesseractError::System("SHM requires Linux".into()))
        }
    }

    pub fn send(&self, cmd: &TesseractCommand) -> Result<()> {
        let data = cmd.to_bytes();
        self.ring.try_push(data)
    }

    pub fn recv(&self) -> Option<TesseractCommand> {
        self.ring.try_pop().and_then(|data| {
            TesseractCommand::from_bytes(&data).ok()
        })
    }

    pub fn attach(&self) -> Result<()> {
        Ok(())
    }

    pub fn detach(&self) -> Result<()> {
        if let Some(mapped) = self.mapped {
            #[cfg(target_os = "linux")]
            unsafe {
                libc::munmap(mapped as *mut std::ffi::c_void, self.size);
            }
        }
        self.fd.as_ref().map(|_| ());
        Ok(())
    }
}

unsafe impl Send for SharedMemoryRing {}
unsafe impl Sync for SharedMemoryRing {}

impl Drop for SharedMemoryRing {
    fn drop(&mut self) {
        self.detach().ok();
    }
}
