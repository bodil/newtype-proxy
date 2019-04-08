## Do Not Use

This crate remains unpublished for a reason. If you need this functionality, please use [`derive_more`](https://crates.io/crates/derive_more).

# newtype-proxy

Automatic deriving of `From` and `Deref` for Rust newtype structs.

## Documentation

Using `#[derive(NewtypeProxy)]` on a single argument tuple struct will
create `From` and `Deref` implementations which wrap and unwrap the argument value.
Thanks to `Deref`, you'll be able to invoke methods on the newtype as
if it were the wrapped type.

```rust
#[macro_use] extern crate newtype_proxy;

#[derive(NewtypeProxy)]
struct MyNewType(Vec<i32>);

#[test]
fn test() {
    // Wrap the inner type using From::from
    let my_new_type = MyNewType::from(vec![1, 2, 3]);
    // Call the len() method on the wrapped Vec using Deref
    let my_new_types_length = my_new_type.len();
}
```

## Licence

Copyright 2017 Bodil Stokke

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public
License along with this program. If not, see
<http://www.gnu.org/licenses/>.
