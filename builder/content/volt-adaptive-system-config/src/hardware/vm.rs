pub fn probe() -> bool {
    if let Ok(val) = std::env::var("SAMARIS_VM") {
        return val == "1"
            || val.eq_ignore_ascii_case("true")
            || val.eq_ignore_ascii_case("yes");
    }

    #[cfg(target_os = "linux")]
    {
        if is_vm_linux() {
            return true;
        }
    }

    false
}

#[cfg(target_os = "linux")]
fn is_vm_linux() -> bool {
    let hypervisor_strings = [
        "virtualbox",
        "vmware",
        "kvm",
        "qemu",
        "xen",
        "hyper-v",
        "microsoft",
        "bochs",
        "parallels",
    ];

    let paths = [
        "/sys/class/dmi/id/product_name",
        "/sys/class/dmi/id/sys_vendor",
        "/sys/class/dmi/id/product_version",
    ];

    for path_str in &paths {
        let path = std::path::Path::new(path_str);
        if let Ok(content) = std::fs::read_to_string(path) {
            let lower = content.trim().to_lowercase();
            if hypervisor_strings.iter().any(|&h| lower.contains(h)) {
                return true;
            }
        }
    }

    false
}
