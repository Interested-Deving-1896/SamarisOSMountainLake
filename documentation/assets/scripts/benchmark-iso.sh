#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# Samaris OS — ISO Full Performance Benchmark
# =============================================================================
# Usage: ./benchmark-iso.sh [path/to/samaris.iso]
#
# Launches the ISO under QEMU (direct kernel boot + serial console),
# auto-logs in, runs 30+ diagnostic commands, and generates a report.
#
# Requires: qemu-system-x86_64, xorriso, python3
# =============================================================================

ISO="${1:-./Samaris-OS-Alpha-One-RC - x86-64 FULL.iso}"
OUTDIR="${2:-./benchmark-output}"
TIMESTAMP=$(date +%Y%m%d-%H%M%S)
SESSION_DIR="/tmp/bench-$TIMESTAMP"
SERIAL_SOCK="/tmp/benchsock"
MONITOR_SOCK="/tmp/benchmon"
SERIAL_LOG="$SESSION_DIR/serial-raw.log"
QEMU_DEBUG_LOG="$SESSION_DIR/qemu-debug.log"
DIAG_OUT="$SESSION_DIR/diag-output.txt"
MONITOR_OUT="$SESSION_DIR/monitor-info.txt"
REPORT="$OUTDIR/iso-boot-qemu.md"
ISO_SIZE_REPORT="$OUTDIR/iso-size.md"
RAM_MB="${RAM_MB:-8192}"
SMP_CORES="${SMP_CORES:-4}"
TIMEOUT_SEC="${TIMEOUT_SEC:-300}"

mkdir -p "$SESSION_DIR" "$OUTDIR"
rm -f "$SERIAL_SOCK" "$MONITOR_SOCK"

if [ ! -f "$ISO" ]; then
  echo "FATAL: ISO not found at $ISO"
  echo "Usage: $0 [path/to/samaris.iso] [output-dir]"
  exit 1
fi

ISO_SIZE=$(stat -f%z "$ISO" 2>/dev/null || stat --format=%s "$ISO" 2>/dev/null)
ISO_SIZE_HUMAN=$(echo "scale=1; $ISO_SIZE/1073741824" | bc)
ISO_SHA=$(shasum -a 256 "$ISO" | cut -d' ' -f1 2>/dev/null || sha256sum "$ISO" | cut -d' ' -f1)
QEMU_BIN=$(command -v qemu-system-x86_64)

echo "═══════════════════════════════════════════════════════════════════"
echo " Samaris OS — ISO Full Performance Benchmark"
echo "═══════════════════════════════════════════════════════════════════"
echo " ISO: $(basename "$ISO") (${ISO_SIZE_HUMAN}GB)"
echo " SHA256: ${ISO_SHA:0:32}..."
echo " QEMU: $($QEMU_BIN --version | head -1)"
echo " Config: ${RAM_MB}MB RAM, ${SMP_CORES} vCPUs"
echo "═══════════════════════════════════════════════════════════════════"

echo ""
echo "[1/5] Extracting kernel+initrd from ISO..."
xorriso -osirrox on -indev "$ISO" \
  -extract /live/x86_64/vmlinuz "$SESSION_DIR/vmlinuz" \
  -extract /live/x86_64/initrd.img "$SESSION_DIR/initrd.img" 2>/dev/null
echo "      vmlinuz: $(stat -f%z "$SESSION_DIR/vmlinuz" 2>/dev/null || stat --format=%s "$SESSION_DIR/vmlinuz") bytes"
echo "      initrd:  $(stat -f%z "$SESSION_DIR/initrd.img" 2>/dev/null || stat --format=%s "$SESSION_DIR/initrd.img") bytes"

echo ""
echo "[2/5] Launching QEMU..."
$QEMU_BIN \
  -machine q35 \
  -kernel "$SESSION_DIR/vmlinuz" \
  -initrd "$SESSION_DIR/initrd.img" \
  -append "boot=live components live-media-path=/live/x86_64 console=ttyS0,115200n8 loglevel=7 systemd.show_status=1 plymouth.enable=0" \
  -cdrom "$ISO" \
  -m "$RAM_MB" -smp "$SMP_CORES" \
  -D "$QEMU_DEBUG_LOG" -d guest_errors,unimp,cpu_reset \
  -chardev "socket,path=$SERIAL_SOCK,server=on,wait=off,id=ser0" \
  -serial "chardev:ser0" \
  -monitor "unix:$MONITOR_SOCK,server,nowait" \
  -display none 2>/dev/null &
QEMU_PID=$!
echo "      PID: $QEMU_PID"

echo ""
echo "[3/5] Auto-boot + system diagnostics..."

python3 << PYEOF
import socket, os, time, sys, re

SERIAL_SOCK = "/tmp/benchsock"
SESSION_DIR = "$SESSION_DIR"
MONITOR_SOCK = "/tmp/benchmon"
BOOT_TIMEOUT = $TIMEOUT_SEC

