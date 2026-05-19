# Installed Web App

**Runtime wrapper for web applications installed from the App Store.**

Provides an isolated webview container for running web applications as first-class desktop windows.

## Features

- **WebView rendering**: Runs installed web apps in a sandboxed iframe
- **Loading state**: Shows a spinner with the app title while the page loads
- **Timeout handling**: 20-second load timeout with error display and retry option
- **Retry mechanism**: Try-again button for failed or timed-out loads
- **Responsive**: Adapts to window resize within the desktop windowing system
- **Cross-origin detection**: Gracefully handles cross-origin iframe restrictions

## Interface

```
┌─────────────────────────────────────────────┐
│ App Name                        [⟳] [✕]    │
│ Subtitle: App Store                         │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────────────────────────────┐    │
│  │                                     │    │
│  │      [Web App Content in iframe]    │    │
│  │                                     │    │
│  │  (or loading spinner / error state) │    │
│  └─────────────────────────────────────┘    │
│                                             │
└─────────────────────────────────────────────┘
```

## Window Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `launchUrl` | string | URL to load in the webview |
| `title` | string | App display name |
| `subtitle` | string | Source label (default "App Store") |
| `source` | string | Installation source identifier |

## States

| State | Display |
|-------|---------|
| Loading | Spinner with "Loading {title}…" message |
| Loaded | Web app content visible in iframe |
| Timed out | Error message with 20s timeout notice and retry button |
| Failed | Error message with retry button |

## Integration

- Launched by the App Store when opening an installed web app
- Window parameters passed via `osStore` window state
- Title displayed in the window chrome/title bar
