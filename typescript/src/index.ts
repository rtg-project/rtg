// import { diff, validate, coverage } from "@graphql-inspector/core";
// import * as chokidar from "chokidar";
// import {
//   buildSchema,
//   printSchema,
//   graphql,
//   graphqlSync,
//   getIntrospectionQuery,
//   parse,
//   buildASTSchema,
//   GraphQLSchema
// } from "graphql";
// import {
//   createPostGraphileSchema,
//   watchPostGraphileSchema,
//   PgConfig,
//   PostGraphileCoreOptions
// } from "postgraphile-core";

// import {
//   find,
//   fromPairs,
//   map,
//   flow,
//   startsWith,
//   get,
//   filter,
//   includes,
//   endsWith
// } from "lodash/fp";
// import { promises } from "fs";
// import pluralize from "pluralize";
// import path from "path";

// const processSchema = async ({
//   realSchema,
//   datamodelSchema,
//   datamodelSchema2,
//   schemaDatamodelJsonPath,
//   schemaDatamodelPath,
//   schemaRealJsonPath,
//   schemaRealPath,
//   outputPath
// }: {
//   realSchema: GraphQLSchema;
//   datamodelSchema: GraphQLSchema;
//   datamodelSchema2: GraphQLSchema;
//   schemaDatamodelJsonPath: string;
//   schemaDatamodelPath: string;
//   schemaRealJsonPath: string;
//   schemaRealPath: string;
//   outputPath: string;
// }) => {
//   //////////////////////////////////////////////////////////////////////
//   // Write the real schema result
//   const realSchemaIntrospectionResult = graphqlSync(
//     realSchema,
//     getIntrospectionQuery()
//   );
//   await promises.writeFile(
//     schemaRealJsonPath,
//     JSON.stringify(realSchemaIntrospectionResult, null, 2)
//   );
//   await promises.writeFile(schemaRealPath, printSchema(realSchema));

//   //////////////////////////////////////////////////////////////////////
//   // Write the datamodel schema result
//   const datamodelSchemaIntrospectionResult = graphqlSync(
//     datamodelSchema,
//     getIntrospectionQuery()
//   );
//   await promises.writeFile(
//     schemaDatamodelJsonPath,
//     JSON.stringify(datamodelSchemaIntrospectionResult, null, 2)
//   );
//   await promises.writeFile(schemaDatamodelPath, printSchema(datamodelSchema));

//   //////////////////////////////////////////////////////////////////////
//   // Manipulate schemas a bit
//   // const schemaDiff = diff(realSchema, datamodelSchema);

//   const schemaDiff = diff(datamodelSchema2, datamodelSchema);
//   const usefulSchemaDiff = flow([
//     // filter(item => item.type === "TYPE_ADDED"),
//     // filter(item => !startsWith("_", item.path)),
//     // filter(item => !startsWith("_", item.path))
//   ])(schemaDiff);
//   const output = datamodelSchema2
//     .getType(datamodelSchema2.getTypeMap().Mission)
//     .inspect();

//   //////////////////////////////////////////////////////////////////////
//   // // First option, using fields of the Query type:
//   // const allQueryTypes = fromPairs(
//   //   map(
//   //     field => [field.name, field],
//   //     find(
//   //       graphqlType =>
//   //         graphqlType.name === realSchemaIntrospectionResult.data.__schema.queryType.name,
//   //       realSchemaIntrospectionResult.data.__schema.types
//   //     ).fields
//   //   )
//   // );
//   // const queryTypes = flow([
//   //   filter(theType => !endsWith("Connection", theType.name)), // Relay connections
//   //   filter(theType => !includes("By", theType.name)), // Relation tables
//   //   filter(
//   //     theType =>
//   //       !includes(theType.name, ["query", "subscription", "node", "nodeId"])
//   //   ), // Hardcoded types
//   //   filter(theType => !pluralize.isPlural(theType.name)) // Lists
//   // ])(allQueryTypes);
//   // const queryTypeNames = map(get("name"), queryTypes);

//   // //////////////////////////////////////////////////////////////////////
//   // // Second option, using types and the Node interface directly
//   // const allNodeTypes = filter(
//   //   theType =>
//   //     find(theInterface => theInterface.name === "Node", theType.interfaces),
//   //   realSchemaIntrospectionResult.data.__schema.types
//   // );
//   // const nodeTypes = flow([
//   //   filter(theType => !includes(theType.name, ["Query", "_RelayId"])) // Hardcoded types
//   // ])(allNodeTypes);
//   // const nodeTypeNames = map(get("name"), nodeTypes);
//   // // At this point we have all entities + all relation tables

