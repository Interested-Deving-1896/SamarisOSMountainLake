# Encryption

**LUKS user partition control and recovery surface.**

Manages disk encryption for the user data partition. Provides controls for passphrase changes, recovery phrase backup, and integrity checks.

## Features

- **Status display**: Shows whether the user partition is encrypted or not
- **Platform info**: Encryption backend platform identifier
- **Change passphrase**: Update the LUKS encryption passphrase
- **Backup recovery phrase**: Export or display the recovery/backup phrase for safe keeping
- **Integrity check**: Run a verification scan on the encrypted partition

## Actions

| Action | Description |
|--------|-------------|
| Change passphrase | Securely update the LUKS passphrase for the user partition |
| Backup recovery phrase | Export the recovery phrase for emergency access |
| Integrity check | Scan the encrypted volume for data integrity issues |

## Interface

Compact card-based UI with status indicators and action buttons. Each action returns a note with result information.

## Integration

Communicates with Kernel A's encryption service via `encryptionKernel` client. Supports status queries, passphrase operations, and integrity verification.
