
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

#[get("/latest")]
async fn latest() -> Template {

    let coca: StockInfo = Stock::new("phm.bmex".to_string()).await.unwrap();
    let aapl: StockInfo = Stock::new("AAPL".to_string()).await.unwrap();
    let fb: StockInfo = Stock::new("FB".to_string()).await.unwrap();
    
    let mut context: HashMap<String, StockInfo> = HashMap::new();
    context.insert("stock1".to_string(), aapl);
    context.insert("stock2".to_string(), coca);
    context.insert("stock3".to_string(), fb);

    let mut vec: HashMap<String, HashMap<String, StockInfo>> = HashMap::new();

    vec.insert(format!("stocks"), context);
    
    Template::render("stock", &vec)

}   

#[launch]
fn rocket() -> _ {
    
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.png",
        ))
        .attach(Template::fairing())
        .mount("/", routes![home, latest, favicon])