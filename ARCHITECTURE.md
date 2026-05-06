# NetHack Rust/wgpu/WASM Architecture

## Overview

**FFI-First Approach:** Reuse existing NetHack C library (stable, tested) for game logic. Implement graphics layer in Rust using wgpu, supporting Desktop, WASM, and Unity targets.

## Project Structure

```
crates/
├── nethack-sys/        # FFI bindings to C library (auto-generated bindgen)
│   ├── build.rs        # Compiles C code, runs bindgen
│   ├── wrapper.c       # Safe C wrapper functions for FFI
│   ├── wrapper.h       # Public FFI interface
│   └── src/lib.rs      # Exports bindgen output
│
├── nethack-core/       # Rust game logic (gradual C replacement)
│   ├── camera.rs       # 3D camera with 5 view modes
│   ├── game_renderer.rs # Game state → vertices conversion
│   ├── game_bridge.rs  # High-level game interface
│   ├── input.rs        # Keyboard input → game commands
│   ├── state.rs        # Game state (player, items, etc.)
│   ├── world.rs        # 3D spatial world
│   └── dungeon.rs      # Dungeon generation/exploration
│
├── nethack-render/     # wgpu rendering pipeline
│   ├── renderer.rs     # WgpuRenderer (GPU setup, render passes)
│   ├── shaders.rs      # WGSL shaders (vertex, fragment)
│   └── vertex.rs       # Vertex structure for GPU
│
├── nethack-desktop/    # Desktop application (winit + wgpu)
│   └── main.rs         # Application loop, window management
│
├── nethack-wasm/       # WASM target (future)
│   └── lib.rs          # Browser binding, WebGPU setup
│
└── nethack-unity/      # Unity plugin (future)
    └── lib.rs          # cdylib with C-compatible API
```

## Rendering Pipeline

```
NetHack C Library (libnetHack.a)
  ├── Game state (player position, dungeon layout, monsters, items)
  └── Game logic (turns, AI, combat, etc.)
           ↓
nethack-sys (FFI layer)
  ├── get_player_x/y
  ├── get_monster_count/by_index
  └── get_object_count/by_index
           ↓
nethack-core (Game layer)
  ├── GameRenderer: convert game state → vertices
  │   └── Calls wrapper C functions to read monsters/items
  ├── Camera3D: player tracking, view switching (1-5 keys)
  │   └── 5 modes: TopDown, Isometric, FirstPerson, ThirdPerson, Cinematic
  └── InputManager: keyboard → GameCommand
           ↓
nethack-render (Graphics layer)
  ├── WgpuRenderer: GPU setup
  │   ├── Instance → Adapter → Device
  │   ├── Surface + RenderPipeline
  │   └── Buffer management
  └── WGSL Shaders
           ↓
Desktop/WASM/Unity Output
```

## Game Architecture

### Entity Representation

**Player:**
- Yellow cube (8 vertices)
- Position tracked from C library via get_player_x/y
- Camera follows player position

**Monsters:**
- Red cube (hostile) or yellow cube (peaceful)
- Enumerated via get_monster_count/by_index
- Rendered at (mx, my) positions from C struct monst

**Dungeon Floor:**
- Gray tile quads (4 vertices each)
- 10x10 visible radius around player
- Simple height = 0.0

**Items:**
- Cyan cube (future)
- Currently stubbed (get_object_count returns 0)
- Will be implemented via level object enumeration

### Input Flow

```
KeyCode (winit) → Key (abstraction) → GameCommand → execute_command()
                                           ↓
                                    Player position update
                                           ↓
                                    GameRenderer update_from_game_state()
                                           ↓
                                    WgpuRenderer render()
```

### Camera System

**5 View Modes (hotkeys 1-5):**
1. **TopDown** (1): Looking straight down
   - Eye: (player_x, 30, player_z)
   - Target: (player_x, 0, player_z)

2. **Isometric** (2): 45° angle, elevated
   - Eye: (player_x + 10, 20, player_z + 10)
   - Target: (player_x, 0, player_z)

3. **FirstPerson** (3): Player eye level, looking forward
   - Eye: (player_x, 1.7, player_z)
   - Target: (player_x + cos(θ), 1.7, player_z + sin(θ))

