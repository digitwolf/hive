fn main() {
    // Re-run when the build-time config changes so a new hive_id / secret
    // actually lands in the binary.
    println!("cargo:rerun-if-changed=cfg.toml");
    embuild::espidf::sysenv::output();
}
