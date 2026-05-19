# Volt USB Manager — SBP-USB Protocol

## Overview

SBP-USB (Samaris Bus Protocol over USB) is a binary protocol for communicating with the
Volt USB Manager. It is used by system services to query status, perform management
operations, and receive event notifications.

## Header Format

The fixed header is **36 bytes**:

```
Offset  Size  Field           Description
────────────────────────────────────────────────────
0       4     Magic           0x5553424D ("USBM")
4       1     Version         1
5       1     Opcode          Operation code (0x30-0x3F)
6       2     Flags           Bitfield (MessageFlags)
8       8     Request ID      Unique request identifier
16      8     Timestamp (µs)  Unix timestamp in microseconds
24      8     App ID          Application or session identifier
32      4     Payload Length  Length of payload in bytes
────────────────────────────────────────────────────
36      N     Payload         Variable-length payload
36+N    4     Checksum        CRC32 of header + payload
```

## Opcodes (16 total)

| Code | Name               | Permission       | Requires Response |
|------|--------------------|------------------|-------------------|
| 0x30 | UsbStatus          | CAP_READ_STATUS  | Yes               |
| 0x31 | UsbRead            | CAP_READ_FILE    | Yes               |
| 0x32 | UsbWrite           | CAP_WRITE_FILE   | Yes               |
| 0x33 | UsbFlush           | CAP_ADMIN_STORAGE| Yes               |
| 0x34 | UsbCacheStatus     | CAP_READ_STATUS  | Yes               |
| 0x35 | UsbPrefetch        | CAP_READ_FILE    | Yes               |
| 0x36 | UsbEject           | CAP_ADMIN_STORAGE| Yes               |
| 0x37 | UsbHeartbeat       | CAP_READ_STATUS  | No                |
| 0x38 | UsbMount           | CAP_ADMIN_STORAGE| Yes               |
| 0x39 | UsbUnmount         | CAP_ADMIN_STORAGE| Yes               |
| 0x3A | UsbJournalStatus   | CAP_ADMIN_STORAGE| Yes               |
| 0x3B | UsbRecoveryRun     | CAP_ADMIN_STORAGE| Yes               |
| 0x3C | UsbDurabilityStatus| CAP_ADMIN_STORAGE| Yes               |
| 0x3D | UsbWriteAckEvent   | INTERNAL         | No                |
| 0x3E | UsbDeviceEvent     | INTERNAL         | No                |
| 0x3F | UsbMetricsSnapshot | CAP_READ_STATUS  | Yes               |

## Permissions

| Permission       | Bit   | Description                    |
|------------------|-------|--------------------------------|
| CAP_READ_STATUS  | 1<<0  | Read status and metrics        |
| CAP_READ_FILE    | 1<<1  | Read file contents             |
| CAP_WRITE_FILE   | 1<<2  | Write file contents            |
| CAP_ADMIN_STORAGE| 1<<3  | Administrative operations      |
| INTERNAL         | 1<<31 | Internal system use only       |

## Flags (6 values)

| Flag         | Bit   | Description                          |
|--------------|-------|--------------------------------------|
| REQUEST      | 0x01  | Message is a request                 |
| RESPONSE     | 0x02  | Message is a response                |
| ERROR        | 0x04  | Response indicates an error          |
| EVENT        | 0x08  | Message is an event notification     |
| ACK_BUFFERED | 0x10  | Write has been buffered (not durable)|
| ACK_DURABLE  | 0x20  | Write is durable (fsynced)           |

## Payloads

### UsbStatus (0x30) Response Payload
JSON-encoded `MetricsSnapshot` (see PERFORMANCE.md for field list).

### UsbFlush (0x33) Request
Empty payload.

### UsbWriteAckEvent (0x3D)
Sent asynchronously. Contains:
- `ACK_BUFFERED` flag when write enters the buffer
- `ACK_DURABLE` flag when write is fsynced to device
- `request_id` matches the original write request ID

### UsbDeviceEvent (0x3E)
Payload contains device path string.

## Error Codes

Errors are returned as `RESPONSE | ERROR` flags with a human-readable error message
in the payload. Error variants include:

| Error Variant              | Description                         |
|----------------------------|-------------------------------------|
| InvalidSbpMessage          | Malformed or corrupted message      |
| UnsupportedOpcode          | Opcode byte is not 0x30-0x3F        |
| ChecksumMismatch           | Payload CRC32 does not match        |
| PermissionDenied           | Caller lacks required permission    |
| DeviceNotFound             | USB device not detected             |
| FileNotFound               | Requested file does not exist       |
| JournalCorrupt             | Journal record is corrupted         |
| RecoveryRequired           | Journal recovery must run first     |

## Example Message (Hex Dump)

```
Request: UsbStatus
0000  55 53 42 4D 01 30 01 00  00 00 00 00 00 00 00 00  USBM.0..........
0010  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  ................
0020  00 00 00 00 00 00 00 00  A1 B2 C3 D4              ............

Response: UsbStatus (success)
0000  55 53 42 4D 01 30 02 00  00 00 00 00 00 00 00 00  USBM.0..........
0010  ...                                                  ...
```
