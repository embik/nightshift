extern crate wayland_scanner;

use std::env::var;
use std::path::Path;

use wayland_scanner::{Side, generate_code, generate_interfaces};

fn main() {
    // Location of protocol XML file relative to Cargo.toml
    let gamma_control_file = "./wayland/protocols/gamma-control.xml";

    // Target directory
    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    generate_code(
        gamma_control_file,
        out_dir.join("gamma_control_api.rs"),
        Side::Client
    );

    generate_interfaces(
        gamma_control_file,
        out_dir.join("gamma_control_interface.rs")
    );
}
