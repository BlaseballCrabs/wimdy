use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLEnum, GraphQLObject,
};
use rocket::{response::content, Rocket, State};
use sqlx::SqlitePool;

#[derive(GraphQLObject)]
#[graphql(description = "Information about a logged-in user")]
struct User {
    pub name: String,
}

#[derive(GraphQLEnum, sqlx::Type)]
enum TargetKind {
    Blessing,
    Decree,
    Will,
}

#[derive(GraphQLObject)]
struct Target {
    pub id: String,
    pub name: String,
    pub season: i32,
    pub kind: TargetKind,
    pub votes: i32,
}

struct Context {
    pub pool: SqlitePool,
}

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

    async fn targets(context: &Context, season: Option<i32>) -> FieldResult<Vec<Target>> {
        let mut conn = context.pool.acquire().await?;
        if let Some(season) = season {
            Ok(sqlx::query_as!(
                Target,
                r#"
            SELECT
                id,
                name,
                season as 'season: i32',
                kind as 'kind: _',
                votes as 'votes: i32'
                FROM target
                WHERE season = ?
            "#,
                season
            )
            .fetch_all(&mut conn)
            .await?)
        } else {
            Ok(sqlx::query_as!(
                Target,
                r#"
            SELECT
                id,
                name,
                season as 'season: i32',
                kind as 'kind: _',
                votes as 'votes: i32'
                FROM target
            "#
            )
            .fetch_all(&mut conn)
            .await?)
        }
    }

    async fn target(context: &Context, id: String) -> FieldResult<Option<Target>> {
        let mut conn = context.pool.acquire().await?;
        Ok(sqlx::query_as!(
            Target,
            r#"
        SELECT
            id,
            name,
            season as 'season: i32',
            kind as 'kind: _',
            votes as 'votes: i32'
            FROM target
            WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&mut conn)
        .await?)
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
    ctx: &State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*ctx).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    ctx: &State<Context>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*ctx).await
}

#[rocket::main]
async fn main() {
    let url = std::env::var("DATABASE_URL").unwrap();
    let pool = SqlitePool::connect(&url).await.unwrap();
    Rocket::build()
        .manage(Context { pool })
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
