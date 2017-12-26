pub use wayland::gamma_control::generated::client::gamma_control::{GammaControl, Implementation};
pub use wayland::gamma_control::generated::client::gamma_control_manager::GammaControlManager;

mod generated {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]

    pub mod interfaces {
        #[doc(hidden)]
        pub use wayland_client::protocol_interfaces::{wl_output_interface};
        include!(concat!(env!("OUT_DIR"), "/gamma_control_interface.rs"));
    }

    pub mod client {
        #[doc(hidden)]
        pub use wayland_client::{Proxy, EventQueueHandle, RequestResult, Implementable, Liveness};
        #[doc(hidden)]
        #[doc(hidden)]
        pub use wayland_client::protocol::{wl_output};
        #[doc(hidden)]
        pub use super::interfaces;
        include!(concat!(env!("OUT_DIR"), "/gamma_control_api.rs"));
    }
}
