# Snek Lang Parser

This is a parser implementation for snek-lang using [pest](https://pest.rs).

Snek-Lang is a programming language that I'm working on and might never finish.
I don't know where this will lead me.


## TODO (parser)

- [ ] constructing complex type instances
- [ ] anonymous types
- [ ] pattern matching
- [ ] if-expressions
- [ ] match-expressions
- [ ] imports and exports

## Snek-Lang (so far)

### Primitives

```sk
100            // Int
1.0, .1        // Float
true, false    // Bool
0xFF, 0b100    // Byte
"Hello World"  // Str
```

### Expressions

Every expression has to evaluate to a value

```sk
// infix addition
1 + 1                

// function calls
add2 4               

// nested expressions
1 * ( 1 - 1 )        

// nested function calls
add2 (sub4 (mul2 4)) 

// if is an expression that can return a value
let value = 
  if a == b then
    a
  else
    b
```

### Declarations

A declaration declares a variable. When listing arguments in that
declaration those arguments have to be provided when accessing the variable (see function call above).
Declarations without variables are only evaluated once (Probably but I have to think about that).

```sk
// simple declaration
let a = 5

// add arguments (converting it into a function)
let add2 n = n + 2

// nested statements with blocks
let complex_addition a b c = {
  let d = a + b
  let e = b + c
  d + e + 1
}

// type annotations
let a: Num = 1

// functions
let add3 a: Num -> Num = a + 3

// multi arg
let add_all a b c: Num -> Num -> Num -> Num =
  a + b + c 
```

### Complex types

The value for a type is one of the following:

- `(|...|...)` enum
- `{...}` record
- `#(...)` tuple

These expressions can be nested.

```sk
type MyEnum = (
  | Var1 Int
  | Var2 Bool
)

type MyRec = {
  field1: Float
  field2: Str
}

// records declared in one line need to put a comma after each field declaration
type MyRecOneline = {field1: Float, field2: Str}

// so need tuples
type MyTuple = #(Float, Str)

type Nested = {
  field1: (
    | Boolean Bool
    | Number Float
  )
  field2: #(Bool, Bool)
  field3: {
    first: Int
    second: Int
  }
}
```

