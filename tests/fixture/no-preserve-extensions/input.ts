import ImportCommonDefault from './lib/commonjs.cts'
import ImportModuleDefault from './lib/module.mts'
import * as ImportCommonAll from './lib/commonjs.cts'
import * as ImportModuleAll from './lib/module.mts'
import { default as ImportCommonDefaultAs, ImportCommonValue } from './lib/commonjs.cts'
import { default as ImportModuleDefaultAs, ImportModuleValue } from './lib/module.mts'
import { type ImportCommonType } from './lib/commonjs.cts'
import { type ImportModuleType } from './lib/module.mts'
import type { ImportOnlyType } from './lib/types'
export * from './lib/commonjs.cts'
export * from './lib/module.mts'
export * as ExportCommonAll from './lib/commonjs.cts'
export * as ExportModuleAll from './lib/module.mts'
export { default as ExportCommonDefaultAs, ExportCommonValue } from './lib/commonjs.cts'
export { default as ExportModuleDefaultAs, ExportModuleValue } from './lib/module.mts'
export type { ExportOnlyType } from './lib/types'
const DynamicImport = await import('./lib/dynamic.ts')

export {
  ImportCommonDefault,
  ImportModuleDefault,
  ImportCommonAll,
  ImportModuleAll,
  ImportCommonDefaultAs,
  ImportCommonValue,
  ImportModuleDefaultAs,
  ImportModuleValue,
  ImportCommonType,
  ImportModuleType,
  ImportOnlyType,
  DynamicImport
}
