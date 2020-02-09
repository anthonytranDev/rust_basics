# Notes

## Compile time error - linker missing

Some common Rust packages depend on C code and will need a C compiler. Therefore, it might be worth installing one now.

## Rust prelude

Module std::prelude

Rust comes with a variety of things in its standard library. However, if you had to **manually** import every single thing that you used, it would be very verbose.

On a technical level, Rust inserts

`extern crate std;Run`

into the crate root of every crate, and

`use std::prelude::v1::*;Run`

Current prelude contents - https://doc.rust-lang.org/std/prelude/

## Typings (data types & structures) 

Rust is a statically typed language, which means that it must know the types of all variables at compile time. 

1. Scalar \[Incomplete\]
   - Integer Types
   - Floating point Types
   - Character Type
2. Compound Types \[Incomplete\]
   - Tuple Type
   - Array Type
3. Structs
   - Tuple Structs
4. Collections \[Incomplete\]
   - Vector
   - String
   - Hash map
5. Enums
6. Variants
7. Types
8. Traits \[Incomplete\]
9.  Lifetimes \[Incomplete\]
10. Additional Notes

Removed `Generics` as concept of the way it works is the same as TypeScript

**Summarise**

### Structs

Struct contains possible fields

The instance of the struct is mutable

Struct instances can be used to create other Struct instances. Utils such as the double dot "..", placed at the bottom of the struct. This enables the Struct to determine what field is being used, whilst spreading the rest to the unused fields

### Tuple Structs

Very useful when you want to give your tuple a name for it very own type

### Variants

Variant is a type??

Variants can be an enum value

### Enums

Enum is a type

Enum values can only be one of its variants, at a time

Enum contains possible variants - called an enum **variant**

Enums enable us to easily define a function as one of its variant - in other words functions may take in multiple variants of the same enum

### Additional notes

Both Enums and 

### Sources
- **Structs**
  - A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
  - If you’re familiar with an object-oriented language, a struct is like an object’s data attributes
  - Note that the entire instance must be mutable
  - Rust doesn’t allow us to mark only certain fields as mutable.
  - field init shorthand syntax
    - Because the email field and the email parameter have the same name, we only need to write email rather than email: email.
- **Tuple Structs**
  - Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields
  - Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples
- **Enums**
  - Enums allow you to define a type by enumerating its possible *variants*.
  - These are the only possibilities for an IP address that our program will come across: we can enumerate all possible variants, which is where enumeration gets its name.
- **Additional Notes**
- 


## Option Enum
- Part of the standard library
- `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value
  - You have to convert an `Option<T>` to a `T` before you can perform T operations with it. 

## Null value
a null is a value that is currently invalid or absent for some reason.