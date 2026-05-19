use crate::core::error::VrmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SbpOpcode {
    RamStatus = 0x15,
    RamFlush = 0x16,
    RamGcSignal = 0x17,
    RamRegisterApp = 0x18,
    RamUnregisterApp = 0x19,
    RamSetQuota = 0x1A,
    RamAppStatus = 0x1B,
    RamPressureEvent = 0x1C,
    RamCompressApp = 0x1D,
    RamReleaseCache = 0x1E,
    RamHeartbeat = 0x1F,
    RamSubscribeEvents = 0x20,
    RamUnsubscribeEvents = 0x21,
    RamPolicyUpdate = 0x22,
    RamSnapshot = 0x23,
}

impl SbpOpcode {
    pub fn from_byte(b: u8) -> Result<Self, VrmError> {
        match b {
            0x15 => Ok(Self::RamStatus),
            0x16 => Ok(Self::RamFlush),
            0x17 => Ok(Self::RamGcSignal),
            0x18 => Ok(Self::RamRegisterApp),
            0x19 => Ok(Self::RamUnregisterApp),
            0x1A => Ok(Self::RamSetQuota),
            0x1B => Ok(Self::RamAppStatus),
            0x1C => Ok(Self::RamPressureEvent),
            0x1D => Ok(Self::RamCompressApp),
            0x1E => Ok(Self::RamReleaseCache),
            0x1F => Ok(Self::RamHeartbeat),
            0x20 => Ok(Self::RamSubscribeEvents),
            0x21 => Ok(Self::RamUnsubscribeEvents),
            0x22 => Ok(Self::RamPolicyUpdate),
            0x23 => Ok(Self::RamSnapshot),
            _ => Err(VrmError::UnsupportedOpcode(b)),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::RamStatus => "RAM_STATUS",
            Self::RamFlush => "RAM_FLUSH",
            Self::RamGcSignal => "RAM_GC_SIGNAL",
            Self::RamRegisterApp => "RAM_REGISTER_APP",
            Self::RamUnregisterApp => "RAM_UNREGISTER_APP",
            Self::RamSetQuota => "RAM_SET_QUOTA",
            Self::RamAppStatus => "RAM_APP_STATUS",
            Self::RamPressureEvent => "RAM_PRESSURE_EVENT",
            Self::RamCompressApp => "RAM_COMPRESS_APP",
            Self::RamReleaseCache => "RAM_RELEASE_CACHE",
            Self::RamHeartbeat => "RAM_HEARTBEAT",
            Self::RamSubscribeEvents => "RAM_SUBSCRIBE_EVENTS",
            Self::RamUnsubscribeEvents => "RAM_UNSUBSCRIBE_EVENTS",
            Self::RamPolicyUpdate => "RAM_POLICY_UPDATE",
            Self::RamSnapshot => "RAM_SNAPSHOT",
        }
    }
}
