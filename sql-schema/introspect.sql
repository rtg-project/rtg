select json_agg("entities")
from (
    select 'databaseTable' as "type",
      "table"."table_name" as "sqlTableName",
      "table"."table_schema" as "sqlSchemaName",
      (
        select json_agg("columns")
        from (
            select 'scalarDatabaseColumn' as "type",
              "column"."column_name" as "sqlColumnName",
              "column"."data_type" as "sqlType",
              "column"."is_nullable"::boolean as "nullable"
            from "information_schema"."columns" "column"
            where "column"."table_schema" = "table"."table_schema"
              and "column"."table_name" = "table"."table_name"
            limit 100
          ) "columns"
      ) as "fields"
    from "information_schema"."tables" "table"
    where "table"."table_schema" = 'public'
    limit 100
  ) "entities";