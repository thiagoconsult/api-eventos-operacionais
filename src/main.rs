mod header;
mod parse_buf;
mod routes;
mod server;
use crate::server::server;
fn main() {
    match server() {
        Ok(_) => println!("Server finalizado."),
        Err(e) => println!("{:?}", e),
    }
}
