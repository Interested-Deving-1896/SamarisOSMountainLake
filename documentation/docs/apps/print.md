# Print

**Printer management and job queue viewer.**

Manages network printers and monitors active print jobs on the system.

## Features

- **Printer list**: View all configured printers with name, status, and connection source
- **Add printer**: Add new network printers via URI (IPP, socket, etc.)
- **Remove printer**: Remove printers from configuration
- **Job queue**: View queued print jobs with job ID and summary

## Adding a Printer

Printers are added via a prompt modal using the format:

```
Name|URI|Protocol
```

Example: `Studio|ipp://printer.local/ipp/print|ipp`

| Field | Description |
|-------|-------------|
| Name | Friendly display name |
| URI | Printer connection URI |
| Protocol | Connection protocol (ipp, socket, lpd, etc.) |

## Interface

Two-panel layout:
- **Left panel** — Printers list with status indicators (idle, printing, paused)
- **Right panel** — Print job queue with job IDs and summaries

## Integration

Communicates with Kernel A's print service via `printKernel` client. Supports `list()`, `add()`, `remove()`, and job queue monitoring.
