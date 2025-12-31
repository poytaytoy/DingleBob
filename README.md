# Dinglebob 

Dinglebob is a small interpreted language I made over the winter break with Rust:

* variables (`let`)
* blocks / lexical scopes (`{ ... }`)
* `if / else`, `while`, and `for`
* functions (`define`) + lambdas (`lambda`)
* lists + indexing (`[ ... ]`, `xs[i]`)
* a handful of built-in functions (`timeit`, `abs`, `len`, `copy`, `append`, `concat`)
* an `import` function that allows for multi-file projects 

If you really wanna know the syntax, checkout [here](SYNTAX.md)
--- 

## Running

### Build a binary

From the Rust project root:

```bash
cargo build --release
```

Your binary will be at:

```bash
./target/release/dinglebob
```

### Run modes

**REPL**

```bash
./target/release/dinglebob
```

**Run a file**

```bash
./target/release/dinglebob path/to/program.dingle
```

---

## File extension

Use whatever you want (`.dingle`, `.dinglebob`, etc). Examples below assume `.dingle`.



