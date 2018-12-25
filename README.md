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


## Resources

Heavily inspired from:
  * [How to build a math expression tokenizer using JavaScript](https://medium.freecodecamp.org/how-to-build-a-math-expression-tokenizer-using-javascript-3638d4e5fbe9)
  * [https://www.esimovmiras.cc/articles/](https://www.esimovmiras.cc/articles/)
  * [Shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
