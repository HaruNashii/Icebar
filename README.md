# ❄️ Icebar

**Icebar** is a lightweight, **Wayland status bar** written in **Rust**, powered by **iced** and **iced-layer-shell**.

It aims to provide a **minimal, fast, and hackable bar/popup system** designed for wlroots and smithay compositors (Hyprland, Sway, Niri, etc.), with dynamic tray integration and interactive popups.

> Built for people who want full control instead of heavyweight desktop panels.

---

## 🖼️ Showcase
![Icebar Showcase](https://github.com/user-attachments/assets/f79d8a53-81e5-408b-84b5-d79de6c0a66f)


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

Icebar targets **smithay and wlroots based compositors**, including:

- Hyprland
- Sway
- Niri 
- Others layer-shell compatible compositors

X11 is **not supported**.

---

## 📦 Tech Stack

- Rust
- iced (GUI framework)
- iced_layershell
- zbus (DBus communication)
- Wayland layer-shell protocol
- libpulse (Volume Data Fetcher) 
- wpctl (Volume Action handling) 

> ⚠️ **Audio stack note:** Icebar reads volume data via `libpulse` (PulseAudio) but controls volume via `wpctl` (PipeWire/WirePlumber). On systems running **pure PipeWire without the PulseAudio compatibility layer**, the volume display will not update even though scroll and mute controls work fine. To fix this, install `pipewire-pulse` (the package name may vary by distro), which provides the PulseAudio interface on top of PipeWire.


`iced` provides a declarative UI model inspired by Elm architecture.

---

## 🚀 Installation

#### **AUR (Recommended):**
```paru -S icebar-git``` 

or 

```yay -S icebar-git```

--

#### **Building From Source:**

Requirements for building:
- Rust/Cargo (stable/2024)
- libpulse (or "libpulse-dev")
- gcc-libs
- libxrandr
- libxcb

> ⚠️ **Runtime requirement:** `pipewire-pulse` must be installed for volume monitoring to work on PipeWire systems. See the audio stack note in the Tech Stack section above.

**Build And Install With:**
```bash
git clone https://github.com/HaruNashii/Icebar
cd Icebar
cargo build --release
mkdir -p $HOME/.local/bin
cp -rf target/release/icebar $HOME/.local/bin/
```


**Tip: Run it from your compositor autostart for best results.**

- Example (Hyprland):
```
exec-once = icebar
```

- Example (Sway):
```
bar {
    swaybar_command icebar
}
```

---

## 🎨 Theme Switcher

Icebar includes a bash script that lets you manage and apply themes from a collection of preset configurations.

### Folder Structure

Place your themes in a folder called `themes` next to the script. Each theme must be a subdirectory containing a `config.ron` file:

```
icebar-theme-switcher.sh
themes/
├── dracula/
│   └── config.ron
├── nord/
│   └── config.ron
└── catppuccin-mocha/
    └── config.ron
```

### Usage

```bash
chmod +x icebar-theme-switcher.sh
./icebar-theme-switcher.sh
```

The script will:

1. **Scan** `./themes/` and list every valid theme (subdirectories that contain a `config.ron`)
2. **Prompt you to pick** a theme by number, or press `q` to quit without making any changes
3. **Check for an existing config** at `~/.config/icebar/config.ron` and offer two options if one is found:
   - **Backup** — renames the existing config to `config.ron.backup_YYYYMMDD_HHMMSS` before installing the new one, so nothing is lost
   - **Overwrite** — replaces the current config permanently; requires a second confirmation (`yes`) before proceeding
4. **Copy** the chosen theme's `config.ron` to `~/.config/icebar/config.ron`

After the script finishes, restart Icebar to apply the new theme.

> **Note:** The script creates `~/.config/icebar/` automatically if it does not exist yet.

---

## 🧩 Architecture Overview
```
src/
├── main.rs             → application entry point
├── subscription.rs     → iced subscriptions
├── update.rs           → iced message handler
├── view.rs             → iced renderer handler
├── ron.rs              → ron configuration handler
├── tray.rs             → StatusNotifier watcher
├── context_menu.rs     → context menu UI + logic
├── modules/            → bar components
├── helpers/            → small helpers
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

## 🪲 Known Bugs

- Icebar crashing on Gnome.
  - Explanation: Icebar depend on [Layer Shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#compositor-support) which Gnome doesn't implemented yet!!!
- Context menu not closing when clicking outside the window on Niri
- Bar size not working on Niri

---

## 🛠 Roadmap (Planned Ideas)

- Plugin/modules API For Custom Modules
- Better tray icon handling
- Animations (very low priority right now)

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
(Accepting screenshots to display here, if you have one and want to share, i would appreciate it)

---

## 📜 License

MIT License.
See [LICENSE](https://github.com/HaruNashii/Icebar/blob/main/LICENSE) for details.

---
