import {
  camelCase,
  size,
  flatMap,
  values,
  fromPairs,
  map,
  filter,
  toPairs,
  first,
  mapValues,
  isNil,
  groupBy,
  find
} from "lodash/fp";
import * as sql from "pg-sql2";

import {
  GraphSchema,
  GraphSchemaEntity,
  GraphSchemaField
} from "../graphschema/types";

import { Connector, TypeReferenceElement } from "../connectors/types";

import {
  IndexedConfig,
  IndexedConnector,
  IndexedGraphSchema,
  IndexedGraphSchemaField,
  IndexedGraphSchemaEntity,
  IndexedTypeReferenceElement
} from "./types";

/**
 * This function indexes the connector and the graphSchema
 * to output an indexed config, which is then used by all modules.
 * This indexed config is not serializable,
 * it allows to navigate an object graph (with cycles)
 * without having to perform manual lookups
 */
export const generateIndexedConfig = <
  TheTypeReferenceElement extends TypeReferenceElement,
  TheGraphSchema extends GraphSchema<readonly TheTypeReferenceElement[]>
>({
  connector,
  graphSchema
}: {
  connector: Connector;
  graphSchema: TheGraphSchema;
}): IndexedConfig<TheTypeReferenceElement> => {
  /////////////////////////////////////////////////////////////////////////////
  const typeReferenceByDatabaseTypeName = fromPairs(
    map((typeReferenceElement: TypeReferenceElement) => {
      return [typeReferenceElement.databaseTypeName, typeReferenceElement];
    }, connector.typeReference)
  );

  /////////////////////////////////////////////////////////////////////////////
  const typeReferenceByGraphqlTypeNameAndGraphqlAnnotation = (mapValues(
    (entries: {
      [key: string]: readonly TypeReferenceElement[];
    }): { [key: string]: TypeReferenceElement } => ({
      ...mapValues(entry => {
        if (
          isNil(entry[0].graphqlTypeName) &&
          isNil(entry[0].graphqlAnnotation)
        ) {
          return undefined;
        }
        if (size(entry) > 1) {
          throw `There are multiple entries in the TypeReference with the graphql type ${entry[0].graphqlTypeName} and annotation ${entry[0].graphqlAnnotation}`;
        }
        return first(entry);
      }, entries),
      preferred:
        find((entry: TypeReferenceElement[]): boolean => {
          return entry[0]?.type === "preferred";
        }, entries)?.[0] ?? undefined
    }),
    mapValues(
      groupBy("graphqlAnnotation"),
      groupBy("graphqlTypeName", connector.typeReference)
    )
  ) as unknown) as {
    [key: string]: {
      [key: string]: IndexedTypeReferenceElement<TheTypeReferenceElement>;
    };
  };

  /////////////////////////////////////////////////////////////////////////////
  const indexedEntities = map(entity => {
    const indexedFields = map(
      field => ({
        ...field,
        databaseType:
          field.type === "scalarDatabaseColumn"
            ? typeReferenceByDatabaseTypeName[field.databaseTypeName]
            : undefined
      }),
      entity.fields
    );
    const fieldsByName = fromPairs(
      map(field => [field.name, field], indexedFields)
    );
    const fieldsByDatabaseColumnName = fromPairs(
      flatMap(
        field =>
          field.type === "scalarDatabaseColumn"
            ? [[field.databaseColumnName, field]]
            : [],
        indexedFields
      )
    );
    const fieldsByGraphqlFieldName = fromPairs(
      (flatMap(
        field =>
          field.type === "scalarDatabaseColumn"
            ? [[field.graphqlFieldName, { operationKind: "getSingle", field }]]
            : field.type === "relationOne"
            ? [
                [
                  field.graphqlGetSingleFieldName,
                  { operationKind: "getSingle", field }
                ]
              ]
            : [
                [
                  field.graphqlGetSingleFieldName,
                  { operationKind: "getSingle", field }
                ],
                [
                  field.graphqlGetListFieldName,
                  { operationKind: "getList", field }
                ],
                [
                  field.graphqlGetConnectionFieldName,
                  { operationKind: "getConnection", field }
                ]
              ],
        indexedFields
      ) as unknown) as Array<
        [
          string,
          {
            operationKind: "getSingle" | "getList" | "getConnection";
            field: IndexedGraphSchemaField<TheTypeReferenceElement>;
          }
        ]
      >
    );

    const fieldsByGraphqlOrderBy = fromPairs(
      flatMap(
        field =>
          field.type === "scalarDatabaseColumn"
            ? [
                [field.graphqlOrderByAsc, { direction: "ASC", field }],
                [field.graphqlOrderByDesc, { direction: "DESC", field }]
              ]
            : [],
        indexedFields
      )
    );
    const indexedEntity = {
      ...entity,
      fieldsByName,
      fieldsByDatabaseColumnName,
      fieldsByGraphqlFieldName,
      fieldsByGraphqlOrderBy
    };
    return indexedEntity;
  }, graphSchema.entities) as Array<
    IndexedGraphSchemaEntity<TheTypeReferenceElement>
  >;

  /////////////////////////////////////////////////////////////////////////////
  const entitiesByName = fromPairs(
    map(entity => [entity.name, entity], indexedEntities)
  );
  /////////////////////////////////////////////////////////////////////////////
  const entitiesByOperationName = fromPairs(
    flatMap(
      entity => [
        [
          entity.graphqlGetSingleOperationName,
          { operationKind: "getSingle", entity }
        ],
        [
          entity.graphqlGetListOperationName,
          { operationKind: "getList", entity }
        ],
        [
          entity.graphqlGetConnectionOperationName,
          { operationKind: "getConnection", entity }
        ]
      ],
      indexedEntities
    )
  ) as Record<
    string,
    {
      operationKind: "getSingle" | "getList" | "getConnection";
      entity: IndexedGraphSchemaEntity<TheTypeReferenceElement>;
    }
  >;

  /////////////////////////////////////////////////////////////////////////////
  const relationsByName = {};

  /////////////////////////////////////////////////////////////////////////////
  return {
    indexedConnector: {
      typeReferenceByDatabaseTypeName,
      typeReferenceByGraphqlTypeNameAndGraphqlAnnotation
    },
    indexedGraphSchema: {
      entitiesByName,
      entitiesByOperationName,
      relationsByName
    }
  };
};
