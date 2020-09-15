#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use rusqlite::{Connection};
use std::collections::HashMap;


#[database("recipes_db")]
struct RecipesDbConn(Connection);
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Recipe {
    pub id: isize,
    pub author: String,
    pub title: String,
    pub source: String,
    pub instructions: String,
    pub ingredients: String,
    pub date_added: String
}
use rocket_contrib::templates::Template;

#[get("/")]
fn index(conn: RecipesDbConn) -> Template {
    let mut stmt = conn.0.prepare("SELECT * FROM recipe").unwrap();
    let recipe_iter = stmt.query_map(&[], |row| {
        Recipe{
            id: row.get(0),
            author: row.get(1),
            title: row.get(2),
            source: row.get(3),
            instructions: row.get(4),
            ingredients: row.get(5),
            date_added: row.get(6)
        }
    }).unwrap();

    let mut context = HashMap::new();
    let all_recipes: Vec<Recipe> = recipe_iter.map(|maybe_recipe| maybe_recipe.unwrap()).collect();
    context.insert("recipes", all_recipes);
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .attach(RecipesDbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
