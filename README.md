# Dinglebob 

Dinglebob is a small interpreted language I made in Rust with:

* variables (`let`)
* blocks / lexical scopes (`{ ... }`)
* `if / else`, `while`, and `for`
* functions (`define`) + lambdas (`lambda`)
* lists + indexing (`[ ... ]`, `xs[i]`)
* a handful of built-in functions (`timeit`, `abs`, `len`, `copy`, `append`, `concat`)

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


## Quick syntax guide

### Comments

```dingle
# single-line comment
let x = 3; # trailing comment
```

### Statements + blocks

* Most statements end with `;` (`let`, assignments, `print`, `return`, `break`, expression statements)
* Blocks use `{ ... }` and create scope

```dingle
let x = 10;
print x;

{
    let y = 5;
    print y;
}
```

---

## Values / literals

```dingle
none
true
false
123        # Int
3.14       # Float
"hello"    # String
[1, 2, 3]  # List
```

Truthiness in conditions:

* falsey: `false`, `none`, `0`, `0.0`
* everything else is truthy

---

## Variables

```dingle
let x = 10;
let y;          # same as: let y = none;
x = x + 1;
```

Valid assignment targets:

```dingle
x = 3;
xs[i] = 5;
```

---

## Control flow (bodies MUST be blocks)

```dingle
if x > 0 {
    print "pos";
} else {
    print "no";
}

while x < 5 {
    x = x + 1;
}

for (let i = 0; i < 5; i = i + 1) {
    print i;
}

break;   # only inside loops
```

---

## Functions + lambdas

Named function:

```dingle
define add(a, b) {
    return a + b;
}
print add(2, 3);
```

Lambda:

```dingle
let f = lambda(x) { return x * x; };
print f(5);
```

Notes:

* `return;` returns `none`
* closures capture surrounding environment

---

## Lists / indexing

```dingle
let xs = [10, 20, 30];
print xs[1];   # 20
xs[1] = 999;
print xs;      # [10, 999, 30]
```

Rules:

* index must be an `Int`
* bounds checked (out of range = runtime error)
* lists are reference-like; use `copy()` to duplicate

---

## Built-ins

* `timeit() -> Float` (seconds since UNIX epoch)
* `abs(x) -> Float` (Int allowed, converted)
* `len(list) -> Int`
* `copy(list) -> List` (shallow copy)
* `append(list, value) -> List` (mutates + returns same list)
* `concat(list1, list2) -> List` (returns new list)

---

## Error stages

1. **Scanner**: bad characters, unterminated strings
2. **Parser**: malformed syntax (missing `;`, `}`, etc.)
3. **Resolver**: duplicate definitions in same scope
4. **Interpreter**: type errors, calling non-functions, bad indexing, invalid assignment targets

---

## Imports

Dinglebob supports importing and executing another file at runtime:

```dingle
import("test2.dingle");
```

What it does (based on your current implementation):

* Reads + executes the target file.
* Exports its top-level bindings into the current program after execution.
* Names starting with `_` are **not exported** (treated as “private”).

Example:

**test2.dingle**

```dingle
let x = 10;
let _secret = 999;

define inc(n) { return n + 1; }
```

**test.dingle**

```dingle
import("test2.dingle");

print x;       # 10
print inc(5);  # 6
print _secret; # runtime error (not imported)
```

