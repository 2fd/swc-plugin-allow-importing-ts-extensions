import ImportCommonDefault from "./lib/commonjs.cjs";
import ImportModuleDefault from "./lib/module.mjs";
import * as ImportCommonAll from "./lib/commonjs.cjs";
import * as ImportModuleAll from "./lib/module.mjs";
import {
  default as ImportCommonDefaultAs,
  ImportCommonValue,
} from "./lib/commonjs.cjs";
import {
  default as ImportModuleDefaultAs,
  ImportModuleValue,
} from "./lib/module.mjs";
export * from "./lib/commonjs.cjs";
export * from "./lib/module.mjs";
export * as ExportCommonAll from "./lib/commonjs.cjs";
export * as ExportModuleAll from "./lib/module.mjs";
export {
  default as ExportCommonDefaultAs,
  ExportCommonValue,
} from "./lib/commonjs.cjs";
export {
  default as ExportModuleDefaultAs,
  ExportModuleValue,
} from "./lib/module.mjs";
const DynamicImport = await import("./lib/dynamic.js");
export {
  ImportCommonDefault,
  ImportModuleDefault,
  ImportCommonAll,
  ImportModuleAll,
  ImportCommonDefaultAs,
  ImportCommonValue,
  ImportModuleDefaultAs,
  ImportModuleValue,
  DynamicImport,
};
