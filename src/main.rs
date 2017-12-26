#[macro_use] extern crate wayland_client;
#[macro_use] extern crate wayland_sys;

mod wayland;

use wayland_client::EnvHandler;
use wayland_client::protocol::{wl_output, wl_compositor};

use wayland::gamma_control;

wayland_env!(WaylandEnv,
    compositor: wl_compositor::WlCompositor,
    output: wl_output::WlOutput,
    gamma_control_manager: wayland::gamma_control::GammaControlManager
);

fn gamma_control_impl() -> gamma_control::Implementation<()> {
    gamma_control::Implementation {
        gamma_size: |_, _, gamma_control, size| {
            println!("gamma_size: {}", size);
            let red = vec![50];
            let green = vec![50];
            let blue = vec![50];

            gamma_control.set_gamma(red, green, blue).expect("set_gamma failed");
            println!("set_gamma applied");
        },
    }
}

fn main() {
    let (display, mut event_queue) = match wayland_client::default_connect() {
        Ok(ret) => ret,
        Err(e) => panic!("Cannot connect to Wayland server: {:?}", e),
    };

    let registry = display.get_registry();
    let env_token = EnvHandler::<WaylandEnv>::init(&mut event_queue, &registry);

    event_queue.sync_roundtrip().unwrap();

    let env = event_queue.state().get(&env_token).clone_inner().unwrap();
    let gamma_control_manager = env.gamma_control_manager;
    let gamma_control = gamma_control_manager.get_gamma_control(&env.output).expect("error");

    event_queue.register(&gamma_control, gamma_control_impl(), ());

    loop {
        display.flush().unwrap();
        event_queue.dispatch().unwrap();
    }
}
