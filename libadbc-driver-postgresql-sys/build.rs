fn main() {
    let env_prefix = "ADBC_DRIVER_POSTGRESQL";

    let link_mode = find_link_mode(env_prefix);
    let lib_name = "adbc_driver_postgresql";

    let lib_dir = env_var(&format!("{env_prefix}_LIB_DIR"));
    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:rustc-link-lib={link_mode}={lib_name}");
}

fn find_link_mode(env_prefix: &str) -> &'static str {
    match &maybe_env_var(&format!("{env_prefix}_STATIC")) {
        Ok(v) if v != "0" => "static",
        _ => "dylib",
    }
}

fn maybe_env_var(name: &str) -> Result<String, std::env::VarError> {
    println!("cargo:rerun-if-env-changed={}", name);
    std::env::var(name)
}

fn env_var(var: &str) -> String {
    maybe_env_var(var).unwrap_or_else(|_| panic!("Error: env var {var} not set"))
}
