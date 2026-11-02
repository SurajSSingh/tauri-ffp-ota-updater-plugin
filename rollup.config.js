import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { cwd } from 'node:process'

const pkg = JSON.parse(readFileSync(join(cwd(), 'package.json'), 'utf8'))

// `tsc` (TypeScript 7) emits the ESM build and type declarations into
// dist-js; rollup only derives the CommonJS build from that output.
export default {
  input: pkg.exports.import,
  output: {
    file: pkg.exports.require,
    format: 'cjs'
  },
  external: [
    /^@tauri-apps\/api/,
    ...Object.keys(pkg.dependencies || {}),
    ...Object.keys(pkg.peerDependencies || {})
  ]
}
