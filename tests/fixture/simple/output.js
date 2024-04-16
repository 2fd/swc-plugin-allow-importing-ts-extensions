import ImportDefault from "./lib/default.js";
import * as ImportAll from "./lib/default.js";
import { default as ImportDefaultAs, ImportValue } from "./lib/default.js";
export * from "./lib/default.js";
export * as ExportAll from "./lib/default.js";
export { default as ExportDefaultAs, ExportValue } from "./lib/default.js";
const DynamicImport = await import("./lib/dynamic.js");
export {
  ImportDefault,
  ImportAll,
  ImportDefaultAs,
  ImportValue,
  DynamicImport,
};
