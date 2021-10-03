import { graphql, buildSchema } from "graphql";
import { map } from "lodash/fp";

import * as sql from "pg-sql2";
import { newDb } from "pg-mem";
import { createPgAny } from "../pg";

import { graphSchema } from "../__data__/graphschema1";

// Construct a schema, using GraphQL schema language
const schema = buildSchema(`
  type Query {
    user(id: String!): User
  }

  type User {
    id: String
    name: String
    email: String
    address: Address
  }

  type Address {
    id: String
    name: String
    users: [User!]!
  }
`);

// The root provides a resolver function for each API endpoint
const root = {
  user: async (args, context, info, undd) => {
    const { pgAny } = context;

    console.log("args");
    console.log(args);
    console.log("context");
    console.log(context);
    console.log("info");
    console.log(JSON.stringify(info, null, 2));
    console.log("undd");
    console.log(undd);

    // const query = `SELECT json_build_object('id',id) FROM "User" LIMIT 1 `;

    // or import sql from 'pg-sql2';

    console.log("sql");
    console.log(sql);

    const fieldNodes = info.fieldNodes[0];

    const tableName = fieldNodes.name.value;
    const fields = map(
      selection => selection.name.value,
      fieldNodes.selectionSet.selections
    );

    console.log("fields");
    console.log(fields);

    // sql.join is used to join fragments with a common separator, NOT to join tables!
    const sqlFields = sql.join(
      // sql.identifier safely escapes arguments and joins them with dots
      map(fieldName => sql.identifier(tableName, fieldName), fields),
      ", "
    );

    console.log("sqlFields");
    console.log(sqlFields);

    // sql.value will store the value and instead add a placeholder to the SQL
    // statement, to ensure that no SQL injection can occur.
    const sqlConditions = sql.query`created_at > NOW() - interval '3 years' and age > ${sql.value(
      22
    )}`;

    // This could be a full query, but we're going to embed it in another query safely
    const innerQuery = sql.query`select ${sqlFields} from ${sql.identifier(
      tableName
    )} where ${sqlConditions}`;

    // Symbols are automatically assigned unique identifiers
    const sqlAlias = sql.identifier(Symbol());

    const query = sql.query`
with ${sqlAlias} as (${innerQuery})
select
  (select json_agg(row_to_json(${sqlAlias})) from ${sqlAlias}) as all_data,
  (select max(age) from ${sqlAlias}) as max_age
`;

    // sql.compile compiles the query into an SQL statement and a list of values
    const { text, values } = sql.compile(query);

    console.log(text);
    /* ->
with __local_0__ as (select "user"."name", "user"."age", "user"."height" from "user" where created_at > NOW() - interval '3 years' and age > $1)
select
  (select json_agg(row_to_json(__local_0__)) from __local_0__) as all_data,
  (select max(age) from __local_0__) as max_age
*/

    console.log(values); // [ 22 ]

    // Then to run the query using `pg` module, do something like:
    // const { rows } = await pg.query(text, values);

    // return {
    //   id: "tutu",
    //   name: "titi"
    // };

    return (await pgAny(query))?.[0];
  }
};

test("basic", async () => {
  const pg = newDb().adapters.createPgPromise();
  await pg.connect();
  const pgAny = createPgAny(pg);
  const response = await graphql(
    schema,
    `
      {
        auser: user(id: "ok") {
          bid: id
          cname: name
          daddress: address {
            eid: id
          }
        }
      }
    `,
    root,
    { thisIsContext: true, pg, pgAny },
    { variable1: 5 }
  );
  console.log(response);
  expect(1).toEqual(1);
});
