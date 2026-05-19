pub fn owner_can_read(mode: u32) -> bool {
    (mode & 0o400) != 0
}

pub fn owner_can_write(mode: u32) -> bool {
    (mode & 0o200) != 0
}

pub fn owner_can_exec(mode: u32) -> bool {
    (mode & 0o100) != 0
}

pub fn group_can_read(mode: u32) -> bool {
    (mode & 0o040) != 0
}

pub fn group_can_write(mode: u32) -> bool {
    (mode & 0o020) != 0
}

pub fn group_can_exec(mode: u32) -> bool {
    (mode & 0o010) != 0
}

pub fn other_can_read(mode: u32) -> bool {
    (mode & 0o004) != 0
}

pub fn other_can_write(mode: u32) -> bool {
    (mode & 0o002) != 0
}

pub fn other_can_exec(mode: u32) -> bool {
    (mode & 0o001) != 0
}

pub fn can_access(
    mode: u32,
    uid: u32,
    gid: u32,
    target_uid: u32,
    target_gid: u32,
    need_write: bool,
) -> bool {
    if need_write {
        if target_uid == uid {
            owner_can_write(mode)
        } else if target_gid == gid {
            group_can_write(mode)
        } else {
            other_can_write(mode)
        }
    } else {
        if target_uid == uid {
            owner_can_read(mode)
        } else if target_gid == gid {
            group_can_read(mode)
        } else {
            other_can_read(mode)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_read_write_exec() {
        let mode = 0o700;
        assert!(owner_can_read(mode));
        assert!(owner_can_write(mode));
        assert!(owner_can_exec(mode));
        assert!(!group_can_read(mode));
        assert!(!other_can_read(mode));
    }

    #[test]
    fn test_group_permissions() {
        let mode = 0o070;
        assert!(!owner_can_read(mode));
        assert!(group_can_read(mode));
        assert!(group_can_write(mode));
        assert!(group_can_exec(mode));
        assert!(!other_can_read(mode));
    }

    #[test]
    fn test_other_permissions() {
        let mode = 0o007;
        assert!(!owner_can_read(mode));
        assert!(!group_can_read(mode));
        assert!(other_can_read(mode));
        assert!(other_can_write(mode));
        assert!(other_can_exec(mode));
    }

    #[test]
    fn test_can_access_write() {
        let mode = 0o755;
        assert!(can_access(mode, 100, 100, 100, 200, true)); // owner
        assert!(!can_access(mode, 100, 100, 200, 200, true)); // other can't write on 755
        let mode2 = 0o777;
        assert!(can_access(mode2, 100, 100, 200, 200, true));
    }

    #[test]
    fn test_can_access_read() {
        let mode = 0o444;
        assert!(can_access(mode, 100, 100, 200, 200, false));
        assert!(!can_access(mode, 100, 100, 200, 200, true));
    }
}
