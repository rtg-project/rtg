// // This script is called from scripts/generate-cache
// import { createPostGraphileSchema } from "postgraphile";
// import { options } from "./postgraphileOptions";
// import { Pool } from "pg";

// const schemas = process.env.DATABASE_SCHEMAS
//   ? process.env.DATABASE_SCHEMAS.split(",")
//   : ["app_public"];

// async function main() {
//   const pgPool = new Pool({
//     connectionString: process.env.DATABASE_URL
//   });
//   await createPostGraphileSchema(pgPool, schemas, {
//     ...options,
//     writeCache: `${__dirname}/../dist/postgraphile.cache`
//   });
//   await pgPool.end();
// }

// main().then(null, e => {
//   // eslint-disable-next-line no-console
//   console.error(e);
//   process.exit(1);
// });

// import sql from "pg-sql2";

// const tableName = "user";
// const fields = ["name", "age", "height"];

// // sql.join is used to join fragments with a common separator, NOT to join tables!
// const sqlFields = sql.join(
//   // sql.identifier safely escapes arguments and joins them with dots
//   fields.map(fieldName => sql.identifier(tableName, fieldName)),
//   ", "
// );

// // sql.value will store the value and instead add a placeholder to the SQL
// // statement, to ensure that no SQL injection can occur.
// const sqlConditions = sql.query`created_at > NOW() - interval '3 years' and age > ${sql.value(
//   22
// )}`;

// // This could be a full query, but we're going to embed it in another query safely
// const innerQuery = sql.query`select ${sqlFields} from ${sql.identifier(
//   tableName
// )} where ${sqlConditions}`;

// // Symbols are automatically assigned unique identifiers
// const sqlAlias = sql.identifier(Symbol());

// const query = sql.query`
// with ${sqlAlias} as (${innerQuery})
// select
//   (select json_agg(row_to_json(${sqlAlias})) from ${sqlAlias}) as all_data,
//   (select max(age) from ${sqlAlias}) as max_age
// `;

// // sql.compile compiles the query into an SQL statement and a list of values
// const { text, values } = sql.compile(query);

// console.log(text);
// /* ->
// with __local_0__ as (select "user"."name", "user"."age", "user"."height" from "user" where created_at > NOW() - interval '3 years' and age > $1)
// select
//   (select json_agg(row_to_json(__local_0__)) from __local_0__) as all_data,
//   (select max(age) from __local_0__) as max_age
// */

// console.log(values); // [ 22 ]

// // Then to run the query using `pg` module, do something like:
// // const { rows } = await pg.query(text, values);
