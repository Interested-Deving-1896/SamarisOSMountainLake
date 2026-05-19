# Driver Support

Firmware and drivers for a wide range of wireless and graphics hardware, included in the ISO.

<br>

## WiFi

| Vendor | Firmware | Chipset Examples |
|--------|----------|-----------------|
| **Intel** | `iwlwifi` | AX200, AX210, AX211, AC 9260, AC 3165 |
| **Broadcom** | `brcmfmac` | BCM4360, BCM4352, BCM43143 |
| **Realtek** | `rtlwifi` | RTL8821CE, RTL8822BE, RTL8723DE |
| **MediaTek** | `mt76` | MT7921, MT7922, MT7915 |
| **Qualcomm/Atheros** | `ath10k`, `ath11k` | QCA6174, QCA9377, WCN685x |

<br>

## Bluetooth

`bluez` + `blueman` — supports most USB and PCI Bluetooth adapters.

<br>

## GPU

| Vendor | Driver | Notes |
|--------|--------|-------|
| **Intel** | `i915` | Integrated graphics |
| **AMD** | `amdgpu` | Radeon RX 6000+ series |
| **NVIDIA** | `nouveau` / `nvidia` | Open-source + proprietary |
| **VirtIO** | `virtio-gpu` | VM guest support |

<br>

## WWAN / Modem

`ModemManager` + `mobile-broadband-provider-info` for cellular modems.

<br>

---

[← Back: Documentation Index](../index.md)
