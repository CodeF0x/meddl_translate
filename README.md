# meddl_translate

Translates German to Meddlfrängisch. If you don't know what that is, this is not for you.

Example:

```rust
fn main() {
  println!("{}", meddl_translate::translate("Hallo"));
}
```

There's also other examples available:

```shell
$ cargo run --example hello
```
```shell
$ cargo run --example long-text
```

### Exceptions

It's possible to exclude words that should not be translated, e. g. "den" by adding it to the "ignored" array in the translation file:

```json
"ignored": [
  "den"
]
```

To see it in action, run:

```shell
$ cargo run --example ignored
```

# Translations file
A dictionary can be found in the `src` directory.

## WebAssembly

The `translate` function is exported through [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) when you build for `wasm32-unknown-unknown`.

### Building

1. Install the WebAssembly target (once):

   ```shell
   rustup target add wasm32-unknown-unknown
   ```

2. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (or `cargo install wasm-pack`).

3. From this repository’s root, build the JavaScript + WASM bundle:

   ```shell
   wasm-pack build --target web --out-dir pkg
   ```

   This writes `pkg/` with the `.wasm` file, a JS glue module, and a `package.json`. Use `--target bundler` instead of `web` if you bundle with Vite, webpack, or similar.

### Using it in a web project

**Option A — ES modules (`--target web`)**

Copy `pkg/` into your site (or depend on the crate via a path/git dependency and run `wasm-pack build` in your pipeline). In a module script, load the WASM and call `translate`:

```javascript
import init, { translate } from './pkg/meddl_translate.js';

async function run() {
  await init(); // fetches and instantiates the WASM
  console.log(translate('Hallo Welt'));
}

run();
```

Serve the page over HTTP(S); opening `file://` URLs usually breaks WASM fetch. Use any static server (e.g. `npx serve`) during development.

**Option B — Bundler (`--target bundler`)**

Build with `wasm-pack build --target bundler --out-dir pkg`, then import the same module from your bundled app:

```javascript
import init, { translate } from './pkg/meddl_translate.js';

await init();
translate('Hallo');
```

Configure your bundler to treat `.wasm` as a WebAssembly asset if it does not already.
