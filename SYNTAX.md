## Comments

Single-line comments start with `#` and go to the end of the line:

```dingle
# this is a comment
let x = 3; # comment after code
```

---

## Syntax basics

### Statements end with `;`

Most “single-line” statements must end in a semicolon:

* variable declarations (`let`)
* expression statements
* `print`
* `return`
* `break`

Example:

```dingle
let x = 10;
print x + 2;
x = x + 1;
```

### Blocks use braces `{ ... }`

Blocks create a new scope:

```dingle
{
    let x = 5;
    print x;
}
```

---

## Types & literals

### `none`

Represents “no value”:

```dingle
let x = none;
```

### Booleans

```dingle
true
false
```

### Numbers (int / float)

* integers parse as `Int`
* decimals parse as `Float`

```dingle
let a = 123;
let b = 3.1415;
```

> Note: the scanner will happily accept weird digit sequences like `0..3` as a “number token”, but parsing to int/float will fail later.

### Strings

Double quotes:

```dingle
let s = "hello";
print s;
```

Strings can include newlines.

### Lists

List literal:

```dingle
let xs = [1, 2, 3];
let empty = [];
```

Note that lists are essentially references like in python, so: 

```
let xs = [1, 2, 3]; 
let a = xs; 

a[1] = "bob"; 

print xs; #[1, "bob", 3]
print a; #[1, "bob", 3]
```

Use `copy` as documeted below to create two seperate instances of a list.

---

## Truthiness rules

In conditions (`if`, `while`, logical ops):

* `false` → false
* `none` → false
* `0` → false
* `0.0` → false
* everything else → true

---

## Operators

### Arithmetic

* `+  -  *  /  %`

### Comparison

* `>  >=  <  <=  ==  !=`

### Logical

* `and`
* `or`
* unary `!`

### Precedence (high → low)

1. indexing: `xs[i]`
2. calls: `f(x)`
3. unary: `!x`, `-x`
4. `* / %`
5. `+ -`
6. comparisons: `< <= > >=`
7. equality: `== !=`
8. `and`
9. `or`
10. assignment: `=`

---

## Variables

### Declare

```dingle
let x = 10;
let y;        # is just a syntactic sugar for y = none; 
```

### Assign

```dingle
x = x + 1;
```

Assignment targets can be:

* a variable: `x = ...`
* a list index: `xs[i] = ...`

---

## Control flow

### `if / else`

No parentheses required around the condition. **Bodies must be blocks.**

```dingle
if x > 0 {
    print "positive";
} else {
    print "non-positive";
}
```

### `while`

Condition expression followed by a block:

```dingle
while x < 5 {
    print x;
    x = x + 1;
}
```

### `for`

Form:

```dingle
for ( initializer ; condition ; increment ) { ... }
```

* `initializer` can be:

  * empty: `;`
  * `let ...`
  * expression statement
* `condition` optional (missing means `true`)
* `increment` optional
* body **must** be a block

Example:

```dingle
for (let i = 0; i < 5; i = i + 1) {
    print i;
}
```

Internally this desugars into a `while` loop.

### `break`

Only valid inside loops:

```dingle
while true {
    break;
}
```

---

## Functions

### Named functions (`define`)

Form:

```dingle
define name(param1, param2, ...) {
    ...
}
```

Example:

```dingle
define add(a, b) {
    return a + b;
}

print add(2, 3);
```

### `return`

* `return expr;`
* or `return;` (returns `none`)

Return is only allowed inside functions (top-level `return` is an interpreter error).

---

## Lambdas

Lambdas are expressions:

```dingle
let f = lambda(x) {
    return x * x;
};

print f(5);
```

They capture the current environment (closure-like behavior).

---

## Lists & indexing

### Index read

```dingle
let xs = [10, 20, 30];
print xs[1];     # 20
```

### Index write

```dingle
xs[1] = 999;
print xs;        # [10, 999, 30]
```

Index rules:

* index must be an `Int`
* bounds are checked (out-of-range is an interpreter error)

---

## Built-in functions

These exist in the global environment:

### `timeit() -> Float`

Returns seconds since UNIX epoch (useful for timing):

Example usage

```

let initial_time = timeit(); 

#Code to time 

print "Time" + (timeit() - intial_time);

```

### `abs(x) -> Float`

* accepts Float (and also Int turns to Float)

```dingle
print abs(-3);
print abs(-3.5);
```                                                                                                                     

### `len(list) -> Int`

```dingle
print len([1,2,3]);  # 3
```

### `copy(list) -> List`

Makes a new list containing cloned elements:

```dingle
let a = [1,2];
let b = copy(a);
b[0] = 999;
print a; # [1,2]
print b; # [999,2]
```

### `append(list, value) -> List`

Mutates the list and returns it:

```dingle
let xs = [1];
append(xs, 2);
print xs;  # [1,2]
```

### `concat(list1, list2) -> List`

Returns a new list:

```dingle
print concat([1,2], [3,4]); # [1,2,3,4]
```

---

## Errors

Dinglebob has 4 main pipeline errors 

1. **Scanner errors**: invalid characters, unterminated strings
2. **Parser errors**: missing semicolons, missing braces, malformed syntax
3. **Semantic/Resolver errors**: duplicate definitions in the same scope, etc
4. **Interpreter errors**: type errors, calling non-functions, index errors, invalid assignment target, etc.

--- 

## Imports

Dinglebob supports importing and executing another file at runtime:

```dingle
import("test2.dingle");
```

What it does: 

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

