# Rust Toolbox
A Rust library of useful code snippets.

## random_special
Generates a random special character.
``` rust
use toolbox::{random_special};

fn main() {
  let my_char: char = random_special();
  println!("'{}'", my_char);
}
```
