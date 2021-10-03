import {
  DocumentNode,
  SelectionSetNode,
  InlineFragmentNode,
  SelectionNode,
  DefinitionNode,
  OperationDefinitionNode,
  FieldNode,
  IntValueNode,
  StringValueNode,
  ListValueNode
} from "graphql";
import { map, fromPairs } from "lodash/fp";
import * as sql from "pg-sql2";
import { TypeReferenceElement } from "../connectors/types";

import {
  IndexedConfig,
  IndexedGraphSchemaEntity
} from "../indexed-config/types";

export const generateQueryForField = <
  TheTypeReferenceElement extends TypeReferenceElement
>(
  graphEntity: IndexedGraphSchemaEntity<TheTypeReferenceElement>,
  subQueryName: string | symbol,
  selection: FieldNode
): sql.SQLQuery => {
  const field =
    graphEntity.fieldsByGraphqlFieldName[selection.name.value].field;
  const resultFieldName = selection.alias?.value ?? selection.name.value;
  switch (field.type) {
    case "scalarDatabaseColumn": {
      const { databaseColumnName } = field;
      return sql.query`to_json((${sql.identifier(
        subQueryName,
        databaseColumnName
      )})) as ${sql.identifier(resultFieldName)}`;
    }
    case "relationOne": {
      throw `Fields of type ${field.type} are not supported`;
    }
    case "relationMany": {
      const relation = field.relation;

      const args = fromPairs(
        map(arg => [arg.name.value, arg], selection.arguments)
      );

      const orderByEntry =
        graphEntity.fieldsByGraphqlOrderBy[
          ((args?.orderBy?.value as ListValueNode)
            ?.values?.[0] as StringValueNode)?.value ??
            // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
            graphEntity.graphqlDefaultOrderBy
        ];

      const orderBy = sql.fragment`${sql.identifier(
        subQueryName,
        orderByEntry.field.databaseColumnName
      )} ${sql.raw(orderByEntry.direction)}`;

      const limitEntry = parseInt(
        (args?.first?.value as IntValueNode)?.value ??
          // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
          `${graphEntity.graphqlDefaultFirst}`
      );

      const limit = sql.value(limitEntry);

      const offsetEntry = parseInt(
        (args?.offset?.value as IntValueNode)?.value ??
          // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
          `${graphEntity.graphqlDefaultOffset}`
      );

      const offset = sql.value(offsetEntry);

      const where = sql.fragment`(TRUE)`;

      const fromRelationTable = sql.identifier(
        graphEntity.databaseSchemaName,
        graphEntity.databaseTableName
      );

      const fromOtherSideOfRelation = sql.identifier(
        graphEntity.databaseSchemaName,
        graphEntity.databaseTableName
      );

      const subSubQueryName = Symbol();
      const subSubQueryObjectName = Symbol();
      const relationQueryName = Symbol();
      // throw `Fields of type ${field.type} are not supported`;
      return sql.query`  to_json(
    (
      select coalesce(
        (
          select json_agg(${sql.identifier(subSubQueryName, "object")})
          from (
            select json_build_object(
              'name'::text,
              (${sql.identifier(subSubQueryObjectName, "name")})
            ) as object
            from (
              select ${sql.identifier(subSubQueryObjectName)}.*
              from "public"."organization" as ${sql.identifier(
                subSubQueryObjectName
              )}
              where (
                ${sql.identifier(subSubQueryObjectName, "id")} in (
                  select "A"
                  from "public"."organization_users" as ${sql.identifier(
                    relationQueryName
                  )}
                  where ("B" = ${sql.identifier(subQueryName, "id")}) and (TRUE)
                )
              ) and (TRUE)
              order by ${sql.identifier(subSubQueryObjectName, "id")} ASC
              limit ${limit}
              offset ${offset}
            ) ${sql.identifier(subSubQueryObjectName)}
          ) as ${sql.identifier(subSubQueryName)}
        ),
        '[]'::json
      )
    )
  ) as ${sql.identifier(resultFieldName)}`;
    }
    default: {
      throw `Unsupported field type`;
    }
  }
};

