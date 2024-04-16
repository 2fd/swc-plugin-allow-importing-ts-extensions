import ImportDefault from './lib/default.ts'
import * as ImportAll from './lib/default.ts'
import { default as ImportDefaultAs, ImportValue } from './lib/default.ts'
import { type ImportType } from './lib/default.ts'
import type { ImportOnlyType } from './lib/types'
export * from './lib/default.ts'
export * as ExportAll from './lib/default.ts'
export { default as ExportDefaultAs, ExportValue } from './lib/default.ts'
export type { ExportOnlyType } from './lib/types'
const DynamicImport = await import('./lib/dynamic.ts')

export {
  ImportDefault,
  ImportAll,
  ImportDefaultAs,
  ImportValue,
  ImportOnlyType,
  ImportType,
  DynamicImport
}
