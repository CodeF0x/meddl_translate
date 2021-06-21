# meddl_translate

Translates German to Meddlfrängisch. If you don't know what that is, this is not for you.

Example:

```rust
fn main() {
  println!("{}", meddl_translate::translate("Hallo"));
}
```

There's also examples available:

```shell
$ cargo run --example hallo
```
```shell
$ cargo run --example langer-text
```

# Translations file
A dictionary can be found in the `src` directory.