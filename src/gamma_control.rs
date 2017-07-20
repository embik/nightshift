extern crate wayland_sys;
extern crate wayland_client;

pub mod generated {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]

    pub mod interfaces {
        #[doc(hidden)] pub use wayland_client::protocol_interfaces::wl_output_interface;
        include!(concat!(env!("OUT_DIR"), "/gamma_control_interfaces.rs"));
    }

    pub mod client {
        #[doc(hidden)] pub use wayland_client::{Proxy, Handler, EventQueueHandle, RequestResult, Liveness};
        #[doc(hidden)] pub use super::interfaces;

        #[doc(hidden)] pub use wayland_client::protocol::wl_output;
        include!(concat!(env!("OUT_DIR"), "/gamma_control.rs"));
    }
}
