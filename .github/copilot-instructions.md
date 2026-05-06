# NetHack Copilot Instructions

## Build System

NetHack uses a traditional Unix make system with a hints-based configuration. Makefiles are distributed from `sys/unix/` templates.

**Initial setup (one time):**
```sh
cd sys/unix
sh setup.sh hints/linux          # or hints/linux.500, hints/macOS.500, etc.
cd ../..
make fetch-Lua                   # download Lua submodule/library (one time)
```

**Build:**
```sh
make all                         # tty-only build (default)
make WANT_WIN_TTY=1 WANT_WIN_CURSES=1 all   # with curses
make WANT_WIN_ALL=1 all          # all available interfaces
```

**Install and clean:**
```sh
make install                     # installs to HACKDIR (destructive – wipes and recreates)
make spotless                    # remove all generated files before a clean rebuild
```

**Tests:**
Tests are Lua scripts in `test/`. They run inside a running game:
1. Build and install NetHack without DLB.
2. Copy test `.lua` files into the nethack playground directory.
3. Start nethack in wizard mode (`nethack -D`).
4. Use `#wizloadlua` extended command to load and run a test file (e.g., `test_src.lua`).

## Architecture

### Directory Layout

- `src/` — All main game C source (flat directory, ~100 `.c` files). Core game logic lives here.
- `include/` — All header files. `hack.h` is the master include that pulls in most others.
- `dat/` — Lua files defining dungeon levels, quests, special rooms, rumors, etc. Level layouts were migrated from the old yacc/lex compiler to Lua in 5.0.
- `sys/` — OS-specific platform code (`unix/`, `windows/`, `vms/`, etc.) and the `share/` utilities.
- `win/` — Window interface implementations: `tty/`, `curses/`, `X11/`, `Qt/`, `win32/`. Each implements the `struct window_procs` vtable.
- `util/` — Utility programs run at build time (e.g., `makedefs`).
- `sound/` — Sound interface code.
- `submodules/` — External dependencies (Lua).

### Window Port System

All UI backends implement the `struct window_procs` function-pointer table defined in `include/winprocs.h`. The active backend is selected at runtime. Multiple backends can be compiled into a single binary and switched via the `windowtype` option. New interfaces must implement every function pointer in `window_procs`.

### Lua Integration

Since 5.0, dungeon/level data (previously compiled by yacc/lex tools) is expressed in Lua. The files in `dat/*.lua` define special levels, quest levels, and rooms. The Lua API exposed to these scripts is implemented in `src/nhlua.c`, `src/nhlsel.c`, and `src/nhlobj.c`. The `dat/nhlib.lua` and `dat/nhcore.lua` files provide Lua-side utilities.

### Key Headers

- `include/hack.h` — Master include; include this instead of individual headers in `src/` files.
- `include/config.h` — Top-level compile-time configuration (OS, windowing systems, features).
- `include/winprocs.h` — Window port vtable (`struct window_procs`).
- `include/extern.h` — Declarations of all exported functions from `src/`.

## Code Conventions

### C Standard and Style

- Code is **C99**. The codebase is compiled with C99-compliant compilers.
- **4 spaces per indent, no tabs.** Max line width: **78 characters.**
- Function definitions: return type on its own line, then function name and args, then `{` on its own line:
  ```c
  void
  foo(int i, char c)
  {
      /* body */
  }
  ```
- Control statement opening brace on the same line as the statement:
  ```c
  if (condition) {
      /* body */
  } else {
      /* body */
  }
  ```
- `case` labels are **not** indented inside `switch`. Fall-throughs must be marked with `/* fall-through */`.
- Variables must **not** be declared in loop initializers or conditions. Assignments used as conditions go in an extra pair of parentheses: `if ((p = fcall()))`.

### `staticfn` Macro

Use `staticfn` instead of `static` for file-local functions in `src/*.c`. This macro expands to `static` on most platforms but can be overridden to expose function names in stack traces. **Never use `staticfn` on data.**

### Reserved/Avoided Names

- Do **not** use `near`, `far` as variable names (treated as keywords by some cross-compilers).
- Do **not** use `NEARDATA` (Amiga-specific macro still present in the tree).

### File Endings

Source files conventionally end with a comment containing the filename, e.g.:
```c
/*zap.c*/
```

### Comments

Two accepted block-comment styles:
```c
/* short undecorated comment */

/*
 * Longer comment with
 * sentence punctuation.
 */
```
Multi-line end-of-line comments must continue with `*` on each line to avoid clang-format mangling:
```c
somecode(); /* this comment
             * continues here */
```

### Cross-Compiling

The codebase supports building on one platform for another. See the top-level `Cross-compiling` file. Hints files in `sys/unix/hints/` encode platform-specific settings and are the preferred way to configure non-standard builds.
