# NetHack Rust + WASM + wgpu

> **🚀 Active Development** | Phase 4.4 Complete, Phase 5 Infrastructure In Place

NetHack 5.0 ported to **FFI-First** approach using Rust + wgpu + WebGPU. Reuses ~250k lines of stable C game logic while implementing modern 3D graphics for Desktop, WASM, and Unity.

---

## 🎯 Project Vision

- ✅ **Reuse Existing C Code** — Stable game logic, AI, dungeon generation
- 🎨 **Modern Graphics** — wgpu 3D rendering with multiple view modes
- 🌐 **Cross-Platform** — Desktop (Linux/Mac/Windows), WebAssembly, Unity Plugin
- 🎮 **Incremental Build** — FFI → Game Bridge → Graphics → Multi-Platform

### Target Platforms

| Platform | Status | Estimated Phase |
|----------|--------|-----------------|
| **Desktop** (Linux/Mac/Windows) | 🔄 **In Progress** (Phase 4) | Phase 4.5 |
| **WebAssembly** (Browser) | 📋 Planned | Phase 6 |
| **Unity** Native Plugin | 📋 Planned | Phase 7 |

---

## 📊 Implementation Progress

```
Phase 0: Workspace Setup                    ✅ DONE
Phase 1: FFI Bindings (nethack-sys)         ✅ DONE (139 C files linked)
Phase 2: Game Bridge                        ✅ DONE (Player state, Game logic)
Phase 3: C Globals & Game State             ✅ DONE (GameBridge, state mgmt)
Phase 4: Desktop Graphics Pipeline          🔄 IN PROGRESS
  4.1: wgpu Rendering                       ✅ DONE (GPU setup, shaders, render pass)
  4.2: Game State → Vertices                ✅ DONE (Player cube, dungeon floor)
  4.3: Camera Integration (5 views)         ✅ DONE (TopDown, Isometric, etc.)
  4.4: Input System                         ✅ DONE (Arrow keys → movement)
Phase 5: Monster & Item Rendering           🔄 IN PROGRESS
  5.0: Infrastructure                       ✅ DONE (FFI wrappers, renderer stubs)
  5.1: Enable Monster Rendering             📋 NEXT
Phase 6: WASM Build                         📋 Planned
Phase 7: Unity Plugin (cdylib)              📋 Planned
```

### Component Status

| Component | Status | Details |
|-----------|--------|---------|
| **nethack-sys** | ✅ Complete | FFI bindings, C wrapper functions |
| **nethack-core** | ✅ Complete | Camera (5 modes), Input, GameRenderer |
| **nethack-render** | ✅ Complete | wgpu pipeline, WGSL shaders |
| **nethack-desktop** | 🔄 Active | winit + wgpu, input handling, rendering |
| **Tests** | ✅ 17 Passing | camera, input, game_renderer, world |

---

## 🚀 Quick Start

### Build Requirements

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build tools (platform-specific)
# Linux:   sudo apt install build-essential
# macOS:   xcode-select --install
# Windows: Visual Studio Build Tools
```

### Build & Run

```bash
# Clone and enter project
git clone https://github.com/oosawak/NetHack.git
cd NetHack

# Build all crates
cargo build

# Run desktop app
cargo run -p nethack-desktop

# Run tests
cargo test -p nethack-core
```

### Controls

- **Arrow Keys** — Move player (↑↓←→)
- **1-5 Keys** — Switch camera view
  - 1 = TopDown
  - 2 = Isometric
  - 3 = FirstPerson
  - 4 = ThirdPerson
  - 5 = Cinematic
- **Q** — Quit

---

## 🏗️ Architecture

```
libnetHack.a (C library)
    ↓ FFI
nethack-sys (auto-generated bindings)
    ↓
nethack-core (Game layer: camera, input, rendering logic)
    ↓
nethack-render (Graphics layer: wgpu + WGSL)
    ↓
nethack-desktop (Desktop: winit event loop)
```

### Rendering Pipeline

1. **Input:** winit → KeyCode → Key → GameCommand
2. **Update:** execute_command() → player position
3. **Render:** GameRenderer.update_from_game_state()
4. **GPU:** WgpuRenderer.render() → wgpu RenderPass
5. **Frame:** Output to window

### Entity Types

- **Player:** Yellow cube at (ux, uy)
- **Dungeon:** Gray tiles (10×10 visible radius)
- **Monsters:** Red cubes (ready for C library integration)
- **Items:** Cyan cubes (infrastructure in place)

---

## 📁 Project Structure

```
crates/
├── nethack-sys/       # FFI layer (bindgen + C wrappers)
├── nethack-core/      # Game logic (camera, input, rendering)
├── nethack-render/    # Graphics (wgpu + WGSL shaders)
├── nethack-desktop/   # Desktop app (winit + event loop)
├── nethack-wasm/      # WASM target (planned)
└── nethack-unity/     # Unity plugin (planned)

docs/
├── ARCHITECTURE.md    # Detailed technical design
├── RENDERING_STRATEGY.md
└── FFI_DESIGN.md
```

---

## 🎮 Current Capabilities

✅ **Working:**
- wgpu rendering pipeline with proper GPU setup
- WGSL vertex/fragment shaders
- Player position tracking from C library
- 5 camera view modes with real-time switching
- Arrow key movement with boundary checking
- Game state updates each frame
- Proper window management with winit
- Input → GameCommand → execution flow

🔄 **In Progress:**
- Monster rendering from C library monsters
- Item placement and rendering
- Expanded dungeon features

📋 **Planned:**
- WASM target for browser play
- Unity native plugin
- More game entities (traps, doors, etc.)
- Sound and music
- Save/load game state
- UI overlays (inventory, status)

---

## 🔗 Key References

- **NetHack Sources:** `/home/oosawak/Workspace/NetHack/src/` (C files)
- **FFI Wrapper:** `crates/nethack-sys/wrapper.{c,h}`
- **Rendering:** `crates/nethack-render/src/renderer.rs`
- **Desktop:** `crates/nethack-desktop/src/main.rs`
- **Architecture:** See `ARCHITECTURE.md` for full technical details

---

## 🛠️ Development Notes

### Building the C Library

The build process automatically:
1. Compiles 139 C files to libnetHack.a
2. Generates FFI bindings via bindgen
3. Links to all Rust crates

No manual C build needed - `cargo build` handles everything.

### Adding Features

When adding new game features:
1. Create C wrapper function in `wrapper.c`
2. Update `wrapper.h` with declaration
3. Add to build.rs allowlist
4. Use safely in Rust code via FFI

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test -p nethack-core camera::tests::test_camera_switch

# Run with output
cargo test -- --nocapture
```

---

## 📝 Recent Changes

**Phase 4.4 - Input System:**
- Implemented InputManager for command queueing
- Arrow keys now move player
- GameCommand enum covers 13+ NetHack actions
- Input flow: KeyCode → Key → GameCommand → execute_command()

**Phase 5.0 - Monster Infrastructure:**
- Created C wrapper functions for monster enumeration
- Extended GameRenderer with rendering methods
- Prepared stubs for C library integration
- All infrastructure ready for monster rendering

---

## 💡 Notes

- All work on `master` branch with regular commits
- Tests must pass before committing
- Build output is clean (warnings are cosmetic)
- Desktop binary is ~160MB (includes all dependencies)

---

## 📄 License

NetHack is licensed under the NetHack General Public License.
Rust code contributions follow the same license.

---

**Want to contribute?** See `ARCHITECTURE.md` for detailed technical overview and next steps!
