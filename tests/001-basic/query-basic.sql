select json_build_object('allPersons',(select coalesce((select json_agg(__rtg_9__."__rtg_10__") from (select json_build_object('id',__rtg_11__."id") as __rtg_10__ from "people" as __rtg_11__ limit 10) as __rtg_9__),'[]'::json))) as __rtg_0__