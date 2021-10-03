// import pgPromise from "pg-promise";
import * as sql from "pg-sql2";

import {
  GraphSchemaField,
  GraphSchemaEntityDatabaseTable,
  GraphSchemaFieldScalarDatabaseColumn,
  ArrayElement,
  GraphSchema,
  GraphSchemaFieldRelationOne,
  GraphSchemaFieldRelationMany,
  GraphSchemaRelationOneToOneForeignKey,
  GraphSchemaRelationOneToManyForeignKey,
  GraphSchemaRelationManyToManyRelationTable,
  GraphSchemaRelationOneToManyRelationTable,
  GraphSchemaRelationOneToOneRelationTable
} from "../graphschema/types";

import {
  TypeReferenceElement,
  TypeReferenceElementPreferred,
  TypeReferenceElementSupported,
  TypeReferenceElementUnsupported
} from "../connectors/types";

///////////////////////////////////////////////////////////////////////////////

// export type IndexedTypeReferenceElement<TheTypeReferenceElement extends TypeReferenceElement> =
export type IndexedTypeReferenceElement<
  TheTypeReferenceElement extends TypeReferenceElement
> = TheTypeReferenceElement; // & {};
// | IndexedTypeReferenceElementPreferred
// | IndexedTypeReferenceElementSupported
// | IndexedTypeReferenceElementUnsupported;

// export type IndexedTypeReferenceElementPreferred = TypeReferenceElementPreferred; // & {};

// export type IndexedTypeReferenceElementSupported = TypeReferenceElementSupported; // & {};

// export type IndexedTypeReferenceElementUnsupported = TypeReferenceElementUnsupported; // & {};

export type IndexedConnector<
  TheTypeReferenceElement extends TypeReferenceElement
> = {
  typeReferenceByDatabaseTypeName: {
    [key: string]: IndexedTypeReferenceElement<TheTypeReferenceElement>;
  };
  typeReferenceByGraphqlTypeNameAndGraphqlAnnotation: {
    [key: string]: {
      [key: string]: IndexedTypeReferenceElement<TheTypeReferenceElement>;
    };
  };
};

///////////////////////////////////////////////////////////////////////////////

export type IndexedGraphSchemaField<
  TheTypeReferenceElement extends TypeReferenceElement
> =
  | IndexedGraphSchemaFieldScalarDatabaseColumn<TheTypeReferenceElement>
  | IndexedGraphSchemaFieldRelationOne<TheTypeReferenceElement>
  | IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;

export type IndexedGraphSchemaFieldScalarDatabaseColumn<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaFieldScalarDatabaseColumn<readonly TheTypeReferenceElement[]> & {
  databaseType: TheTypeReferenceElement;
};

export type IndexedGraphSchemaFieldRelationOne<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaFieldRelationOne<readonly TheTypeReferenceElement[]> & {
  relation: IndexedGraphSchemaRelation<TheTypeReferenceElement>;
  relationRole: "left" | "right";
};

export type IndexedGraphSchemaFieldRelationMany<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaFieldRelationMany<readonly TheTypeReferenceElement[]> & {
  relation: IndexedGraphSchemaRelation<TheTypeReferenceElement>;
  relationRole: "left" | "right";
};

///////////////////////////////////////////////////////////////////////////////

export type IndexedGraphSchemaRelation<
  TheTypeReferenceElement extends TypeReferenceElement
> =
  | IndexedGraphSchemaRelationOneToOneForeignKey<TheTypeReferenceElement>
  | IndexedGraphSchemaRelationOneToManyForeignKey<TheTypeReferenceElement>
  | IndexedGraphSchemaRelationManyToManyRelationTable<TheTypeReferenceElement>
  | IndexedGraphSchemaRelationOneToManyRelationTable<TheTypeReferenceElement>
  | IndexedGraphSchemaRelationOneToOneRelationTable<TheTypeReferenceElement>;

