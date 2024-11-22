Crisp is a toy programming language I'm writing for learning about compilers and Rust. It has a familiar C block structure syntax with Rust syntax for definitions but has a Lisp-style Polish Notation for expressions because I can't be bothered to write a Pratt parser.

## How to run
Test source files are found in `tests/`. Files have a `.cri` extension (pronounced 'cry', because that's what I metaphorically do every time I work on this; painful, but worth it for the experience, I guess). To compile files, run `cargo run -- tests/<filename>.cri`.

## Project to-dos
### Syntax analysis
- [x] Prototype a parser with [pest.rs](https://pest.rs).
- [ ] Expand grammar with comments.
- [ ] Expand grammar with a keyword modifier for tail recursive functions.
- [ ] Handwrite a lexer and a parser for outputting better error messages.

### Semantic analysis
- [x] Prototype scope resolution.
- [ ] Implement type checking.
- [ ] Implement type inference.
- [ ] Implement lowering.

### Code generation
- [ ] Prototype code generation.