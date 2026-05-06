# NetHack Rust + WASM + wgpu

> **🚀 Active Development** | Phase 6.2 Complete: WASM Browser Demo Ready

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
| **Desktop** (Linux/Mac/Windows) | 🔄 **In Progress** (Phase 5) | Phase 5.4 |
| **WebAssembly** (Browser) | ✅ **Build Complete** (Phase 6) | Phase 6.2 |
| **Unity** Native Plugin | 📋 Planned | Phase 7 |

---

## 📊 Implementation Progress

```
Phase 0: Workspace Setup                    ✅ DONE
Phase 1: FFI Bindings (nethack-sys)         ✅ DONE (139 C files linked)
Phase 2: Game Bridge                        ✅ DONE (Player state, Game logic)
Phase 3: C Globals & Game State             ✅ DONE (GameBridge, state mgmt)
Phase 4: Desktop Graphics Pipeline          ✅ DONE
  4.1: wgpu Rendering                       ✅ DONE (GPU setup, shaders, render pass)
  4.2: Game State → Vertices                ✅ DONE (Player cube, dungeon floor)
  4.3: Camera Integration (5 views)         ✅ DONE (TopDown, Isometric, etc.)
  4.4: Input System                         ✅ DONE (Arrow keys → movement)
Phase 5: Dungeon Entity Rendering           ✅ DONE
  5.0: Infrastructure                       ✅ DONE (FFI wrappers, renderer stubs)
  5.1: Fix Linker & Enable Monster Render   ✅ DONE (svl extern, static lib, wrappers)
  5.2: Item Rendering                       ✅ DONE (Cyan cubes, OBJ_FLOOR enumeration)
  5.3: Dungeon Features (Traps & Stairs)    ✅ DONE (Purple traps, green/blue stairs)
Phase 6: WASM Build & Browser Demo          ✅ DONE
  6.1: WASM Target Setup                    ✅ DONE (wasm-bindgen, wasm-pack build)
  6.2: Browser Example & Server             ✅ DONE (examples/wasm.html, run_server.sh)
  6.3: Local Testing Environment            ✅ DONE (Auto-detecting HTTP server)
Phase 7: Unity Plugin (cdylib)              📋 Planned
```

### Component Status

| Component | Status | Details |
|-----------|--------|---------|
| **nethack-sys** | ✅ Complete | FFI bindings, C wrapper functions, libnhmain integration |
| **nethack-core** | ✅ Complete | Camera (5 modes), Input, GameRenderer, Monster render enabled |
| **nethack-render** | ✅ Complete | wgpu pipeline, WGSL shaders |
| **nethack-desktop** | 🔄 Active | winit + wgpu, input handling, monster/item rendering |
| **nethack-wasm** | ✅ Complete | WASM build (28KB binary), wasm-bindgen API, no C library |
| **Tests** | ✅ 17 Passing | camera, input, game_renderer, world, all passing release/debug |

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

### Build for WebAssembly (WASM)

```bash
# Run setup script to install wasm-pack and related tools
./setup_wasm.sh

# Build WASM module
wasm-pack build crates/nethack-wasm --target web --release

# Output: crates/nethack-wasm/pkg/
#   - nethack_wasm.wasm (28KB binary)
#   - nethack_wasm.js (JavaScript bindings)
#   - nethack_wasm.d.ts (TypeScript types)
#   - package.json (npm package)
```

**Note:** The WASM build uses Rust-only game logic (no C library). It's ideal for quick browser testing and deployment.

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
Desktop (C library):
libnetHack.a (C library)
    ↓ FFI
nethack-sys (auto-generated bindings)
    ↓
nethack-core (Game layer: camera, input, rendering logic)
    ↓
nethack-render (Graphics layer: wgpu + WGSL)
    ↓
nethack-desktop (Desktop: winit event loop)

WASM (Rust-only):
nethack-core (Rust game logic - no C library)
    ↓
nethack-wasm (wasm-bindgen JavaScript API)
    ↓
Browser (WebAssembly binary + JavaScript)
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
- **Monsters:** Red (hostile) / Yellow (peaceful) cubes, auto-rendered from C library
- **Items:** Cyan cubes (infrastructure in place, stub implementation)

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
- Monster rendering from C library (red/yellow colored cubes)
- Monster enumeration via safe FFI wrapper (get_monster_count/by_index)
- Item rendering from C library (cyan colored cubes)
- Item enumeration via safe FFI wrapper (get_object_count/by_index, OBJ_FLOOR filter)
- Trap rendering from C library (purple cubes, tiny size)
- Trap enumeration via safe FFI wrapper (get_trap_count/by_index)
- Stairway rendering from C library (green=up, blue=down)
- Stairway enumeration via safe FFI wrapper (get_stair_count/by_index)

