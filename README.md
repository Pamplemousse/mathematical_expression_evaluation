# mathematical expression evaluation

**Disclaimer**
This is a project for me to learn Rust.
Do not make this your official calculator.


## Usage

```bash
math_eval <string to evaluate>

# example
math_eval 22+3*44
```

## Dev

```bash
nix-shell -p rustup

# testing
[nix-shell]$ cargo test

# coding style
[nix-shell]$ cargo clippy
```


## Resources

Heavily inspired from:
  * [How to build a math expression tokenizer using JavaScript](https://medium.freecodecamp.org/how-to-build-a-math-expression-tokenizer-using-javascript-3638d4e5fbe9)
  * [https://www.esimovmiras.cc/articles/](https://www.esimovmiras.cc/articles/)
  * [Shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
  * [AST interpreter](http://rosettacode.org/wiki/Compiler/AST_interpreter)
