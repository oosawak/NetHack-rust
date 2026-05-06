use std::path::PathBuf;
use std::env;
use glob::glob;

fn main() {
    let nethack_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("NetHack");

    if !nethack_root.exists() {
        panic!(
            "NetHack source not found at {}",
            nethack_root.display()
        );
    }

    let src_dir = nethack_root.join("src");
    let include_dir = nethack_root.join("include");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Recompile if wrapper.h changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Generate FFI bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("wrapper.h").to_string_lossy())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg(format!("-I{}", src_dir.display()))
        // Game initialization functions
        .allowlist_function("early_init")
        .allowlist_function("choose_windows")
        .allowlist_function("initoptions")
        .allowlist_function("init_nhwindows")
        .allowlist_function("dlb_init")
        .allowlist_function("vision_init")
        .allowlist_function("init_sound_disp_gamewindows")
        .allowlist_function("newgame")
        .allowlist_function("moveloop")
        .allowlist_function("docommand")
        .allowlist_function("getlock")
        .allowlist_function("player_selection")
        // Variables
        .allowlist_var("dlevel")
        .allowlist_var("dunlevs")
        .allowlist_var("u")
        .allowlist_var("dungeon")
        .allowlist_var("fobj")
        .allowlist_var("fmon")
        .allowlist_var("windowprocs")
        // Types
        .allowlist_type("you")
        .allowlist_type("monst")
        .allowlist_type("obj")
        .allowlist_type("coord")
        .allowlist_type("dungeon_topology")
        .allowlist_type("window_procs")
        // Generate safe bindings where possible
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ffi.rs"))
        .expect("Couldn't write bindings!");

    // Link all NetHack object files except main entry points
    let obj_pattern = src_dir.join("*.o");
    let exclude_files = vec!["unixmain.o", "winmain.o", "macmain.o"];
    let mut obj_count = 0;
    
    for obj_file in glob(&obj_pattern.to_string_lossy()).expect("Failed to read glob pattern") {
        if let Ok(path) = obj_file {
            let filename = path.file_name().unwrap().to_string_lossy();
            if !exclude_files.contains(&filename.as_ref()) {
                println!("cargo:rustc-link-arg={}", path.display());
                obj_count += 1;
            }
        }
    }

    // Link system libraries that NetHack depends on
    println!("cargo:rustc-link-lib=m");        // math
    println!("cargo:rustc-link-lib=lua5.4");   // lua
    println!("cargo:rustc-link-lib=ncurses");  // terminal control

    println!("cargo:warning=Phase 2: FFI バインディング生成完了");
    println!("cargo:warning=Linked {} NetHack object files (excluded: main entries)", obj_count);
    println!("cargo:warning=Exposed {} initialization functions", 10);
    println!("cargo:warning=Generated FFI: {}", out_path.join("ffi.rs").display());
}
