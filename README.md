# swc-plugin-allow-importing-ts-extensions

SWC Plugin for allowImportingTsExtensions

## When and Why

If you are using SWC to transpile your TypeScript code, and your target is `es2016` or higher, you can use this plugin to import `.ts` files extensions (instead of `.js`), and it will be transpiled to `.js` files.

## Usage

First, setup your `tsconfig.json` to allow importing `.ts` files extensions.

```json5
// tsconfig.json
{
  compilerOptions: {
    // ...
    noEmit: true,
    allowImportingTsExtensions: true,
  },
  // ...
}
```

Then, add the plugin to your `.swcrc` file.

```json5
// .swcrc
{
  $schema: "http://json.schemastore.org/swcrc",
  jsc: {
    experimental: {
      plugins: [["swc-plugin-allow-importing-ts-extensions", {}]],
    },
  },
}
```

## Options

```json5
// .swcrc
{
  // ...
  plugins: [
    [
      "swc-plugin-allow-importing-ts-extensions",
      {
        preserveImportExtension: true, // default: false
      },
    ],
  ],
  // ...
}
```

| option                    | type      | default | description                                              |
| ------------------------- | --------- | ------- | -------------------------------------------------------- |
| `preserveImportExtension` | `boolean` | `false` | If `true` all imports with extension `.mts` and `.cts` with be map to `.mjs` and `.cjs` respectively, otherwise it will map to `.js` |
