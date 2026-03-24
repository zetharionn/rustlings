# Variables
Variables are *containers* for **data**. They can be used to store any data.

## Binding
In Rust, variables are an abstraction over **binding** values to names. To bind a value to a name, use `let` keyword **before** variable name. The most common syntax for a binding is:
```
let name = value;
```

### Initialization
Initialization is process of **binding** an *initial* value to a name. Without initialization, a variable **cannot** be used. Attempting to use an uninitialized variable will cause a compilation error.

## Mutability
In Rust, variables are **immutable** by **default**. This means that once a value is **bound** to a name, it **cannot** be changed. If you try to assign a new value to the same name, you'll encounter an immutability error. However, there's still an option to make variables **mutable** by adding `mut` **before** the variable name. After that, assigning a new value to the same name won't cause an immutability error.

```
let mut name = value;
```

## Constants
Constants are values that are bound to a name and aren't allowed to change. To create a CONSTANT, use `const` keyword **before** CONSTANT name. They're very similar to immutable variables, but there are some differences between them:
- There's **no** option to make constants mutable. You **can't** use `mut` with constants. Constants aren't just immutable by default - they're **always** immutable.
- The type of the CONSTANT's value **must** be annotated.
- Constants **must** be initialized with a value.
- Constants can be declared in **any** scope, including global scope, which makes them **useful** for *reusable* values that many parts of code need to know about.
- Constants are valid for the **entire** time a program runs, within the scope in which they were declared.
- According to Rust's naming convention, constants' names should be in **uppercase** with **underscores** between words.

```
const NAME: type = value;
```