🔄 **In Progress:**
- Game turn cycle integration
- More dungeon features (doors, etc.)

📋 **Planned:**
- WASM target for browser play
- Unity native plugin
- More game entities (doors, etc.)
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

## 📝 Recent Changes

### Phase 6.2: Browser Example & Local Server (Latest - Current)
- ✅ Created `examples/wasm.html` with full-featured Canvas demo
  - Real-time vertex rendering from WASM game state
  - FPS counter and player position tracking
  - Camera mode switching buttons (1-5 keys support)
  - Responsive sidebar with game stats
- ✅ Implemented `examples/run_server.sh` auto-detecting HTTP server
  - Works with Python 3, Python 2, Node.js, or Ruby
  - Clear startup instructions with browser URL
  - Verifies WASM binary before starting
- ✅ Created `examples/verify.sh` for environment validation
  - Checks WASM binary presence and size
  - Validates JavaScript bindings
  - Confirms all example files are in place
- ✅ Created `examples/README.md` with user guide
  - Quick start instructions
  - Control mapping
  - Camera mode descriptions
  - Troubleshooting guide

**Ready to test:** Users can now run `./examples/run_server.sh` and open `http://localhost:8000/examples/wasm.html` in their browser!

### Phase 6: WASM Build Complete
- ✅ Setup wasm32-unknown-unknown target with wasm-pack
- ✅ Created nethack-wasm crate with wasm-bindgen API
- ✅ Conditional compilation: WASM uses Rust-only logic (no C library)
- ✅ Implemented Game struct: init(), player_x/y, move_player(), render()
- ✅ Vertex rendering: render() returns flat f32 array for JavaScript
- ✅ Successfully built WASM binary (28KB) with wasm-pack
- ℹ️  Note: C library not compiled for WASM - pure Rust game logic only

### Phase 5.3: Dungeon Features (Traps & Stairs)
- ✅ Implemented trap enumeration from C library (gf.ftrap list)
- ✅ Added safe FFI wrappers: `get_trap_count()`, `get_trap_by_index()`
- ✅ Implemented stairway enumeration (gs.stairs list)
- ✅ Added safe FFI wrappers: `get_stair_count()`, `get_stair_by_index()`
- ✅ Traps render as tiny (0.15 size) purple cubes
- ✅ Stairs render with direction distinction: green (up), blue (down)
- ✅ Fixed FFI header conflicts with NetHack includes
- ✅ All 17 tests passing, release binary 14MB

### Phase 5.2: Item Rendering
- ✅ Implemented item enumeration from C library
- ✅ Added safe FFI wrappers: `get_object_count()`, `get_object_by_index()`
- ✅ Items render as cyan cubes (distinct from monsters and player)
- ✅ Proper OBJ_FLOOR filtering to show only dungeon floor items

### Phase 5.1: Monster Rendering & Linker Fix
- Fixed critical linker error by declaring `extern struct instance_globals_saved_l svl`
- Resolved missing symbol issues via static library archive (libnetHack.a)
- Implemented monster enumeration and rendering (red=hostile, yellow=peaceful)

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

**Phase 5.1 - Fix Linker Error & Enable Monster Rendering:**
- Fixed undefined symbol `svl` linker error by declaring `extern struct instance_globals_saved_l svl`
- Modified wrapper.c to directly access `svl.level.monlist` in get_monster_count/by_index
- Implemented static library archive creation in build.rs for proper object file linking
- Compiled sys/libnh/libnhmain.c with LIBNH flags to resolve symbol conflicts
- Added wrapper functions for chdirx and whoami platform compatibility
- All 17 tests passing in both debug and release modes
- Release binary compiles to 14MB executable

**Phase 5.0 - Monster Infrastructure:**
- Created C wrapper functions for monster enumeration
- Extended GameRenderer with rendering methods
- Prepared stubs for C library integration
- Monster rendering code already enabled in game_renderer.rs

---

## 💡 Notes

- All work on `master` branch with regular commits
- Tests must pass before committing
- Build output is clean (warnings are cosmetic)
- Release binary is 14MB, Debug binary is 160MB (includes all dependencies)
- Monster rendering is enabled - all monsters in game will be rendered as colored cubes

---

## 📄 License

NetHack is licensed under the NetHack General Public License.
Rust code contributions follow the same license.

---

**Want to contribute?** See `ARCHITECTURE.md` for detailed technical overview and next steps!
