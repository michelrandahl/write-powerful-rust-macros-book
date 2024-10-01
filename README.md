# Notes, implementations and exercises for "Write Powerful Rust Macros"

## Source Rust environments with Nix
`./stable` contains a stable (fixed channel version) Rust environment.

`./unstable` contains an unstable Rust environment.

Enter one of the directories and run `nix develop` to source the environment into your current terminal.

## Macro development troubleshooting
- Always create an example project where you test out the macros, because `cargo build` in the macro project will not be enough to tell you about all the errors there might be in your macro definitions.
- Write out an example of what you want to achieve with the macro.

### print-debug macros
Get more insight into the macro ast datastructure by printing them out. Use error printing to make sure no output is swallowed or discarded.
```
#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", &ast);
    ...
```

### expand macros
install `cargo-expand` globally.
```
$ cargo install cargo-expand
```
In the project where you are applying the macro, run
```
cargo expand
```
