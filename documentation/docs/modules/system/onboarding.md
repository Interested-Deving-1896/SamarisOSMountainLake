# Onboarding

First-boot welcome flow that guides users through initial setup of the Samaris desktop environment.

<br>

## Steps

1. **Welcome** — Greeting and OS introduction with language selection
2. **Theme Selection** — Light or dark mode preference with live preview
3. **WiFi Setup** — Scan and connect to a wireless network
4. **Tour** — Quick overview of desktop, AirBar, Finder, Peregrine, and Orbit AI
5. **Complete** — Transition to the desktop with `onboardingComplete` flag

<br>

## Architecture

```
OnboardingFlow (React)
├── OnboardingStep (step container with slide transitions)
├── WelcomeStep
│   └── LanguageSelector
├── ThemeStep
│   └── ThemePreview (live light/dark toggle)
├── WiFiStep
│   └── NetworkScanner
├── TourStep
│   └── TooltipOverlay (highlights UI elements)
└── CompleteStep
    └── LaunchButton
```

<br>

## State Management

Steps persist progress to `osStore` so the user can navigate forward/backward without losing input. On completion, `osStore.onboardingComplete` is set to `true`, which prevents the flow from showing on subsequent boots. The onboarding state is stored in `/User/.config/samaris/onboarding.json`.

<br>

## Related

- [Settings App](../../apps/settings.md)
- [Theme System](theme-system.md)
- [Network App](../../apps/network.md)
- [First Boot Guide](../../guides/first-boot.md)
- [VOLT Architecture — Onboarding Module](../../architecture/volt-onboarding.md)

<br>

---

[← Back: Documentation Index](../../index.md)
