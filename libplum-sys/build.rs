use std::path::PathBuf;

fn main() {
    const DEFINES: [&str; 2] = [
        // Since Rust tends to be stricter than C, I would rather cast pointers via actual casts,
        // rather than unions. (That's all the anonymous members are used for as of v1.2.)
        "-DPLUM_NO_ANON_MEMBERS",
        // We don't use the VLA macros, and defining this thins the header a bit.
        "-DPLUM_NO_VLA",
    ];

    cc::Build::new()
        .file("src/libplum.c")
        // libplum compiles with a lot of warnings under `-Wall -Wextra`; don't annoy downstream users with these.
        // All of the triggered warnings are disabled in the Makefile, and are only non-critical warnings
        // (like "dangling else", which for ax6 is a personal preference—good luck talking him out of it),
        // so please don't fret about this line.
        .warnings(false)
        // libplum requires C17 for actually building it.
        .flag_if_supported("-std=c17") // GCC, Clang, et al.
        .flag_if_supported("/std:c17") // MSVC.
        .compile("plum"); // To end up with "libplum.a" rather than "liblibplum.a".

    let bindings = bindgen::builder()
        .header("src/libplum.h")
        // libplum tests the enabled standard, and enables certain features for C99 or C11.
        // For the sake of reproducibility, avoid depending on the platform's default setting;
        // C11 was picked because it's 12 years old at the time of writing, and well supported.
        .clang_arg("-std=c11")
        .clang_args(&DEFINES)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Only generate bindings for libplum's items, which are namespaced.
        .allowlist_type("plum_.*")
        .allowlist_function("plum_.*")
        // The two macros currently defined are the libplum version (not useful for static linking)
        // and VLA support (internal). So don't bother porting macros over.
        // .allowlist_var("PLUM_.*") // (Macros.)
        .prepend_enum_name(false) // `plum_image_types_PLUM_IMAGE_NONE` kind of sucks, ngl.
        // IMPORTANT: Some manual defines in `lib.rs` export `size_t` as `usize`!
        //            If disabling this, make sure to change their type as well!
        .size_t_is_usize(true)
        // Pointers are involved in almost all structs, even `Clone` is dangerous!
        // (Plus, libplum's `allocator` shenanigans mean you shouldn't attempt to replace anything with a copy.)
        // (It'd be nice to allowlist instead of blocklisting, but that's not supported.)
        .no_copy("plum_(buffer|callback|metadata|image)")
        // TODO:
        // - annotate functions as `must_use` (would require patching libplum.h, though...)
        .generate()
        .expect("Failed to generate libplum bindings");
    let out_path = {
        let mut out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
        out_dir.push("bindings.rs");
        out_dir
    };
    bindings
        .write_to_file(out_path)
        .expect("Failed to write libplum bindings");
}
