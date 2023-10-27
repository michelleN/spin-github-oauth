cargo_component_bindings::generate!();

use crate::bindings::exports::wasi::http::incoming_handler::{
    Guest, IncomingRequest, ResponseOutparam,
};

use crate::bindings::fermyon::github_oauth::github_oauth_flow as oauth;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn handle(_req: IncomingRequest, _out: ResponseOutparam) {
        if oauth::validate(&[/* headers */], "" /*state-param*/) {
            panic!("HELLO");
        }

        let token = oauth::exchange(
            "", // code
            "", // redirect-url
            "", // client-id
            "", // client-secret
        );

        println!("{token}");
    }
}