//   //////////////////////////////////////////////////////////////////////
//   await promises.writeFile(outputPath, JSON.stringify(output, null, 2));
//   console.log("Finished building schema");
// };

// const getDatamodelSchema = async (
//   datamodelPath: string
// ): Promise<GraphQLSchema> => {
//   const datamodelString = await promises.readFile(datamodelPath, {
//     encoding: "utf8"
//   });
//   const completedDatamodelString = `
// scalar DateTime
// scalar Json

// directive @deprecated(
//   reason: String = "No longer supported"
// ) on FIELD_DEFINITION | ENUM_VALUE
// directive @createdAt on FIELD_DEFINITION
// directive @updatedAt on FIELD_DEFINITION
// directive @id on FIELD_DEFINITION
// directive @unique on FIELD_DEFINITION
// enum _RelationLink {
//   TABLE
//   INLINE
// }
// enum _RelationOnDelete {
//   SET_NULL
//   CASCADE
// }
// directive @relation(
//   name: String
//   link:_RelationLink = TABLE
//   onDelete:_RelationOnDelete = SET_NULL
// ) on FIELD_DEFINITION
// enum _ScalarListStrategy {
//   RELATION
// }
// directive @scalarList(
//   strategy:_ScalarListStrategy = RELATION
// ) on FIELD_DEFINITION
// directive @db(
//   name: String
// ) on FIELD_DEFINITION  | OBJECT | ENUM
// directive @default(
//   value: String
// ) on FIELD_DEFINITION
// directive @migrationValue(
//   value: String
// ) on FIELD_DEFINITION

// ${datamodelString}

// type Query {
//   _dummy: String
// }

//   `;

//   const datamodelSchema = buildSchema(completedDatamodelString);
//   // console.log(datamodelSchema);
//   return datamodelSchema;
// };

// const watchDatamodel = async (datamodelPath: string): Promise<void> => {
//   let running = false;
//   let runAgain = false;
//   const queue = async (): Promise<void> => {
//     if (running) {
//       runAgain = true;
//     }
//     running = true;
//     try {
//       await getDatamodelSchema(datamodelPath);
//     } catch (e) {
//       // eslint-disable-next-line no-console
//       console.error(e);
//     } finally {
//       running = false;
//       if (runAgain) {
//         runAgain = false;
//         queue();
//       }
//     }
//   };
//   const watcher = chokidar.watch(datamodelPath, {
//     /*
//      * Without `usePolling`, on Linux, you can prevent the watching from
//      * working by issuing `git stash && sleep 2 && git stash pop`. This is
//      * annoying.
//      */
//     usePolling: true,

//     /*
//      * Some editors stream the writes out a little at a time, we want to wait
//      * for the write to finish before triggering.
//      */
//     awaitWriteFinish: {
//       stabilityThreshold: 200,
//       pollInterval: 100
//     }
//   });
//   watcher.on("change", queue);
//   await queue();
//   return;
// };

// export const sterblueGraph = async ({
//   pgConfig,
//   schemas,
//   options,
//   datamodelPath,
//   schemaDatamodelJsonPath,
//   schemaDatamodelPath,
//   schemaRealJsonPath,
//   schemaRealPath,
//   outputPath
// }: {
//   pgConfig: PgConfig;
//   schemas: string | string[];
//   options: PostGraphileCoreOptions;
//   datamodelPath: string;
//   schemaDatamodelJsonPath: string;
//   schemaDatamodelPath: string;
//   schemaRealJsonPath: string;
//   schemaRealPath: string;
//   outputPath: string;
// }): Promise<void> => {
//   // Read the target datamodelSchema
//   const datamodelSchema = await getDatamodelSchema(datamodelPath);
//   const datamodelSchema2 = await getDatamodelSchema(datamodelPath);
//   ///////////////////////////////////////////////////////////////////
//   // // But then we only createPostGraphileSchema
//   const realSchema = await createPostGraphileSchema(pgConfig, schemas, options);
//   // // And process the result
//   // await processSchema(finalSchema,datamodel);
//   // ///////////////////////////////////////////////////////////////////
//   // // Or we watch schema and do the same as above but in watch mode
//   // await watchPostGraphileSchema(pgConfig, schemas, options, async newSchema => {
//   //   // // And process the result
//   //   await processSchema(newSchema, datamodelSchema);
//   // });
//   await processSchema({
//     realSchema,
//     datamodelSchema,
//     datamodelSchema2,
//     schemaDatamodelJsonPath,
//     schemaDatamodelPath,
//     schemaRealJsonPath,
//     schemaRealPath,
//     outputPath
//   });
// };
