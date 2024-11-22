Crisp is a toy programming language I'm writing for learning about compilers and Rust. It has a familiar C block structure syntax with Rust syntax for definitions but has a Lisp-style Polish Notation for expressions because I can't be bothered to write a Pratt parser.

### How to run
Test source files are found in `tests/`. Files have a `.cri` extension (pronounced 'cry', because that's what I metaphorically do every time I work on this; painful, but worth it for the experience, I guess). To compile files, run `cargo run -- tests/<filename>.cri`.