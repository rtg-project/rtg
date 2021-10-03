import { graphql, buildSchema } from "graphql";
import { map } from "lodash/fp";

import * as sql from "pg-sql2";
import { newDb } from "pg-mem";
import { createPgAny } from "../utils/pg";

export const generateDatabase = () => {
  sql.join(
    [
      sql.query`create table toto (id integer)`,
      sql.query`insert into toto (id) values (1)`
    ],
    ";"
  );
};
