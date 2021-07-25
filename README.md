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