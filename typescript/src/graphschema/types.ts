import { TypeReferenceElement } from "../connectors/types";

// https://stackoverflow.com/a/51399781/2371254
export type ArrayElement<
  ArrayType extends readonly unknown[]
> = ArrayType extends readonly (infer ElementType)[] ? ElementType : never;

///////////////////////////////////////////////////////////////////////////////

export type GraphSchemaField<
  TheTypeReference extends readonly TypeReferenceElement[]
> =
  | GraphSchemaFieldScalarDatabaseColumn<TheTypeReference>
  | GraphSchemaFieldRelationOne<TheTypeReference>
  | GraphSchemaFieldRelationMany<TheTypeReference>;

export type GraphSchemaFieldScalarDatabaseColumn<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "scalarDatabaseColumn";
  name: string;
  databaseTypeName: ArrayElement<TheTypeReference>["databaseTypeName"];
  databaseColumnName: string;
  graphqlFieldName: string;
  graphqlTypeName: ArrayElement<TheTypeReference>["graphqlTypeName"];
  graphqlOrderByAsc: string;
  graphqlOrderByDesc: string;
};

export type GraphSchemaFieldRelationOne<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "relationOne";
  name: string;
  relationName: string;
  graphqlGetSingleFieldName: string;
  graphqlOrderByPrefix: string;
};

export type GraphSchemaFieldRelationMany<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "relationMany";
  name: string;
  relationName: string;
  graphqlGetListFieldName: string;
  graphqlGetConnectionFieldName: string;
  graphqlGetSingleFieldName: string;
  graphqlOrderByPrefix: string;
};

///////////////////////////////////////////////////////////////////////////////

export type GraphSchemaRelation<
  TheTypeReference extends readonly TypeReferenceElement[]
> =
  | GraphSchemaRelationOneToOneForeignKey<TheTypeReference>
  | GraphSchemaRelationOneToManyForeignKey<TheTypeReference>
  | GraphSchemaRelationManyToManyRelationTable<TheTypeReference>
  | GraphSchemaRelationOneToManyRelationTable<TheTypeReference>
  | GraphSchemaRelationOneToOneRelationTable<TheTypeReference>;

export type GraphSchemaRelationOneToOneForeignKey<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "oneToOneConstraint";
  name: string;
  leftEntityName: string;
  leftEntityFieldName: string;
  leftDatabaseColumnNames: string[];
  rightEntityName: string;
  rightEntityFieldName: string;
  rightDatabaseColumnNames: string[];
  rightToLeftDatabaseForeignKeyConstraintName: string;
  rightDatabaseUniqueConstraintName: string;
};

export type GraphSchemaRelationOneToManyForeignKey<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "oneToManyConstraint";
  name: string;
  leftEntityName: string;
  leftEntityFieldName: string;
  leftDatabaseColumnNames: string[];
  rightEntityName: string;
  rightEntityFieldName: string;
  rightDatabaseColumnNames: string[];
  rightToLeftDatabaseForeignKeyConstraintName: string;
};

export type GraphSchemaRelationManyToManyRelationTable<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "manyToManyRelationTable";
  name: string;
  leftEntityName: string;
  leftEntityFieldName: string;
  leftDatabaseColumnNames: string[];
  rightEntityName: string;
  rightEntityFieldName: string;
  rightDatabaseColumnNames: string[];
  relationDatabaseSchemaName: string;
  relationDatabaseTableName: string;
  relationLeftDatabaseColumnNames: string[];
  relationRightDatabaseColumnNames: string[];
  relationToLeftDatabaseForeignKeyConstraintName: string;
  relationToRightDatabaseForeignKeyConstraintName: string;
};

export type GraphSchemaRelationOneToManyRelationTable<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "oneToManyRelationTable";
  name: string;
  leftEntityName: string;
  leftEntityFieldName: string;
  leftDatabaseColumnNames: string[];
  rightEntityName: string;
  rightEntityFieldName: string;
  rightDatabaseColumnNames: string[];
  relationDatabaseSchemaName: string;
  relationDatabaseTableName: string;
  relationLeftDatabaseColumnNames: string[];
  relationRightDatabaseColumnNames: string[];
  relationToLeftDatabaseForeignKeyConstraintName: string;
  relationToLeftDatabaseUniqueConstraintName: string;
  relationToRightDatabaseForeignKeyConstraintName: string;
};

export type GraphSchemaRelationOneToOneRelationTable<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "oneToOneRelationTable";
  name: string;
  leftEntityName: string;
  leftEntityFieldName: string;
  leftDatabaseColumnNames: string[];
  rightEntityName: string;
  rightEntityFieldName: string;
  rightDatabaseColumnNames: string[];
  relationDatabaseSchemaName: string;
  relationDatabaseTableName: string;
  relationLeftDatabaseColumnNames: string[];
  relationRightDatabaseColumnNames: string[];
  relationToLeftDatabaseForeignKeyConstraintName: string;
  relationToLeftDatabaseUniqueConstraintName: string;
  relationToRightDatabaseForeignKeyConstraintName: string;
  relationToRightDatabaseUniqueConstraintName: string;
};

///////////////////////////////////////////////////////////////////////////////

export type GraphSchemaEntity<
  TheTypeReference extends readonly TypeReferenceElement[]
> = GraphSchemaEntityDatabaseTable<TheTypeReference>;
// | GraphSchemaEntityDatabaseView<TheTypeReference>;

export type GraphSchemaEntityDatabaseTable<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  type: "databaseTable";
  name: string;
  fields: Array<GraphSchemaField<TheTypeReference>>;
  databaseSchemaName: string;
  databaseTableName: string;
  graphqlEntityTypeName: string;
  graphqlGetSingleOperationName: string;
  graphqlGetListOperationName: string;
  graphqlGetConnectionOperationName: string;
  graphqlFilterTypeName: string;
  graphqlDefaultOrderBy: string;
  graphqlDefaultFirst: number;
  graphqlDefaultOffset: number;
};

// export type GraphSchemaEntityDatabaseView<
//   TheTypeReference extends readonly TypeReferenceElement[]
// > = {
//     type: "databaseView";
//     name: string;
//     fields: Array<GraphSchemaField<TheTypeReference>>;
//     databaseSchema: string;
//     databaseView: string;
//     graphqlEntityType: string;
//     graphqlGetSingleOperationName: string;
//     graphqlGetListOperation: string;
//     graphqlGetConnectionOperation: string;
//     graphqlFilterTypeName: string;
//     graphqlDefaultOrderBy: string;
//     graphqlDefaultFirst: number;
//     graphqlDefaultOffset: number;
//   };

///////////////////////////////////////////////////////////////////////////////

export type GraphSchema<
  TheTypeReference extends readonly TypeReferenceElement[]
> = {
  databaseConnectionUri: {
    protocol: "postgres";
    user: string;
    password: string;
    host: string;
    port: number;
    database: string;
    schema?: string;
  };
  entities: Array<GraphSchemaEntity<TheTypeReference>>;
  relations: Array<GraphSchemaRelation<TheTypeReference>>;
};

///////////////////////////////////////////////////////////////////////////////
