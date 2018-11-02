extern crate actix_web;

use std::env;

use actix_web::{fs, server, App};

mod constants;
mod route_handlers;
use route_handlers::main_page;

fn main() {
    let application_server = server::new(|| {
        let static_file_provider = fs::StaticFiles::new(constants::dev::STATIC_DIR)
            .unwrap()  // TODO: how should this error be handled?
            .show_files_listing();

        App::new()
            .resource("/", |r| r.f(main_page::index))
            .handler("/static", static_file_provider)
    });
    let application_server = match env::var_os(constants::dev::DEV_ENV_VAR) {
        Some(_) => application_server
            .bind(constants::dev::DEV_IPADDRESS)
            .expect(&format!(
                "Could not bind to port \"{}\".",
                constants::dev::DEV_IPADDRESS
            )),
        _ => {
            let error_message = format!(
                "\n\n{}\n{}\n{}\n\n",
                format!(
                    "Could not bind to port \"{}\".",
                    constants::dev::PROD_IPADDRESS
                ),
                format!(
                    "To run on the development port, set \"{}\" as an environment variable.",
                    constants::dev::DEV_ENV_VAR,
                ),
                "Or try running as root or forwarding port 80 to this port with iptables.",
            );

            application_server
                .bind(constants::dev::PROD_IPADDRESS)
                .expect(&error_message)
        }
    };

    if let Some(&socket_addr) = application_server.addrs().first() {
        println!(
            "Now running on http://{}:{}",
            socket_addr.ip(),
            socket_addr.port(),
        );
    }
    application_server.run();
}
