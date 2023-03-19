![CI](https://github.com/rnpozharskiy/ruff/actions/workflows/rust.yml/badge.svg)
# Overview
`ruff` is blazingly slow and memory inefficient finite fields library. It provides basic operations like `+`, `-`, `*`, `/` for arbitrary finite field.
## Common usage example
```rust
use ruff::finite_field::{FiniteField, FFElement};

fn main(){
    // initialize finite field with characteristics, pow and irreducable polynomial
    let finite_field = FiniteField::new(7, 8, vec![5,1,0,3,1,0,0,0,1]);
    // initialize element with representation and field
    let a = FFElement::new(vec![6,1,0,3,1,6,0,6], &finite_field);
    let b = FFElement::new(vec![5,1,0,3,1,2,0,6], &finite_field);
    let sum = a + b;
    let mul = a * b;
    let sub = a - b;
    // getting inverse element in multiplicative group
    let a_inverse = a.inverse();
    let div = a / b;
    // getting inverse element in additive group
    let b_opposite = -b;
}
```
## GF256

Since $GF(256)$ elements represents bytes, extra more concise interface with hardcoded Rijndael polynomial $X^8 + X^4 + X^3 + X + 1$ provided for them!

```rust
use ruff::finite_field_element::gf256_element::GF256Element;

fn main(){
    // no need no initialize field and provide it to elements
    let a = GF256Element::new(vec![0, 0, 0, 1, 1, 0, 0, 0]);
    let b = GF256Element::new(vec![1, 0, 1, 1, 1, 0, 0, 1]);
    //can perform same operations
    let sum = a + b;
    let mul = a * b;
    let sub = a - b;
    let div = a / b;
    let a_inverse = a.inverse();
    let b_opposite = -b;

    // extra useful operations
    // expected element has representation [0, 1, 1, 1, 1, 1, 1, 1]
    let c = GF256Element::from_byte(254);
    // expected output - 254 
    let bytes = c.to_byte();
}
```
Notice that given code examples will not complie due to [variables movement](https://doc.rust-lang.org/rust-by-example/scope/move.html) in Rust. They are just shorthands so that readme does not grow much. You must use it considering Rust peculiarities.

## AES stage
As an application of using the $GF(256)$ finite field, the library also provides the ability to encode messages that are a multiple of 16 bytes long with a key of 256 bytes using one of the stages of the AES algorithm
```rust
use ruff::aes_service::{Crypter, AesService};

fn main(){
    // Initialize new service with key representing [u8; 256]
    let aes_service = AesService::new(&key);
    let message = "asdfgtyhplnbty56"
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
}
```
