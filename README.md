`cargo build` succeeds
`cargo test` fails
`rustc --test -v src/lib.rs` succeeds, but running `./lib` afterwards shows
no tests were actually run by rustc, so i dont think rustc uses
the rustdoc example code like cargo test does which is why cargo test
fails and rustc --test doesnt

