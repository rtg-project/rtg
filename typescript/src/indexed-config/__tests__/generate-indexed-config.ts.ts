import { generateIndexedConfig } from "../generate-indexed-config";
import { connector } from "../../connectors/postgres";
import { graphSchema } from "../../__data__/config1";

test("graphschema-index-1", () => {
  expect(generateIndexedConfig({ graphSchema, connector }))
    .toMatchInlineSnapshot(`
    
  `);
});
