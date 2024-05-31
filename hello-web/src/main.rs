#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod db;
mod models;
mod schema;

use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::form::Form;
use db::DbConn;
use models::{Item, NewItem};
use schema::items::dsl::*;
use diesel::prelude::*;
use rocket::response::content::Html;

#[database("postgres_db")]
struct DbConn(diesel::PgConnection);

#[get("/")]
async fn index(conn: DbConn) -> Html<String> {
    let result = conn.run(|c| {
        items
            .load::<Item>(c)
    }).await.unwrap();

    let mut html = String::from(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Items</title>
        </head>
        <body>
            <form method="get" action="/">
                <input type="text" name="search" placeholder="Search">
                <select name="sort_by">
                    <option value="name">Name</option>
                    <option value="description">Description</option>
                </select>
                <button type="submit">Search</button>
            </form>
            <table>
                <tr>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Description</th>
                </tr>
    "#);

    for item in result {
        html.push_str(&format!(r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>
        "#, item.id, item.name, item.description));
    }

    html.push_str(r#"
            </table>
        </body>
        </html>
    "#);

    Html(html)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index])
}