export const generateQueryForListOperation = <
  TheTypeReferenceElement extends TypeReferenceElement
>(
  indexedConfig: IndexedConfig<TheTypeReferenceElement>,
  graphEntity: IndexedGraphSchemaEntity<TheTypeReferenceElement>,
  fieldNode: FieldNode
): sql.SQLQuery => {
  const selectionSet: SelectionSetNode = fieldNode?.selectionSet;

  const subQueryName = Symbol();

  const fields = sql.join(
    map((selection: SelectionNode) => {
      switch (selection.kind) {
        case "Field": {
          return generateQueryForField(graphEntity, subQueryName, selection);
        }
        case "InlineFragment": {
          throw "InlineFragments are not supported";
        }
        case "FragmentSpread": {
          throw "FragmentSpreads are not supported";
        }
        default: {
          throw "Only simple fields are supported for now";
        }
      }
    }, selectionSet?.selections),
    ",\n"
  );

  const args = fromPairs(
    map(arg => [arg.name.value, arg], fieldNode.arguments)
  );

  const orderByEntry =
    graphEntity.fieldsByGraphqlOrderBy[
      ((args?.orderBy?.value as ListValueNode)?.values?.[0] as StringValueNode)
        ?.value ??
        // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
        graphEntity.graphqlDefaultOrderBy
    ];

  const orderBy = sql.fragment`${sql.identifier(
    subQueryName,
    orderByEntry.field.databaseColumnName
  )} ${sql.raw(orderByEntry.direction)}`;

  const limitEntry = parseInt(
    (args?.first?.value as IntValueNode)?.value ??
      // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
      `${graphEntity.graphqlDefaultFirst}`
  );

  const limit = sql.value(limitEntry);

  const offsetEntry = parseInt(
    (args?.offset?.value as IntValueNode)?.value ??
      // (args?.first?.value as VariableNode)?.name ?? // FIXME No support for variables yet
      `${graphEntity.graphqlDefaultOffset}`
  );

  const offset = sql.value(offsetEntry);

  const where = sql.fragment`(TRUE)`;

  const from = sql.identifier(
    graphEntity.databaseSchemaName,
    graphEntity.databaseTableName
  );

  const subQuery = sql.query`select ${sql.identifier(subQueryName)}.*
  from ${from} as ${sql.identifier(subQueryName)}
  where ${where}
  order by ${orderBy}
  limit ${limit}
  offset ${offset}`;

  const mainQuery = sql.query`select ${fields}
from (
  ${subQuery}
) ${sql.identifier(subQueryName)}`;

  return mainQuery;
};

export const generateSqlQueryForOperationDefinition = <
  TheTypeReferenceElement extends TypeReferenceElement
>(
  indexedConfig: IndexedConfig<TheTypeReferenceElement>,
  definitionNode: OperationDefinitionNode
): sql.SQLQuery => {
  const operationType = definitionNode?.operation;
  if (operationType !== "query") {
    throw "Only queries are supported for now, not mutations or subscriptions";
  }
  const selectionSet: SelectionSetNode = definitionNode?.selectionSet;
  if (selectionSet.selections.length != 1) {
    throw `Multi selection root queries are not supported yet, you can only query one field at the root`;
  }
  const selectionNode: SelectionNode = selectionSet.selections[0];
  if (selectionNode.kind !== "Field") {
    throw `SelectionNodes of type "${selectionNode.kind}" are not supported yet, only fields are supported`;
  }
  const {
    operationKind,
    entity
  } = indexedConfig.indexedGraphSchema.entitiesByOperationName[
    selectionNode.name.value
  ];

  switch (operationKind) {
    case "getList": {
      return generateQueryForListOperation(
        indexedConfig,
        entity,
        selectionNode
      );
    }
    default: {
      throw `Operations of type "${operationKind}" are not supported yet, only listMultiple are supported`;
    }
  }
};

export const generateSqlQuery = <
  TheTypeReferenceElement extends TypeReferenceElement
