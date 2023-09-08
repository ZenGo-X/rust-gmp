fn main() {
    #[cfg(target_os = "macos")]
    {
        let gmp_config = pkg_config::probe_library("gmp").unwrap();
        for link_lib in gmp_config.libs {
            println!("cargo:rustc-link-lib={}", link_lib);
        }
        for link_path in gmp_config.link_paths {
            println!("cargo:rustc-link-search=native={}", link_path.to_str().expect("Path is not unicode"));
        }
    }
}

