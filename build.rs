use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let memory_x = fs::read_to_string("memory.x").expect("Failed to read memory.x");
    let link_rp_x = fs::read_to_string("link-rp.x").expect("Failed to read link-rp.x");

    let output_memory = out_dir.join("memory.x");
    fs::write(&output_memory, memory_x).expect("Failed to write memory.x into OUT_DIR");
    let output_link_rp = out_dir.join("link-rp.x");
    fs::write(&output_link_rp, link_rp_x).expect("Failed to write link-rp.x into OUT_DIR");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link-rp.x");
}
