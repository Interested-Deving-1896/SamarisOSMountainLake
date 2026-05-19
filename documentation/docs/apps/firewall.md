# Firewall

**Inbound and outbound access rule management for Samaris OS.**

Controls network access rules with a simple on/off toggle and custom rule management.

## Features

- **Master toggle**: Enable or disable the firewall system-wide
- **System status indicator**: Shows whether the underlying system firewall is active
- **Default policies**: Independent inbound and outbound default policies (allow/deny)
- **Custom rules**: Add rules with label, port, direction, and action
- **Rule management**: Remove existing rules from the list
- **Visual state**: Colour-coded policy buttons reflect current state

## Rule Format

Rules are added via a prompt modal using the format:

```
label:port:direction:action
```

Example: `Web:8080:inbound:allow`

| Field | Values |
|-------|--------|
| label | Friendly name (e.g. "Web", "SSH") |
| port | Port number (e.g. 8080, 22) |
| direction | `inbound` or `outbound` |
| action | `allow` or `deny` |

## Interface

```
┌─────────────────────────────────────────┐
│ Firewall                    [Enabled]   │
│ Control inbound/outbound access rules   │
│ System firewall: Active                 │
│                                         │
│ [Inbound: allow] [Outbound: allow] [+Add]│
│                                         │
│ Web · inbound · port 8080 · allow  [x] │
│ SSH · inbound · port 22 · allow    [x]  │
└─────────────────────────────────────────┘
```

## Integration

Communicates with Kernel A's firewall service via `firewallKernel` client. Supports list/enable/disable/addRule/removeRule/setPolicy operations.
