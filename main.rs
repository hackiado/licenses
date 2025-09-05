#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Roots",
            description: "A simple, fast, and secure web framework for Rust.",
        },
    )
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from("static"))
        .attach(Template::fairing())
}
