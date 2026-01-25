#[macro_use] extern crate rocket;

mod html_parser;

#[get("/")]
fn home() -> String {
    html_parser::parse_html("./index.html".to_string())
}

#[get("/")]
fn blog() -> String {
    html_parser::parse_html("./blog.html".to_string())
}


#[get("/<blog>")]
fn blogs(blog: &str) -> String {
    html_parser::parse_html("./blog.html".to_string())
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![home])
        .mount("/blog", routes![blog, blogs])
}
