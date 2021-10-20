use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject};
use rocket::{response::content, Rocket, State};

#[derive(GraphQLObject)]
#[graphql(description = "Information about a logged-in user")]
struct User {
    pub name: String,
}

struct Context;

impl juniper::Context for Context {}

struct Query;

#[graphql_object(context = Context)]
#[allow(non_snake_case)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    fn currentUser(_context: &Context) -> FieldResult<User> {
        Ok(User {
            name: "Test".into(),
        })
    }
}

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/")]
async fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &Context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &Context).await
}

#[rocket::main]
async fn main() {
    Rocket::build()
        .manage(Schema::new(
            Query,
            EmptyMutation::<Context>::new(),
            EmptySubscription::<Context>::new(),
        ))
        .mount("/", rocket::routes![graphiql, get_graphql, post_graphql])
        .launch()
        .await
        .expect("couldn't launch")
}
