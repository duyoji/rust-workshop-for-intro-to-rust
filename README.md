## Setup environment

### Install cargo

`cargo` is package manager of Rust. It is same as `npm` or `yarn` of JavaScript.


Open terminal, put following command and follow the process.

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Select "1" When Showing up following.

```bash
Current installation options:

   default host triple: x86_64-apple-darwin
     default toolchain: stable
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

- Don't forget adding path of cargo showing up on terminal.
- After finishing this process, you can use `cargo` command.



### Run main program

```bash
$ cargo run
```

#### To run programs.

- Add `main.rs` in src directory.
- Create function named `main` in main.rs.
  - "`fn main {}`" is a entry point.

### Run test

```bash
$ cargo test
```

- A function that is annotated with `#[test]` is executed as a test code when running `$ cargo test`.
- Put the annotation before `fn` keyword.


## Basic Requirements


- [ ] Translate below JavaScript code into Rust.
  - Hint: If you stuck with something from above code, you can refer [slides of this lecture](https://docs.google.com/presentation/d/1aG7Uq4D9LPW7FKsi6ImRQ2_SabCbOJcDhABhkb9-vao/edit?usp=sharing).
- [ ] Difine type of return value and arugments on each function/method.
- [ ] Write tests for each function/method.
  - Hint: Do you remember annotation for tests.

```javascript
function main() {
  const number_1 = 1;
  console.log(number_1);

  let number_2 = 1;
  number_2 += 1;
  console.log(number_2);

  const number_3 = add(1, 2);
  console.log(number_3);

  output('this is input value');

  const person = new Person(
    'Code',
    'Chrysalis',
    2
  );
  console.log(person);
  person.greet();
}

function add(a, b) {
  return a + b;
}

function output(input) {
  console.log(input);
}

class Person {
  constructor(firstName, lastName, age) {
    this.firstName = firstName;
    this.lastName = lastName;
    this.age = age;
  }

  greet() {
    console.log(
      "My name is %s %s. I'm %d years old.",
      this.firstName,
      this.lastName,
      this.age
    );
  }
}
```

## Advanced Requirements.

- [ ] Serialize and deserialize your person instance with [Serde](https://github.com/serde-rs/serde).
- [ ] Separate functions and a struct into other files.
  - Hint: [Crates and Modules](https://doc.rust-lang.org/book/first-edition/crates-and-modules.html)


## Resources

- [Slides for today's lecture](https://docs.google.com/presentation/d/1aG7Uq4D9LPW7FKsi6ImRQ2_SabCbOJcDhABhkb9-vao/edit?usp=sharing)
- [Install Rust](https://www.rust-lang.org/en-US/install.html)
- [Serde](https://github.com/serde-rs/serde)
- [Crates and Modules](https://doc.rust-lang.org/book/first-edition/crates-and-modules.html)