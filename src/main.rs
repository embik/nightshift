#[macro_use] extern crate wayland_client;
#[macro_use] extern crate wayland_sys;

use gamma_control::generated::client::gamma_control_manager as gamma_control_manager;
use gamma_control::generated::client::gamma_control as wl_gamma_control;
use wayland_client::{EnvHandler, EventQueueHandle};
use wayland_client::protocol::wl_output;

mod gamma_control;

wayland_env!(WaylandEnv,
    output: wl_output::WlOutput,
    gamma_control_manager: gamma_control_manager::GammaControlManager,
    gamma_control: wl_gamma_control::GammaControl
);

struct GammaControlHandler {
    pub gamma_size: u32
}

impl wl_gamma_control::Handler for GammaControlHandler {
    fn gamma_size(&mut self,
                  evqh: &mut EventQueueHandle,
                  me: &wl_gamma_control::GammaControl,
                  size: u32) {
        self.gamma_size = size;
    }
}

impl GammaControlHandler {
    pub fn new() -> GammaControlHandler { GammaControlHandler { gamma_size: 0 } }
}

declare_handler!(GammaControlHandler, wl_gamma_control::Handler, wl_gamma_control::GammaControl);

fn main() {
    let (display, mut event_queue) = match wayland_client::default_connect() {
        Ok(ret) => ret,
        Err(e) => panic!("Cannot connect to Wayland server: {:?}", e),
    };

    let registry = display.get_registry();
    let env_id = event_queue.add_handler(EnvHandler::<WaylandEnv>::new());
    event_queue.register::<_, EnvHandler<WaylandEnv>>(&registry, env_id);

    event_queue.sync_roundtrip().unwrap();

    let env = event_queue.state().get_handler::<EnvHandler<WaylandEnv>>(env_id);

    let gc_id = event_queue.add_handler(GammaControlHandler::new());
    event_queue.register::<_, GammaControlHandler>(&env.gamma_control, gc_id);

    println!("{}", env.ready());

    let gamma_control = env.gamma_control_manager.get_gamma_control(&env.output).expect(
        "Cannot connect to gamma control"
    );

    let red: Vec<u8> = vec![1, 1, 1];
    let green: Vec<u8> = vec![1, 1, 1];
    let blue: Vec<u8> = vec![1, 1, 1];

    gamma_control.set_gamma(red, green, blue).expect("Cannot set gamma");
}