>(
  indexedConfig: IndexedConfig<TheTypeReferenceElement>,
  graphqlQuery: DocumentNode
): sql.SQLQuery => {
  if (graphqlQuery?.definitions.length != 1) {
    throw "Multi definition queries not supported yet, your document can only contain one query definition";
  }
  const definitionNode: DefinitionNode = graphqlQuery?.definitions[0];

  switch (definitionNode.kind) {
    case "OperationDefinition": {
      return generateSqlQueryForOperationDefinition(
        indexedConfig,
        definitionNode
      );
    }
    default: {
      throw `Definitions of type "${definitionNode.kind}"" are not supported yet, only operation definitions are supported by the query engine`;
    }
  }
};

// // Construct a schema, using GraphQL schema language
// export const schema = buildSchema(`
//   type Query {
//     user(id: String!): User
//   }

//   type User {
//     id: String
//     name: String
//     email: String
//     address: Address
//   }

//   type Address {
//     id: String
//     name: String
//     users: [User!]!
//   }
// `);

// // The root provides a resolver function for each API endpoint
// export const root = {
//   user: async (args, context, info, undd) => {
//     const { pgAny } = context;

//     console.log("args");
//     console.log(args);
//     console.log("context");
//     console.log(context);
//     console.log("info");
//     console.log(JSON.stringify(info, null, 2));
//     console.log("undd");
//     console.log(undd);

//     // const query = `SELECT json_build_object('id',id) FROM "User" LIMIT 1 `;

//     // or import sql from 'pg-sql2';

//     console.log("sql");
//     console.log(sql);

//     const fieldNodes = info.fieldNodes[0];

//     const tableName = fieldNodes.name.value;
//     const fields = map(
//       selection => selection.name.value,
//       fieldNodes.selectionSet.selections
//     );

//     console.log("fields");
//     console.log(fields);

//     // sql.join is used to join fragments with a common separator, NOT to join tables!
//     const sqlFields = sql.join(
//       // sql.identifier safely escapes arguments and joins them with dots
//       map(fieldName => sql.identifier(tableName, fieldName), fields),
//       ", "
//     );

//     console.log("sqlFields");
//     console.log(sqlFields);

//     // sql.value will store the value and instead add a placeholder to the SQL
//     // statement, to ensure that no SQL injection can occur.
//     const sqlConditions = sql.query`created_at > NOW() - interval '3 years' and age > ${sql.value(
//       22
//     )}`;

//     // This could be a full query, but we're going to embed it in another query safely
//     const innerQuery = sql.query`select ${sqlFields} from ${sql.identifier(
//       tableName
//     )} where ${sqlConditions}`;

//     // Symbols are automatically assigned unique identifiers
//     const sqlAlias = sql.identifier(Symbol());

//     const query = sql.query`
// with ${sqlAlias} as (${innerQuery})
// select
//   (select json_agg(row_to_json(${sqlAlias})) from ${sqlAlias}) as all_data,
//   (select max(age) from ${sqlAlias}) as max_age
// `;

//     // sql.compile compiles the query into an SQL statement and a list of values
//     const { text, values } = sql.compile(query);

//     console.log(text);
//     /* ->
// with __local_0__ as (select "user"."name", "user"."age", "user"."height" from "user" where created_at > NOW() - interval '3 years' and age > $1)
// select
//   (select json_agg(row_to_json(__local_0__)) from __local_0__) as all_data,
//   (select max(age) from __local_0__) as max_age
// */

//     console.log(values); // [ 22 ]

//     // Then to run the query using `pg` module, do something like:
//     // const { rows } = await pg.query(text, values);

//     // return {
//     //   id: "tutu",
//     //   name: "titi"
//     // };

//     return (await pgAny(query))?.[0];
//   }
// };

// export const toto = async () => {
//   const response = await graphql(
//     schema,
//     `
//       {
//         auser: user(id: "ok") {
//           bid: id
//           cname: name
//           daddress: address {
//             eid: id
//           }
//         }
//       }
//     `,
//     root,
//     { thisIsContext: true },
//     { variable1: 5 }
//   );
// };