export type IndexedGraphSchemaRelationOneToOneForeignKey<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaRelationOneToOneForeignKey<
  readonly TheTypeReferenceElement[]
> & {
  leftEntity: IndexedGraphSchemaFieldRelationOne<TheTypeReferenceElement>;
  rightEntity: IndexedGraphSchemaFieldRelationOne<TheTypeReferenceElement>;
};

export type IndexedGraphSchemaRelationOneToManyForeignKey<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaRelationOneToManyForeignKey<
  readonly TheTypeReferenceElement[]
> & {
  leftEntity: IndexedGraphSchemaFieldRelationOne<TheTypeReferenceElement>;
  rightEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
};

export type IndexedGraphSchemaRelationManyToManyRelationTable<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaRelationManyToManyRelationTable<
  readonly TheTypeReferenceElement[]
> & {
  leftEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
  rightEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
};

export type IndexedGraphSchemaRelationOneToManyRelationTable<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaRelationOneToManyRelationTable<
  readonly TheTypeReferenceElement[]
> & {
  leftEntity: IndexedGraphSchemaFieldRelationOne<TheTypeReferenceElement>;
  rightEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
};

export type IndexedGraphSchemaRelationOneToOneRelationTable<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaRelationOneToOneRelationTable<
  readonly TheTypeReferenceElement[]
> & {
  leftEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
  rightEntity: IndexedGraphSchemaFieldRelationMany<TheTypeReferenceElement>;
};

///////////////////////////////////////////////////////////////////////////////

export type IndexedGraphSchemaEntity<
  TheTypeReferenceElement extends TypeReferenceElement
> = IndexedGraphSchemaEntityDatabaseTable<TheTypeReferenceElement>;
// | IndexedGraphSchemaEntityDatabaseView<TheTypeReferenceElement>;

export type IndexedGraphSchemaEntityDatabaseTable<
  TheTypeReferenceElement extends TypeReferenceElement
> = GraphSchemaEntityDatabaseTable<readonly TheTypeReferenceElement[]> & {
  fieldsByName: Record<
    string,
    IndexedGraphSchemaField<TheTypeReferenceElement>
  >;
  fieldsByDatabaseColumnName: Record<
    string,
    IndexedGraphSchemaField<TheTypeReferenceElement>
  >;
  fieldsByGraphqlFieldName: Record<
    string,
    {
      operationKind: "getSingle" | "getList" | "getConnection";
      field: IndexedGraphSchemaField<TheTypeReferenceElement>;
    }
  >;
  fieldsByGraphqlOrderBy: Record<
    string,
    {
      direction: "ASC" | "DESC";
      field: IndexedGraphSchemaField<TheTypeReferenceElement>;
    }
  >;
};

// export type IndexedGraphSchemaEntityDatabaseView<
//   TheTypeReferenceElement extends TypeReferenceElement
// > = GraphSchemaEntityDatabaseView<TheTypeReferenceElement> & {
//     indexedFields: Array<IndexedGraphSchemaField<TheTypeReferenceElement>>;
//   };

///////////////////////////////////////////////////////////////////////////////

export type IndexedGraphSchema<
  TheTypeReferenceElement extends TypeReferenceElement
> = {
  entitiesByOperationName: Record<
    string,
    {
      operationKind: "getSingle" | "getList" | "getConnection";
      entity: IndexedGraphSchemaEntity<TheTypeReferenceElement>;
    }
  >;
  entitiesByName: Record<
    string,
    IndexedGraphSchemaEntity<TheTypeReferenceElement>
  >;
  relationsByName: Record<
    string,
    IndexedGraphSchemaRelation<TheTypeReferenceElement>
  >;
};

///////////////////////////////////////////////////////////////////////////////

export type IndexedConfig<
  TheTypeReferenceElement extends TypeReferenceElement
> = {
  indexedConnector: IndexedConnector<TheTypeReferenceElement>;
  indexedGraphSchema: IndexedGraphSchema<TheTypeReferenceElement>;
};

///////////////////////////////////////////////////////////////////////////////
