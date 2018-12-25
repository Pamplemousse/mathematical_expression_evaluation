# mathematical expression evaluation

**Disclaimer**
This is a project for me to learn Rust.
Do not make this your official calculator.


## Usage

```bash
cargo run <string to evaluate>

# example
cargo run 22+3*44
```

## Dev

```bash
nix-shell -p cargo rustc
```


## Possible features

  * functions: pi(), sin(), cos()
  * floating points numbers: 3.14
  * validate input (verify matching parenthesis, verify integer size etc.)
  * help

### Refactor

  * use [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait (in `src/tokenizer/token/mod.rs`, `src/tokenizer/token/operator.rs`)
  * use [`Deref` and `DerefMut`]() in particular in the Shunting Yard algorithm
  * implement some property based testing


## Resources

Heavily inspired from:
  * [How to build a math expression tokenizer using JavaScript](https://medium.freecodecamp.org/how-to-build-a-math-expression-tokenizer-using-javascript-3638d4e5fbe9)
  * [https://www.esimovmiras.cc/articles/](https://www.esimovmiras.cc/articles/)
  * [Shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
  * [AST interpreter](http://rosettacode.org/wiki/Compiler/AST_interpreter)
