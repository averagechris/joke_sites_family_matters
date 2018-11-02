extern crate actix_web;

use std::env;

use actix_web::{fs, server, App};

mod route_handlers;
use route_handlers::main_page;

fn main() {
    let dev_env_var = "FAMILY_MATTERS_DEV";
    let dev_ipaddress = "127.0.0.1:8088";
    let prod_ipaddress = "0.0.0.0:80";
    let application_server = server::new(|| {
        App::new().resource("/", |r| r.f(main_page::index)).handler(
            "/static",
            fs::StaticFiles::new("../static")
                .unwrap()
                .show_files_listing(),
        )
    });
    let application_server = match env::var_os(&dev_env_var) {
        Some(_) => application_server
            .bind(dev_ipaddress)
            .expect(&format!("Could not bind to port {}.", dev_ipaddress)),
        _ => {
            let error_message = format!(
                "\n\n{}\n{}\n{}\n\n",
                format!("Could not bind to port \"{}\".", prod_ipaddress),
                format!(
                    "To run on the development port, set \"{}\" as an environment variable.",
                    dev_env_var
                ),
                "Or try running as root or forwarding port 80 to this port with iptables.",
            );

            application_server
                .bind(prod_ipaddress)
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
