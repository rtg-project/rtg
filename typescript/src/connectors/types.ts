///////////////////////////////////////////////////////////////////////////////

export type TypeReferenceElement =
  | TypeReferenceElementPreferred
  | TypeReferenceElementSupported
  | TypeReferenceElementUnsupported;

export type TypeReferenceElementPreferred = {
  type: "preferred";
  databaseTypeName: string;
  graphqlTypeName: string;
  graphqlAnnotation: string;
};

export type TypeReferenceElementSupported = {
  type: "supported";
  databaseTypeName: string;
  graphqlTypeName: string;
  graphqlAnnotation: string;
};

export type TypeReferenceElementUnsupported = {
  type: "unsupported";
  databaseTypeName: string;
  graphqlTypeName?: string;
  graphqlAnnotation?: string;
};

export type TypeReference = TypeReferenceElement[];

export type Connector = {
  typeReference: TypeReference;
};
