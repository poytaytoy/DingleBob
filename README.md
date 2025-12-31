# Dinglebob 

Dinglebob is a small dynamically typed interpretted language that I made with Rust:

* variables (`let`)
* blocks / lexical scopes (`{ ... }`)
* `if / else`, `while`, and `for`
* functions (`define`) + lambdas (`lambda`)
* lists + indexing (`[ ... ]`, `xs[i]`)
* a handful of built-in functions (`timeit`, `abs`, `len`, `copy`, `append`, `concat`)
* an `import` function that allows for multi-file projects 

TODO: 

* Implement OOP
* Make a standard library

I recommend you skim over the syntax from [here](SYNTAX.md), especially the part on lists and closure. 

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

## File extension

Use whatever you want (`.dingle`, `.dinglebob`, etc). Examples below assume `.dingle`.

---

## Examples 

The dinglebob language is pretty limited. It doesn't have objected oriented programming, and while it's technically a multi-paradigm language, it leans towards a functional programming language. This is a natural result of just completing a functional programming course. 

### Sorting

Enough bantering and here's how you would implement something like sorting. (also on `exmaples/sort.dingle`)

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

Lists in dinglebob are like references in python. You need to copy it in order to create a seperate instnace of it. Hence, you see `copy` being used. You can also access this code via: 

```js
import ("examples/sort.dingle");

print quick_sort([10, -1, 2, 5, 0, 9, 3])
```

---

### Structs and Classes

That aside, while `structs` and `classes` are not implemented, you can mimic them because of how dinglebob captures the entire environment scope with its closures on funcitons. For structs (also on `examples/structs.dingle`): 

```js
define person(name, age) {
    return lambda(msg) {
        if msg == "name" { return name; }
        if msg == "age"  { return age; }
    };
}

let p = person("Alice", 20);

print p("name"); # Alice 
print p("age"); # 20
```

Meanwhile, for classes (also on `examples/class.dingle`):

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
        if message == "history" { return transactions; }
        
        print "Method not found: " + message;
    };
}

let my_acc = BankAccount("d2i-23", 1000);

my_acc("deposit")(500);
my_acc("withdraw")(200);

```

### Y-Combinator

To end things off, I also learned a lot of lambda calculus, so if for whatever reason you want to implement recursion, for lambda functions, the Y-Combinator can be implemented like this (also on examples/ycombinator.dingle): 

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

And that's about it for dinglebob, there isn't really much practical usage and I made it for fun to learn more about programming languages. 

