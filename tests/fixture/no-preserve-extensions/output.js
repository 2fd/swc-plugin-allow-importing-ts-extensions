import ImportCommonDefault from "./lib/commonjs.js";
import ImportModuleDefault from "./lib/module.js";
import * as ImportCommonAll from "./lib/commonjs.js";
import * as ImportModuleAll from "./lib/module.js";
import {
  default as ImportCommonDefaultAs,
  ImportCommonValue,
} from "./lib/commonjs.js";
import {
  default as ImportModuleDefaultAs,
  ImportModuleValue,
} from "./lib/module.js";
export * from "./lib/commonjs.js";
export * from "./lib/module.js";
export * as ExportCommonAll from "./lib/commonjs.js";
export * as ExportModuleAll from "./lib/module.js";
export {
  default as ExportCommonDefaultAs,
  ExportCommonValue,
} from "./lib/commonjs.js";
export {
  default as ExportModuleDefaultAs,
  ExportModuleValue,
} from "./lib/module.js";
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