def connect_socket(path, retries=120):
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    for _ in range(retries):
        try:
            sock.connect(path)
            return sock
        except (ConnectionRefusedError, FileNotFoundError):
            time.sleep(0.5)
    return None

print("      Connecting to serial...", flush=True)
sock = connect_socket(SERIAL_SOCK)
if not sock:
    print("FATAL: serial connection failed")
    sys.exit(1)

sock.settimeout(0.5)
print("      Connected.", flush=True)

buffer = b""
login_found = False
start = time.time()

while time.time() - start < BOOT_TIMEOUT:
    try:
        data = sock.recv(4096)
        if data:
            buffer += data
            sys.stdout.buffer.write(data)
            sys.stdout.buffer.flush()
            with open(f"{SESSION_DIR}/serial-raw.log", "ab") as f:
                f.write(data)
    except socket.timeout:
        pass
    if b"login:" in buffer:
        login_found = True
        boot_time = time.time() - start
        print(f"\n >>> LOGIN at +{boot_time:.0f}s <<<", flush=True)
        break

if not login_found:
    print(f"\n TIMEOUT: No login after {BOOT_TIMEOUT}s", flush=True)
    sys.exit(1)

time.sleep(1)
sock.sendall(b"user\n")
time.sleep(1)
try:
    data = sock.recv(4096)
    buffer += data
except: pass

if b"assword:" in buffer:
    sock.sendall(b"user\n")
    time.sleep(1)
    try:
        data = sock.recv(4096)
        buffer += data
    except: pass
    with open(f"{SESSION_DIR}/serial-raw.log", "ab") as f:
        f.write(data)

# Wait for shell prompt
shell_start = time.time()
while time.time() - shell_start < 20:
    try:
        data = sock.recv(4096)
        if data:
            buffer += data
            with open(f"{SESSION_DIR}/serial-raw.log", "ab") as f:
                f.write(data)
    except socket.timeout: pass
    if re.search(rb'[\$#]\s*$', buffer[-200:]):
        print(" >>> SHELL READY", flush=True)
        break

commands = [
    ("uname -a", 2), ("uptime", 2),
    ("systemd-analyze time", 3),
    ("systemd-analyze blame | head -30", 5),
    ("systemd-analyze critical-chain | head -20", 3),
    ("systemctl list-units --state=failed 2>/dev/null || true", 3),
    ("systemctl is-system-running 2>/dev/null || echo unknown", 2),
    ("journalctl -p err -b --no-pager 2>/dev/null | tail -30 || true", 3),
    ("free -h", 2), ("cat /proc/meminfo", 3),
    ("cat /proc/cpuinfo | grep -E 'model name|cpu MHz|processor' | head -20", 2),
    ("df -h", 2), ("lsblk 2>/dev/null || cat /proc/partitions", 2),
    ("cat /proc/mounts | grep -E 'squashfs|overlay|live' | head -10", 2),
    ("ip addr 2>/dev/null || ifconfig", 2),
    ("cat /proc/net/dev", 2),
    ("ps aux --sort=-%mem | head -20", 3),
    ("ps aux --sort=-%cpu | head -20", 3),
    ("ls -la /opt/volt/", 2),
    ("ls -la /opt/volt/bin/ 2>/dev/null || echo no bin", 2),
    ("systemctl list-units --type=service --state=running --no-pager 2>/dev/null | head -40", 3),
    ("systemctl status volt-* --no-pager 2>/dev/null | head -40", 3),
    ("cat /proc/cmdline", 2),
    ("cat /sys/devices/system/clocksource/clocksource0/current_clocksource 2>/dev/null || echo N/A", 1),
    ("lsmod | head -30", 2),
    ("cat /run/samaris/adaptive.generated.toml 2>/dev/null | head -30 || echo no asc", 2),
]

for cmd, wait in commands:
    sock.sendall((cmd + "\n").encode())
    time.sleep(wait)
    try:
        data = sock.recv(8192)
        with open(f"{SESSION_DIR}/serial-raw.log", "ab") as f:
            f.write(data)
        sys.stdout.buffer.write(data)
        sys.stdout.buffer.flush()
    except: pass

# QEMU monitor
try:
    msock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    msock.settimeout(3)
    msock.connect(MONITOR_SOCK)
    time.sleep(0.3)
    mon_data = ""
    for cmd in ['info status', 'info cpus', 'info block', 'info network', 'info usernet']:
        msock.sendall((cmd + '\n').encode())
        time.sleep(0.3)
        try:
            d = msock.recv(4096)
            mon_data += f"# {cmd}\n{d.decode(errors='replace')}\n\n"
        except: pass
    msock.close()
    with open(f"{SESSION_DIR}/monitor-info.txt", "w") as f:
        f.write(mon_data)
except: pass

