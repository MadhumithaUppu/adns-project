// We need to include all the packages needed for the script to work
#[macro_use] extern crate rocket;
use rocket::form::Form;
use rocket_dyn_templates::Template;
use std::{collections::HashMap, error::Error};
use rust_iptables::iptables::IPTables;

#[derive(FromForm)]
struct BlockIp {
    ip: String,
}
// the below function is used to get the ip address that is used for blocking 
#[post("/block_ip", data = "<block_ip>")]
fn block_ip_route(block_ip_form: Form<BlockIp>) -> String {
    let block_ip = &block_ip_form.ip;
    match block_ip(block_ip) {
        Ok(_) => format!("IP {} has been blocked.", block_ip),
        Err(e) => format!("Error blocking IP: {}", e),
    }
}

fn block_ip(ip: &str) -> Result<(), Box<dyn Error>> {
    let ipt = IPTables::new()?;

    // Blocking incoming traffic from the specified IP
    ipt.append("filter", "INPUT", &format!("-s {} -j DROP", ip))?;

    Ok(())
}
// below  I have integrated a route handler to keep the ip tp be blocked
#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}

// Below funtion is used to crate the rocket instace to run the web server to give the ui to set the ip address to be blocked
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, block_ip_route])
        .attach(Template::fairing())
}
