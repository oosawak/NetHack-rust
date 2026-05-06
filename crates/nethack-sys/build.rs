use std::path::PathBuf;
use std::env;
use std::process::Command;
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
    let sys_dir = nethack_root.join("sys");
    let include_dir = nethack_root.join("include");
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Recompile if wrapper.h or wrapper.c changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=wrapper.c");

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
        // Player state accessor functions
        .allowlist_function("get_player_x")
        .allowlist_function("get_player_y")
        .allowlist_function("get_player_level")
        .allowlist_function("get_player_hp")
        .allowlist_function("get_player_maxhp")
        .allowlist_function("get_player_state")
        .allowlist_function("get_dlevel")
        .allowlist_function("get_dunlevs")
        // Monster and object accessors
        .allowlist_function("get_monster_count")
        .allowlist_function("get_monster_by_index")
        .allowlist_function("get_object_count")
        .allowlist_function("get_object_by_index")
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
        .allowlist_type("monster_data_t")
        .allowlist_type("object_data_t")
        .allowlist_type("player_state_t")
        // Generate safe bindings where possible
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("ffi.rs"))
        .expect("Couldn't write bindings!");

    // Compile wrapper.c (accessor functions)
    let wrapper_src = manifest_dir.join("wrapper.c");
    if wrapper_src.exists() {
        cc::Build::new()
            .file(&wrapper_src)
            .include(&include_dir)
            .include(&src_dir)
            .compile("wrapper");
    }

    // Compile essential sys files that provide library interface
    let sys_files = vec![
        "sys/libnh/libnhmain.c",  // Provides check_user_string, player_selection, etc.
    ];
    
    for file in &sys_files {
        let src_path = nethack_root.join(file);
        if src_path.exists() {
            cc::Build::new()
                .file(&src_path)
                .include(&include_dir)
                .include(&src_dir)
                .include(sys_dir.join("libnh"))
                .flag("-DSHIM_GRAPHICS")
                .flag("-DNOTTYGRAPHICS")
                .flag("-DNOSHELL")
                .flag("-DLIBNH")
                .compile(&format!("sys_{}", PathBuf::from(file).file_stem().unwrap().to_string_lossy()));
        }
    }

    // Create static archive from all pre-compiled NetHack object files
    let obj_pattern = src_dir.join("*.o");
    let exclude_files = vec!["unixmain.o", "winmain.o", "macmain.o"];
    let mut obj_files = Vec::new();
    let mut obj_count = 0;
    
    for obj_file in glob(&obj_pattern.to_string_lossy()).expect("Failed to read glob pattern") {
        if let Ok(path) = obj_file {
            let filename = path.file_name().unwrap().to_string_lossy();
            if !exclude_files.contains(&filename.as_ref()) {
                obj_files.push(path);
                obj_count += 1;
            }
        }
    }

    // Create static library archive from object files
    if !obj_files.is_empty() {
        let lib_path = out_dir.join("libnetHack.a");
        
        // Remove existing archive
        let _ = std::fs::remove_file(&lib_path);
        
        // Create archive with 'ar'
        let mut ar_cmd = Command::new("ar");
        ar_cmd.arg("rcs").arg(&lib_path);
        
        for obj_file in &obj_files {
            ar_cmd.arg(obj_file);
        }
        
        let output = ar_cmd.output().expect("Failed to run 'ar' command");
        if !output.status.success() {
            panic!(
                "Failed to create static library: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        
        // Link the static library
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=netHack");
    }

    // Link system libraries that NetHack depends on
    println!("cargo:rustc-link-lib=m");        // math
    println!("cargo:rustc-link-lib=lua5.4");   // lua
    println!("cargo:rustc-link-lib=ncurses");  // terminal control

    println!("cargo:warning=Phase 5.1: FFI エラー修正 (svl linker issue resolved)");
    println!("cargo:warning=Linked {} NetHack object files (excluded: main entries)", obj_count);
    println!("cargo:warning=Generated FFI: {}", out_dir.join("ffi.rs").display());
}
