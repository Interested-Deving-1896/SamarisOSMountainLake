use crate::core::result::VgmResult;
use crate::sbp_gpu::message::SbpGpuMessage;
use crate::sbp_gpu::opcode::SbpGpuOpcode;

pub fn serialize(opcode: SbpGpuOpcode, payload: Vec<u8>) -> VgmResult<SbpGpuMessage> {
    let msg = SbpGpuMessage::new(opcode, payload);
    Ok(msg)
}

pub fn serialize_status(
    device_name: &str,
    driver_version: &str,
    temp_c: Option<f64>,
    vram_total_mb: u64,
    vram_used_mb: u64,
) -> VgmResult<SbpGpuMessage> {
    let mut payload = Vec::new();
    let name_bytes = device_name.as_bytes();
    let ver_bytes = driver_version.as_bytes();
    payload.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
    payload.extend_from_slice(name_bytes);
    payload.extend_from_slice(&(ver_bytes.len() as u32).to_le_bytes());
    payload.extend_from_slice(ver_bytes);
    payload.push(if temp_c.is_some() { 1 } else { 0 });
    if let Some(t) = temp_c {
        payload.extend_from_slice(&t.to_le_bytes());
    }
    payload.extend_from_slice(&vram_total_mb.to_le_bytes());
    payload.extend_from_slice(&vram_used_mb.to_le_bytes());
    Ok(SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, payload))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_gpu::message::SBP_GPU_HEADER_SIZE;

    #[test]
    fn test_serialize_basic() {
        let msg = serialize(SbpGpuOpcode::GpuRenderFrame, vec![0xFF]).unwrap();
        assert_eq!(msg.opcode, SbpGpuOpcode::GpuRenderFrame);
        assert_eq!(msg.payload, vec![0xFF]);
    }

    #[test]
    fn test_serialize_status_message() {
        let msg = serialize_status("TestGPU", "1.0", Some(45.0), 8192, 2048).unwrap();
        assert_eq!(msg.opcode, SbpGpuOpcode::GpuStatus);
        assert!(msg.payload.len() > 8);
        let bytes = msg.to_bytes();
        assert!(bytes.len() >= SBP_GPU_HEADER_SIZE);
    }

    #[test]
    fn test_serialize_status_no_temp() {
        let msg = serialize_status("iGPU", "0.5", None, 1024, 512).unwrap();
        assert_eq!(msg.opcode, SbpGpuOpcode::GpuStatus);
        assert!(msg.payload.len() > 8);
    }

    #[test]
    fn test_serialize_different_opcodes() {
        for op in &[
            SbpGpuOpcode::GpuStatus,
            SbpGpuOpcode::GpuExecCompute,
            SbpGpuOpcode::GpuThermalStatus,
        ] {
            let msg = serialize(*op, vec![]).unwrap();
            assert_eq!(msg.opcode, *op);
            let bytes = msg.to_bytes();
            let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
            assert_eq!(decoded.opcode, *op);
        }
    }

    #[test]
    fn test_status_payload_format() {
        let msg = serialize_status("VulkanGPU", "3.2.1", Some(60.0), 16384, 4096).unwrap();
        let bytes = msg.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, SbpGpuOpcode::GpuStatus);
        let payload = &decoded.payload;
        assert!(payload.len() >= 25);
        let name_len = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
        assert_eq!(&payload[4..4 + name_len], b"VulkanGPU");
    }
}
