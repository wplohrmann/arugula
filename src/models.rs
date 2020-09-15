#[derive(Queryable)]
pub struct Recipe {
    author: String,
    title: String,
    source: String,
    instructions: String,
    ingredients: String,
    date_added: String
}
