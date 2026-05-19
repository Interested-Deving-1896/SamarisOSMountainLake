# Volt USB Manager — Recovery Procedure for System Administrators

## Overview

This document provides a step-by-step recovery procedure for system administrators
when the Volt USB Manager journal is dirty or corrupted.

## When Recovery Is Needed

Recovery is required when:
- System crashes or loses power while USB device is mounted
- USB device is removed without clean eject
- `volt-usb-manager --status` shows `Recovery Required` or `Journal Dirty: true`
- System logs contain `Journal is dirty` warnings

## Step-by-Step Procedure

### Step 1: Assess the Situation

```bash
# Check current status
volt-usb-manager --status

# Check if journal is dirty
volt-usb-manager --recover --config /etc/volt/usb-manager.toml
```

Sample output:
```
Loading config from: /etc/volt/usb-manager.toml
Config is valid:
  mount_point: /mnt/volt_usb
  backing_path: /var/volt/usb_backing
  Journal opened, dirty: true
```

### Step 2: Inspect the Journal

```bash
# Find the journal directory (from config)
ls -la /var/volt/journal/
# Expected: wal.dat, possibly wal.dat.checkpoint
```

### Step 3: Simulate Recovery First

Always simulate before running actual recovery:

```bash
volt-usb-manager --simulate-recovery
```

This creates a temporary directory and runs recovery on simulated data.
It does NOT touch the real device.

### Step 4: Run Actual Recovery

```bash
# Ensure the USB device is connected and mounted read-only
# Run recovery with the config file
volt-usb-manager --recover --config /etc/volt/usb-manager.toml
```

Expected output:
```
Recovering journal at: /var/volt/journal
Journal opened, dirty: true
Replayed 42 records, applied 38 writes
```

If recovery fails with a corruption error, proceed to Step 5.

### Step 5: Handle Corruption

If recovery reports corruption:

```bash
# 1. Check dmesg for hardware errors
dmesg | grep -i "usb\|disk\|i/o error"

# 2. Back up the journal and backing store
cp -r /var/volt/journal /var/volt/journal.backup.$(date +%Y%m%d)
cp -r /var/volt/usb_backing /var/volt/usb_backing.backup.$(date +%Y%m%d)

# 3. Force clear the journal (LAST RESORT - data loss possible)
rm -f /var/volt/journal/wal.dat /var/volt/journal/wal.dat.checkpoint
echo "Journal manually cleared on $(date)" >> /var/volt/journal/MANUAL_CLEAR_LOG

# 4. Mount in read-only mode
volt-usb-manager --mount --config /etc/volt/usb-manager.toml
# The system will detect the clean journal and mount without recovery
```

### Step 6: Mount After Recovery

```bash
# Mount normally — recovery runs automatically if needed
volt-usb-manager --mount --config /etc/volt/usb-manager.toml

# Verify mount
volt-usb-manager --status
# State should be "Running"
```

### Step 7: Verify Data Integrity

```bash
# Check mounted filesystem
ls -la /mnt/volt_usb/

# Compare with expected file listing
# If files are missing, check the recovery log
```

## Recovery Failure Scenarios

### Scenario A: Corrupt WAL Record

**Symptom**: `JournalChecksumFailed` error during recovery.

**Action**: The system automatically falls back to read-only mode if
`device.read_only_fallback` is true (default). All data up to the corrupt
record is recoverable. Data after the corrupt point is lost.

### Scenario B: Device Not Found

**Symptom**: `DeviceNotFound` error.

**Action**: Reconnect the USB device and retry. If device is permanently
unavailable, data in the journal cannot be replayed.

### Scenario C: Backing Store Full

**Symptom**: `WriteFailed` during recovery — disk full.

**Action**: Free space on the backing store, then re-run recovery.

## Prevention

1. Always perform **clean eject** before disconnecting USB device:
   ```bash
   volt-usb-manager --eject
   ```
2. Monitor journal dirty state via metrics
3. Configure `journal.fsync_on_record = true` for maximum safety
4. Regular checkpointing reduces replay time on crash

## Emergency Commands

```bash
# Quick status check
volt-usb-manager --status

# Force unmount (if mount is hung)
volt-usb-manager --unmount --config /etc/volt/usb-manager.toml

# Force eject (bypasses safety checks)
# WARNING: May lose data
volt-usb-manager --eject --config /etc/volt/usb-manager.toml

# Clear journal (emergency only — data loss)
rm -rf /var/volt/journal/*
```