# Shutdown
sock.sendall(b"sudo poweroff 2>/dev/null || poweroff 2>/dev/null || shutdown -h now\n")
time.sleep(3)
try:
    msock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    msock.settimeout(3)
    msock.connect(MONITOR_SOCK)
    msock.sendall(b'system_powerdown\n')
    time.sleep(2)
    msock.sendall(b'quit\n')
    msock.close()
except: pass
sock.close()
print("\n >>> Session complete.", flush=True)
PYEOF

# Kill QEMU
if kill -0 "$QEMU_PID" 2>/dev/null; then
  python3 -c "
import socket, time
try:
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.settimeout(3)
    sock.connect('/tmp/benchmon')
    sock.sendall(b'system_powerdown\n')
    time.sleep(2)
    sock.sendall(b'quit\n')
    sock.close()
except: pass
" 2>/dev/null
  sleep 2
  kill -0 "$QEMU_PID" 2>/dev/null && kill -9 "$QEMU_PID" 2>/dev/null || true
fi

echo ""
echo "[4/5] Extracting metrics..."

SERIAL_LINES=$(wc -l < "$SERIAL_LOG" 2>/dev/null || echo 0)
BOOT_TIMING=$(grep -oP 'Startup finished in.*' "$SERIAL_LOG" | head -1 || echo "N/A")
KERNEL_TIME=$(grep -oP 'Startup finished in \K[0-9.]+' "$SERIAL_LOG" | head -1 || echo "N/A")
USERSPACE_TIME=$(grep -oP '\(kernel\) \+ \K[0-9.]+' "$SERIAL_LOG" | head -1 || echo "N/A")
TOTAL_TIME=$(grep -oP '= \K[0-9.]+' "$SERIAL_LOG" | head -1 || echo "N/A")
GRAPHICAL_TARGET=$(grep -oP 'graphical.target reached after \K[0-9.]+' "$SERIAL_LOG" | head -1 || echo "N/A")
PANICS=$(grep -ci "kernel panic\|Kernel Panic" "$SERIAL_LOG" || echo 0)
FAILED_UNITS=$(grep -ci "FAILED\|failed to start" "$SERIAL_LOG" || echo 0)

echo ""
echo "[5/5] Generating report..."

# Clean ANSI escapes from serial log for cleaner report
cat > "$REPORT" << ENDREPORT
# ISO Boot QEMU Benchmark

**ISO:** $(basename "$ISO") (${ISO_SIZE_HUMAN}GB)  
**SHA256:** \`${ISO_SHA}\`  
**Date:** $(date -u +%Y-%m-%dT%H:%M:%SZ)  
**QEMU:** $($QEMU_BIN --version | head -1)  
**Host:** $(uname -a)  

## Configuration

| Parameter | Value |
|-----------|-------|
| VM Engine | QEMU x86_64 (q35 machine) |
| RAM | ${RAM_MB} MB |
| vCPUs | ${SMP_CORES} |
| Boot method | Direct kernel (+initrd) via QEMU |
| Serial console | ttyS0, 115200 baud |

## Boot Performance

| Metric | Value |
|--------|-------|
| Kernel boot | ${KERNEL_TIME}s |
| Userspace | ${USERSPACE_TIME}s |
| **Total** | **${TOTAL_TIME}s** |
| graphical.target | ${GRAPHICAL_TARGET}s (userspace) |

## Health

| Metric | Value |
|--------|-------|
| Serial log lines | ${SERIAL_LINES} |
| Kernel panics | ${PANICS} |
| Failed units | ${FAILED_UNITS} |

## Files

| File | Description |
|------|-------------|
| serial-raw.log | Full serial console output |
| qemu-debug.log | QEMU debug trace |
| diag-output.txt | Raw diagnostic output |
| monitor-info.txt | QEMU monitor state |
ENDREPORT

# Copy artifacts
cp "$SERIAL_LOG" "$OUTDIR/serial-raw.log" 2>/dev/null || true
cp "$QEMU_DEBUG_LOG" "$OUTDIR/qemu-debug.log" 2>/dev/null || true
[ -f "$MONITOR_OUT" ] && cp "$MONITOR_OUT" "$OUTDIR/monitor-info.txt" 2>/dev/null || true
[ -f "$DIAG_OUT" ] && cp "$DIAG_OUT" "$OUTDIR/diag-output.txt" 2>/dev/null || true

# Clean up temp session
rm -rf "$SESSION_DIR"
rm -f "$SERIAL_SOCK" "$MONITOR_SOCK"

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo " BENCHMARK COMPLETE"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "  Report:  $REPORT"
echo "  ISO:     $ISO"
echo "  Boot:    ${TOTAL_TIME}s (kernel ${KERNEL_TIME}s + userspace ${USERSPACE_TIME}s)"
echo "  Panics:  ${PANICS}"
echo "  Failed:  ${FAILED_UNITS}"
echo ""
echo "  To run again: $0 <iso-path> [output-dir]"
echo ""
