use crate::core::error::TesseractError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Opcode {
    GpuRender = 0x01,
    GpuCompute = 0x02,
    CpuReserve = 0x03,
    CpuRelease = 0x04,
    CpuExec = 0x05,
    MemAlloc = 0x06,
    MemFree = 0x07,
    StreamVideo = 0x08,
    StreamAudio = 0x09,
    QueryCores = 0x0A,
    QueryGpu = 0x0B,
    Heartbeat = 0x0C,
    ThermalStatus = 0x0F,
    ContextCreate = 0x30,
    ContextShare = 0x31,
}

impl Opcode {
    pub fn from_byte(b: u8) -> Result<Self, TesseractError> {
        match b {
            0x01 => Ok(Self::GpuRender),
            0x02 => Ok(Self::GpuCompute),
            0x03 => Ok(Self::CpuReserve),
            0x04 => Ok(Self::CpuRelease),
            0x05 => Ok(Self::CpuExec),
            0x06 => Ok(Self::MemAlloc),
            0x07 => Ok(Self::MemFree),
            0x08 => Ok(Self::StreamVideo),
            0x09 => Ok(Self::StreamAudio),
            0x0A => Ok(Self::QueryCores),
            0x0B => Ok(Self::QueryGpu),
            0x0C => Ok(Self::Heartbeat),
            0x0F => Ok(Self::ThermalStatus),
            0x30 => Ok(Self::ContextCreate),
            0x31 => Ok(Self::ContextShare),
            _ => Err(TesseractError::Protocol(format!("unknown opcode: 0x{b:02X}"))),
        }
    }

    pub fn is_query(self) -> bool {
        matches!(self, Self::QueryCores | Self::QueryGpu | Self::Heartbeat | Self::ThermalStatus)
    }

    pub fn is_gpu(self) -> bool {
        matches!(self, Self::GpuRender | Self::GpuCompute)
    }

    pub fn is_compute(self) -> bool {
        matches!(self, Self::CpuReserve | Self::CpuRelease | Self::CpuExec)
    }

    pub fn is_memory(self) -> bool {
        matches!(self, Self::MemAlloc | Self::MemFree)
    }

    pub fn is_stream(self) -> bool {
        matches!(self, Self::StreamVideo | Self::StreamAudio)
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::GpuRender => "GPU_RENDER",
            Self::GpuCompute => "GPU_COMPUTE",
            Self::CpuReserve => "CPU_RESERVE",
            Self::CpuRelease => "CPU_RELEASE",
            Self::CpuExec => "CPU_EXEC",
            Self::MemAlloc => "MEM_ALLOC",
            Self::MemFree => "MEM_FREE",
            Self::StreamVideo => "STREAM_VIDEO",
            Self::StreamAudio => "STREAM_AUDIO",
            Self::QueryCores => "QUERY_CORES",
            Self::QueryGpu => "QUERY_GPU",
            Self::Heartbeat => "HEARTBEAT",
            Self::ThermalStatus => "THERMAL_STATUS",
            Self::ContextCreate => "CONTEXT_CREATE",
            Self::ContextShare => "CONTEXT_SHARE",
        }
    }
}
