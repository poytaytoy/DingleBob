# Dinglebob

Dinglebob is a small, dynamically-typed interpreted language I wrote in Rust that evaluates in applicative order. It supports:

* variables (`let`)
* blocks / lexical scopes (`{ ... }`)
* `if / else`, `while`, and `for`
* functions (`define`) + lambdas (`lambda`)
* lists + indexing (`[ ... ]`, `xs[i]`)
* a handful of built-ins (`timeit`, `abs`, `len`, `copy`, `append`, `concat`)
* an `import` function for multi-file projects

**TODO**

* Implement OOP (Altho it technically kinda exists)
* Build a standard library

I recommend skimming the syntax guide [here](SYNTAX.md) -> especially the sections on **lists** and **closures**.

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

I also provided a `dummy.dingle`, so if you want to directly play with it, you can also modify the file and run from the root directory:  

```bash
./target/release/dinglebob dummy.dingle
```

## File extension

Use whatever extension you want (`.dingle`, `.dinglebob`, etc). It doesn't really matter but I preferably use `.dingle`.

---

## Examples

Dinglebob is pretty limited. There’s no object-oriented programming yet, and while it’s technically multi-paradigm, it definitely leans functional -> mostly because I built it right after finishing a functional programming course.

### Sorting

Enough yapping -> here’s how you could implement sorting (also in `examples/sort.dingle`):

```js
define quick_sort(xs) {
    if len(xs) <= 1 {
        return xs;
    }

    let pivot = xs[0];
    let less = [];
    let equal = [];
    let greater = [];

    for (let i = 0; i < len(xs); i = i + 1) {
        let val = xs[i];
        if val < pivot {
            append(less, val);
        } else {
            if val == pivot {
                append(equal, val);
            } else {
                append(greater, val);
            }
        }
    }

    return concat(concat(quick_sort(less), equal), quick_sort(greater));
}

let unsorted = [10, -1, 2, 5, 0, 9, 3];
print quick_sort(copy(unsorted)); # [-1, 0, 2, 3, 5, 9, 10]
```

Lists in Dinglebob behave like references in Python, so you’ll often want `copy()` if you need a separate instance.

You can also import this file and call it directly:

```js
import("examples/sort.dingle");

print quick_sort([10, -1, 2, 5, 0, 9, 3]);
```

---

### Structs and Classes (kinda)

Structs/classes aren’t implemented, but you can fake them thanks to closures capturing scope.

**Struct-like (also in `examples/structs.dingle`)**:

```js
define person(name, age) {
    return lambda(msg) {
        if msg == "name" { return name; }
        if msg == "age"  { return age; }
    };
}

let p = person("Alice", 20);

print p("name"); # Alice
print p("age");  # 20
```

**Class-like (also in `examples/class.dingle`)**:

```js
define BankAccount(owner, initial_balance) {
    let balance = initial_balance;
    let transactions = [];

    let deposit = lambda(amount) {
        balance = balance + amount;
        append(transactions, "Deposit: " + amount);
        return balance;
    };

    let withdraw = lambda(amount) {
        if amount > balance {
            print "Insufficient funds for " + owner;
            return none;
        }
        balance = balance - amount;
        append(transactions, "Withdraw: " + amount);
        return balance;
    };

    return lambda(message) {
        if message == "balance" { return balance; }
        if message == "owner"   { return owner; }
        if message == "deposit" { return deposit; }
        if message == "withdraw" { return withdraw; }
        if message == "history" { return copy(transactions); } 

        print "Method not found: " + message;
    };
}

let my_acc = BankAccount("poytaytoy", 1000);

my_acc("deposit")(500);
my_acc("withdraw")(200);
```

---

### Y-Combinator

Dinglebob evaluates in applicative order, so if you *really* want recursion for lambdas, you can use the Z-combinator (also in `examples/zcombinator.dingle`):

```js
let Y = lambda(f) {
    let g = lambda(x) {
        return f(lambda(v) { 
            return x(x)(v); 
        });
    };
    return g(g);
};

let factorial_gen = lambda(rec) {
    return lambda(n) {
        if n == 0 { return 1; }
        return n * rec(n - 1);
    };
};

let fact = Y(factorial_gen);

print fact(5); # 120
```

---

And yeah, that’s basically Dinglebob. There isn’t much practical uses -> it's just for me to learn more about programming languages. 