4. **ThirdPerson** (4): Behind and above player
   - Eye: (player_x, 5, player_z - 8)
   - Target: (player_x, 2, player_z)

5. **Cinematic** (5): Dramatic elevated angle
   - Eye: (player_x + 15, 25, player_z + 15)
   - Target: (player_x, 5, player_z)

## Completed Phases

### ✅ Phase 0: Workspace Setup
- Cargo workspace with 6 crates
- Project structure established

### ✅ Phase 1: FFI Layer (nethack-sys)
- C code compiled to libnetHack.a
- 139 object files linked
- bindgen generates FFI bindings
- Safe wrapper functions in wrapper.c

### ✅ Phase 2: Rendering (nethack-render)
- WgpuRenderer with complete pipeline
- WGSL shaders (vertex/fragment)
- Proper wgpu API (DeviceExt, compilation_options)
- Surface management, RenderPipeline setup

### ✅ Phase 3: Game Bridge
- GameBridge struct connects FFI to Rust
- Safe wrapper around C globals (u.ux, u.uy)
- Player state tracking

### ✅ Phase 4: Desktop App
**4.1 - wgpu Integration:**
- winit event loop + wgpu surface
- Render loop with proper window management
- Frame timing and diagnostics

**4.2 - Game State Rendering:**
- GameRenderer converts player/dungeon to vertices
- Player: yellow cube at (ux, uy)
- Dungeon: gray tiles in 10x10 visible area
- Dynamic vertex buffer updates

**4.3 - Camera Integration:**
- Camera3D with 5 view modes
- View switching via 1-5 keys
- Camera follows player position
- Aspect ratio calculation per frame
- View-projection matrix to GPU

**4.4 - Input System:**
- InputManager command queueing
- Arrow keys → player movement
- Boundary checking (80×24 dungeon)
- Command-based action execution

### ✅ Phase 5: Monster/Item Infrastructure
- wrapper.c functions for monster enumeration
- get_monster_count() and get_monster_by_index()
- GameRenderer methods for creature rendering
- add_creature_cube() helper for small entities
- Object enumeration stubs (deferred)

## Next Steps

### Phase 5.1: Enable Real Monster Rendering
- Uncomment C FFI calls in add_monsters_from_c()
- Test with actual C library monsters
- Verify positioning (mx, my coordinates)
- Debug rendering if needed

### Phase 6: WASM Build
- Target: wasm32-unknown-unknown
- wgpu WebGPU backend
- wasm-bindgen for JS interface
- Handle C library limitations (fork, signals)

### Phase 7: Unity Plugin (cdylib)
- Target: x86_64-pc-windows-msvc, aarch64-apple-darwin, etc.
- #[no_mangle] pub extern "C" functions
- C# DllImport bindings

## Key Technical Decisions

1. **FFI-First:** Keep C library for stability, wrap carefully
2. **Safe Wrappers:** wrapper.c hides complex C structs from Rust
3. **GPU-Accelerated:** wgpu for graphics, instancing for performance
4. **Portable:** Same code for Desktop/WASM/Unity via modular design
5. **Incremental:** Phase by phase, test at each step

## Testing

**Unit Tests:** 17 passing (camera, input, game_renderer, world)

```bash
cargo test -p nethack-core
```

**Integration:** Desktop app builds and runs headless
- Frame logging every 5 seconds
- Input handling tested manually
- Camera switching responsive

## Performance Notes

- **Frame Time:** Target 60 FPS
- **Vertex Budget:** ~500 typical (player + floor tiles + monsters)
- **Culling:** Frustum culling on roadmap
- **Instancing:** GPU instancing for repeated tiles (future)

## Known Limitations

1. **Object Enumeration:** Currently stubbed (get_object_count = 0)
2. **C Integration:** Monster/object rendering on per-game-loop timing
3. **Dungeon Size:** Hardcoded 80×24, should read from C
4. **Save/Load:** Not yet integrated
5. **Audio:** Disabled for now

## File References

- **C Library:** `/home/oosawak/Workspace/NetHack/src/*.c`
- **Headers:** `/home/oosawak/Workspace/NetHack/include/*.h`
- **Wrapper:** `crates/nethack-sys/wrapper.{c,h}`
- **Rendering:** `crates/nethack-render/src/renderer.rs`
- **Desktop:** `crates/nethack-desktop/src/main.rs`
