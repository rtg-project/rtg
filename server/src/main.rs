#![allow(missing_docs)]
#![allow(unused_variables)]

mod fixtures;

use fixtures::starwars::schema::{Database, Query};
use handlebars::Handlebars;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use scooby::postgres::{select, Aliasable, Joinable, Orderable, Parameters};
use serde::Serialize;
// use serde_json::json;
use std::env;
use warp::{http::Response, Filter};

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

fn schema() -> Schema {
  Schema::new(
    Query,
    EmptyMutation::<Database>::new(),
    EmptySubscription::<Database>::new(),
  )
}

#[derive(Serialize)]
struct SqlPagePrint {
  query: String,
}

#[tokio::main]
async fn main() {
  env::set_var("RUST_LOG", "warp_server");
  env_logger::init();

  let log = warp::log("warp_server");

  let mut handlebars = Handlebars::new();
  let registration_result = match handlebars.register_template_string(
    "template_page_sql",
    "<html><h1>juniper_warp</h1><div>SQL query:<br/>{{query}}</div></html>",
  ) {
    Ok(registration_result) => registration_result,
    Err(e) => return (),
  };

  let homepage = warp::path::end().map(|| {
    Response::builder()
      .header("content-type", "text/html")
      .body(format!(
        "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></div></html>"
      ))
  });

  let sql_page = warp::path::end().map(move || {
    let mut params = Parameters::new();

    let query = select(("country.name".as_("name"), "COUNT(*)".as_("count")))
      .from(
        "Country"
          .as_("country")
          .inner_join("City".as_("city"))
          .on("city.country_id = country.id"),
      )
      .where_(format!("city.population > {}", params.next()))
      .group_by("country.name")
      .order_by("count".desc())
      .limit(10)
      .to_string();

    let sql_page_print = SqlPagePrint { query: query };

    match handlebars.render("template_page_sql", &sql_page_print) {
      Ok(s) => Response::builder()
        .header("content-type", "text/html")
        .body(format!("{}", s)),
      Err(e) => Response::builder()
        .header("content-type", "text/html")
        .body(format!("{}", "Error")),
    }
  });

  log::info!("Listening on 127.0.0.1:8080");

  let state = warp::any().map(move || Database::new());
  let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

  warp::serve(
    warp::get()
      .and(warp::path("graphiql"))
      .and(juniper_warp::graphiql_filter("/graphql", None))
      .or(homepage)
      .or(warp::path("sql").and(sql_page))
      .or(warp::path("graphql").and(graphql_filter))
      .with(log),
  )
  .run(([127, 0, 0, 1], 8080))
  .await
}
