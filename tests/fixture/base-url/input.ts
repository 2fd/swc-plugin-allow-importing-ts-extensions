import ImportDefault from 'default.ts'
import * as ImportAll from 'default.ts'
import { default as ImportDefaultAs, ImportValue } from 'default.ts'
import { type ImportType } from 'default.ts'
import type { ImportOnlyType } from 'types'
export * from 'default.ts'
export * as ExportAll from 'default.ts'
export { default as ExportDefaultAs, ExportValue } from 'default.ts'
export type { ExportOnlyType } from 'types'
const DynamicImport = await import('dynamic.ts')

export {
  ImportDefault,
  ImportAll,
  ImportDefaultAs,
  ImportValue,
  ImportOnlyType,
  ImportType,
  DynamicImport
}
