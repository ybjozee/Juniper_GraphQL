use database::Database;
use resolver::{Query, Schema, Mutation};
use juniper::EmptySubscription;
use rocket::{response::content, State};

mod database;
mod model;
mod schema;
mod resolver;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, context)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
        .expect("server to launch");
}
