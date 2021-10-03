import * as sql from "pg-sql2";
import { newDb } from "pg-mem";
import { createPgAny } from "../../utils/pg";

test("basic2", async () => {
  const pg = newDb().adapters.createPgPromise();
  await pg.connect();

  const pgAny = createPgAny(pg);
  await pgAny(sql.query`create table toto (id integer);`);
  await pgAny(sql.query`insert into toto (id) values (1);`);

  const result = await pgAny(sql.query`select * from toto;`);

  expect(result).toMatchInlineSnapshot(`
    Array [
      Object {
        "id": 1,
        Symbol(_id): "toto_0",
      },
    ]
  `);
});
