use crate::core::error::{Result, TesseractError};


#[derive(Debug, Clone, Default)]
pub struct ProcessMetrics {
    pub total: u32,
    pub running: u32,
    pub processes: Vec<ProcessInfo>,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: String,
    pub cpu_percent: f64,
    pub memory_kb: u64,
}

impl ProcessMetrics {
    pub fn collect() -> Result<Self> {
        let mut total = 0u32;
        let mut running = 0u32;
        let mut processes = Vec::new();

        let proc_dir = std::fs::read_dir("/proc")
            .map_err(|e| TesseractError::System(format!("read /proc: {e}")))?;

        for entry in proc_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let pid_str = entry.file_name().to_string_lossy().to_string();
            let pid: u32 = match pid_str.parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let stat_path = entry.path().join("stat");
            let status_path = entry.path().join("status");

            if let Some(info) = read_process_info(pid, &stat_path, &status_path) {
                total += 1;
                if info.state == "R" {
                    running += 1;
                }
                if processes.len() < 256 {
                    processes.push(info);
                }
            }
        }

        Ok(Self {
            total,
            running,
            processes,
        })
    }

    pub fn find_by_pid(&self, pid: u32) -> Option<&ProcessInfo> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&ProcessInfo> {
        self.processes.iter().filter(|p| p.name.contains(name)).collect()
    }
}

fn read_process_info(pid: u32, stat_path: &std::path::PathBuf, status_path: &std::path::PathBuf) -> Option<ProcessInfo> {
    let _name = if let Ok(content) = std::fs::read_to_string(status_path) {
        for line in content.lines() {
            if line.starts_with("Name:") {
                return None;
            }
        }
        String::new()
    } else {
        return None;
    };

    let state = if let Ok(content) = std::fs::read_to_string(stat_path) {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            parts[2].trim_matches('(').trim_matches(')').to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    Some(ProcessInfo {
        pid,
        name: name_for_pid(pid),
        state,
        cpu_percent: 0.0,
        memory_kb: 0,
    })
}

fn name_for_pid(pid: u32) -> String {
    let path = format!("/proc/{pid}/comm");
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}
