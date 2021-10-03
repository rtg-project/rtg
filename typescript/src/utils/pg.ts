import * as sql from "pg-sql2";
import { IDatabase } from "pg-promise";

export const createPgAny = (pg: IDatabase<null>) => async (
  q: sql.SQLQuery
): Promise<any[]> => {
  const { text, values } = sql.compile(q);
  return await pg.any(text, values);
};
