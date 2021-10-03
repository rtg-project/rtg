// #![deny(warnings)]
#![allow(missing_docs)]
#![allow(unused_variables)]

use juniper::{
  graphql_object, EmptySubscription, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject,
  ScalarValue,
};
use std::env;
use std::fmt::Display;
use warp::{http::Response, Filter};

struct DatabasePool;
impl DatabasePool {
  fn get_connection(&self) -> FieldResult<DatabasePool> {
    Ok(DatabasePool)
  }
  fn find_human(&self, _id: &str) -> FieldResult<Human> {
    Err("")?
  }
  fn insert_human(&self, _human: &NewHuman) -> FieldResult<Human> {
    Err("")?
  }
}

#[derive(GraphQLEnum)]
enum Episode {
  NewHope,
  Empire,
  Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
  id: String,
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

// Now, we create our root Query and Mutation types with resolvers by using the
// object macro.
// Objects can have contexts that allow accessing shared state like a database
// pool.

struct Context {
  // Use your real database pool here.
  pool: DatabasePool,
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}

impl Context {
  fn new() -> Context {
    Context { pool: DatabasePool }
  }
}

struct Query;

#[graphql_object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    context = Context,
)]
impl Query {
  fn apiVersion() -> &'static str {
    "1.0"
  }

  // Arguments to resolvers can either be simple types or input objects.
  // To gain access to the context, we specify a argument
  // that is a reference to the Context type.
  // Juniper automatically injects the correct context here.
  fn human(context: &Context, id: String) -> FieldResult<Human> {
    // Get a db connection.
    let connection = context.pool.get_connection()?;
    // Execute a db query.
    // Note the use of `?` to propagate errors.
    let human = connection.find_human(&id)?;
    // Return the result.
    Ok(human)
  }
}

// Now, we do the same for our Mutation type.

struct Mutation;

#[graphql_object(
    context = Context,
    // If we need to use `ScalarValue` parametrization explicitly somewhere
    // in the object definition (like here in `FieldResult`), we could
    // declare an explicit type parameter for that, and specify it.
    scalar = S: ScalarValue + Display,
)]
impl Mutation {
  fn createHuman<S: ScalarValue + Display>(
    context: &Context,
    new_human: NewHuman,
  ) -> FieldResult<Human, S> {
    let db = context
      .pool
      .get_connection()
      .map_err(|e| e.map_scalar_value())?;
    let human: Human = db
      .insert_human(&new_human)
      .map_err(|e| e.map_scalar_value())?;
    Ok(human)
  }
}

// impl juniper::GraphQLValueAsync for Mutation {

// }

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

// fn main() {
//   let _ = Schema::new(Query, Mutation{}, EmptySubscription::new());
// }

fn schema() -> Schema {
  Schema::new(Query, Mutation, EmptySubscription::<Context>());
}

#[tokio::main]
async fn main() {
  env::set_var("RUST_LOG", "warp_server");
  env_logger::init();

  let log = warp::log("warp_server");

  let homepage = warp::path::end().map(|| {
    Response::builder()
      .header("content-type", "text/html")
      .body(format!(
        "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
      ))
  });

  log::info!("Listening on 127.0.0.1:8080");

  let state = warp::any().map(move || Context::new());
  let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

  warp::serve(
    warp::get()
      .and(warp::path("graphiql"))
      .and(juniper_warp::graphiql_filter("/graphql", None))
      .or(homepage)
      .or(warp::path("graphql").and(graphql_filter))
      .with(log),
  )
  .run(([127, 0, 0, 1], 8080))
  .await
}
