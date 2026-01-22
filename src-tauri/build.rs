fn main() {
    // Windows-specific: help the linker find libxml2 from vcpkg
    #[cfg(target_os = "windows")]
    {
        if let Ok(vcpkg_root) = std::env::var("VCPKG_ROOT") {
            // Try static-md triplet first (preferred), then dynamic
            let triplets = ["x64-windows-static-md", "x64-windows"];
            for triplet in triplets {
                let lib_path = format!("{}/installed/{}/lib", vcpkg_root, triplet);
                if std::path::Path::new(&lib_path).exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                    break;
                }
            }
        }
        // Also check direct environment variable
        if let Ok(lib_dir) = std::env::var("LIBXML2_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", lib_dir);
        }
    }

    tauri_build::build()
}
