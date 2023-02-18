# json-parser

This is a simple project to parse JSON without using any third party dependencies.

---

### Status

There is no error handling at the moment and it is by no means a fully compliant json parser.

## Usage

---

To install it using cargo:

```sh
cargo install --git "https://github.com/alvarojimenez95/json-parser.git"
```

Then, to parse a json file simply type:

```sh
json-parser -f "<YOUR FILE>"
```

To display the AST of the json, add a `-d` flag at the end to ouput it into the console.
