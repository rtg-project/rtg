import gql from "graphql-tag";
import * as sql from "pg-sql2";

import { generateIndexedConfig } from "../../indexed-config/generate-indexed-config";
import { connector } from "../../connectors/postgres";
import { graphSchema } from "../../__data__/config1";

import { generateSqlQuery } from "../generate-sql-query";

const indexedConfig = generateIndexedConfig({ connector, graphSchema });

test("simple1.1", () => {
  const result = sql.compile(
    generateSqlQuery(
      indexedConfig,
      gql`
        query {
          users {
            id
          }
        }
      `
    )
  );
  expect(result.text).toMatchInlineSnapshot(`
    "select to_json((__local_0__.\\"id\\")) as \\"id\\"
    from (
      select __local_0__.*
      from \\"public\\".\\"user\\" as __local_0__
      where (TRUE)
      order by __local_0__.\\"id\\" ASC
      limit $1
      offset $2
    ) __local_0__"
  `);

  expect(result.values).toMatchInlineSnapshot(`
    Array [
      11,
      0,
    ]
  `);
});

test("simple1.2", () => {
  const result = sql.compile(
    generateSqlQuery(
      indexedConfig,
      gql`
        query {
          users(first: 3, offset: 1, orderBy: ["idDesc"]) {
            id
          }
        }
      `
    )
  );
  expect(result.text).toMatchInlineSnapshot(`
    "select to_json((__local_0__.\\"id\\")) as \\"id\\"
    from (
      select __local_0__.*
      from \\"public\\".\\"user\\" as __local_0__
      where (TRUE)
      order by __local_0__.\\"id\\" DESC
      limit $1
      offset $2
    ) __local_0__"
  `);

  expect(result.values).toMatchInlineSnapshot(`
    Array [
      3,
      1,
    ]
  `);
});

test("simple2.1", () => {
  const result = sql.compile(
    generateSqlQuery(
      indexedConfig,
      gql`
        query {
          users(first: 3) {
            id
            organizations(first: 5) {
              name
            }
          }
        }
      `
    )
  );
  expect(result.text).toEqual(`select to_json((__local_0__."id")) as "id",
  to_json(
    (
      select coalesce(
        (
          select json_agg(__local_1__."object")
          from (
            select json_build_object(
              'name'::text,
              (__local_2__."name")
            ) as object
            from (
              select __local_2__.*
              from "public"."organization" as __local_2__
              where (
                __local_2__."id" in (
                  select "A"
                  from "public"."organization_users" as __local_3__
                  where ("B" = __local_0__."id") and (TRUE)
                )
              ) and (TRUE)
              order by __local_2__."id" ASC
              limit $1
              offset $2
            ) __local_2__
          ) as __local_1__
        ),
        '[]'::json
      )
    )
  ) as "organizations"
from (
  select __local_0__.*
  from "public"."user" as __local_0__
  where (TRUE)
  order by __local_0__."id" ASC
  limit $3
  offset $4
) __local_0__`);
  expect(result.values).toMatchInlineSnapshot(`
    Array [
      5,
      0,
      3,
      0,
    ]
  `);
});

test.skip("simple3", () => {
  expect(
    sql.compile(
      generateSqlQuery(
        indexedConfig,
        gql`
          query {
            users(first: 3) {
              id
              email
              organizations(first: 5) {
                id
                nickname
              }
              operatorOfMissions(first: 7) {
                id
                name
              }
              comments(first: 11) {
                id
                content
              }
            }
          }
        `
      )
    ).text
  ).toEqual(
    `select to_json((__local_0__."id")) as "id",
to_json((__local_0__."email")) as "email",
to_json(
  (
    select coalesce(
      (
        select json_agg(__local_1__."object")
        from (
          select json_build_object(
            'id'::text,
            (__local_2__."id"),
            'nickname'::text,
            (__local_2__."nickname")
          ) as object
          from (
            select __local_2__.*
            from "public"."Organization" as __local_2__
            where (
              __local_2__."id" in (
                select "A"
                from "public"."_OrganizationUsers" as __local_3__
                where ("B" = __local_0__."id") and (TRUE) and (TRUE)
              )
            ) and (TRUE) and (TRUE)
            order by __local_2__."id" ASC
            limit 5
          ) __local_2__
        ) as __local_1__
      ),
      '[]'::json
    )
  )
) as "@organizations",
to_json(
  (
    select coalesce(
      (
        select json_agg(__local_4__."object")
        from (
          select json_build_object(
            'id'::text,
            (__local_5__."id"),
            'name'::text,
            (__local_5__."name")
          ) as object
          from (
            select __local_5__.*
            from "public"."Mission" as __local_5__
            where (
              __local_5__."id" in (
                select "A"
                from "public"."_MissionOperator" as __local_6__
                where ("B" = __local_0__."id") and (TRUE) and (TRUE)
              )
            ) and (TRUE) and (TRUE)
            order by __local_5__."id" ASC
            limit 7
          ) __local_5__
        ) as __local_4__
      ),
      '[]'::json
    )
  )
) as "@operatorOfMissions",
to_json(
  (
    select coalesce(
      (
        select json_agg(__local_7__."object")
        from (
          select json_build_object(
            'id'::text,
            (__local_8__."id"),
            'content'::text,
            (__local_8__."content")
          ) as object
          from (
            select __local_8__.*
            from "public"."Comment" as __local_8__
            where (__local_8__."author" = __local_0__."id") and (TRUE) and (TRUE)
            order by __local_8__."id" ASC
            limit 11
          ) __local_8__
        ) as __local_7__
      ),
      '[]'::json
    )
  )
) as "@comments"
from (
  select __local_0__.*
  from "public"."User" as __local_0__
  where (TRUE) and (TRUE)
  order by __local_0__."id" ASC
  limit 3
) __local_0__
`
  );
});
