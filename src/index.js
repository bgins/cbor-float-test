import { encode } from "cbor-x";
import * as fs from "node:fs";

const json = {
  // eslint-disable-next-line unicorn/no-zero-fractions
  name: 5.0,
};

const serializedAsBuffer = encode(json);
fs.writeFileSync("output/test.cbor", serializedAsBuffer);
