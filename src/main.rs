
use std::collections::HashMap;
use rocket_dyn_templates::Template;



#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_include_static_resources;

mod models;
use models::stock_model::*;

static_response_handler! {
    "/favicon.png" => favicon => "favicon"
}

#[get("/")]
fn home() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();

    Template::render("home", context)
}