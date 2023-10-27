cargo_component_bindings::generate!();

use bindings::wasi::http::types::Fields;

use crate::bindings::exports::fermyon::github_oauth::github_oauth_flow::Guest;
use crate::bindings::wasi::http::outgoing_handler::{self, OutgoingRequest};
use crate::bindings::wasi::http::types::Method;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn validate(headers: Vec<(String, String)>, state_param: String) -> bool {
        return true;
    }

    fn exchange(
        code: String,
        redirect_url: String,
        client_id: String,
        client_secret: String,
    ) -> String {
        let req = OutgoingRequest::new(&Method::Get, None, None, None, &Fields::new(&[]));
        let res = outgoing_handler::handle(req, None)
            .expect("executing outgoing request")
            .get();
        // let resp = OutgoingRequest::handle(&req);
        println!("{res:#?}");
        // match resp {
        //     Ok(resp) => {
        //         let body = resp.body();
        //         return String::from(body);
        //     }
        //     Err(e) => {
        //         return String::from("Error");
        //     }
        // }
        return String::from("Hello, world!");
    }
}
