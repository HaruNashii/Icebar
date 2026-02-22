# ❄️ Icebar

**Icebar** is a lightweight, **Wayland status bar** written in **Rust**, powered by **iced** and **iced-layer-shell**.

It aims to provide a **minimal, fast, and hackable bar/popup system** designed for wlroots compositors (Hyprland, Sway, River, etc.), with dynamic tray integration and interactive popups.

> Built for people who want full control instead of heavyweight desktop panels.

---

## ✨ Features

- 🧊 Native Wayland layer-shell bar
- ⚡ Written entirely in Rust
- 🎨 UI powered by `iced`
- 🖱 Interactive popups
- 🔔 Status Notifier / system tray support (DBusMenu)
- 📍 Cursor-aware popup positioning
- 🧩 Modular architecture (easy to extend)
- 🪶 Lightweight and compositor-friendly

---

## 🧠 What Icebar Does

Icebar is **not** a traditional desktop environment panel.

Instead, it acts as a:

- Wayland **layer-shell surface**
- Tray watcher (StatusNotifierItem)
- Popup/menu renderer
- Event-driven UI application

### Core responsibilities

Icebar:

- listens for tray applications via DBus
- renders tray items inside a bar
- opens contextual menus as popups
- communicates directly with Wayland outputs
- dynamically positions UI relative to cursor/output

Conceptually:
```
Tray Apps ──DBus──▶ Icebar Core
│
▼
iced UI Renderer
│
▼
Wayland Layer Surface
```

---

## 🖥 Supported Environments

Icebar targets **wlroots-based compositors**, including:

- Hyprland
- Sway
- River
- Wayfire
- Other layer-shell compatible compositors

X11 is **not supported**.

---

## 📦 Tech Stack

- Rust
- iced (GUI framework)
- iced_layershell
- zbus (DBus communication)
- Wayland layer-shell protocol

`iced` provides a declarative UI model inspired by Elm architecture.

---

## 🚀 Installation

### Requirements

- Rust (stable/2024)
- Wayland session
- wlroots compositor
- DBus session running
- libxrandr
- libxcb


**Install Rust:**

```bash
curl https://sh.rustup.rs -sSf | sh
```

**Build:**

```bash
git clone https://github.com/HaruNashii/Icebar
cd Icebar
cargo build --release
```

**Run with:**
```bash 
./target/release/icebar
```

Run it from your compositor autostart for best results.

**Example (Hyprland):**
```
exec-once = icebar
```

---

## 🧩 Architecture Overview
```
src/
├── main.rs        → application entry point
├── tray.rs        → StatusNotifier watcher
├── popup.rs       → popup UI + logic
├── modules/       → bar components
```

### Key Systems
**1. Layer Shell Integration**
- Creates anchored Wayland surfaces without a desktop environment.

**2. Tray Watcher**
- Uses DBus to detect and interact with tray applications.

**3. Popup Engine**
- Menus are rendered as independent iced views that can:
  - follow cursor position
  - anchor to outputs
  - react to clicks outside the window

**4. Event Model**
- Icebar follows iced's update/view architecture:
  - Message → Update → State → View
 
---
  
## 🎯 Project Goals
**Icebar focuses on:**
- simplicity over feature bloat
- hackability
- learning modern Wayland APIs
- experimenting with iced + layer-shell
- This is intentionally closer to a framework for a bar than a finished desktop panel.

---

## ⚠️ Current Status

Experimental / Work in Progress
Expect:
- breaking changes
- incomplete modules
- rapid iteration

The project is primarily a learning and experimentation platform.

---

## 🛠 Roadmap (Planned Ideas)

- Animations
- Plugin/modules API For Custom Modules
- Better tray icon handling

---

## 🤝 Contributing

**Contributions are welcome!!!**
**Good areas to help:**

- Wayland handling
- iced widgets
- tray compatibility
- performance improvements
- compositor testing
- architecture improvements

**Steps:**
```
  fork → branch → commit → pull request
```

---

## 📸 Screenshots

(Wil add screenshots later)

---

## 📜 License

MIT License.
See [LICENSE](https://github.com/HaruNashii/Icebar/blob/main/LICENSE) for details.

---

## ❤️ Author

Created by HaruNashii.
Icebar exists to explore what a modern Wayland bar can look like when built with Rust and iced.

---
