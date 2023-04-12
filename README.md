# buckets

buckets is a crate for sorting lots of things at once.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
mean = "0.1"
```

## Usage

```rust
use buckets::SortWithBucket;
let mut input: Vec<u16> = vec![65444, 50, 12532, 121];
input.sort_with_buckets();

assert_eq!(input, vec![50, 121, 12532, 65444])
```

## `#![no_std]`

Coming later.

## Nightly

This is nightly because it's blocked on `generic_const_exprs`, because I need a constant used in 
the output of a trait that is also defined in that trait for arrays. Hopefully I can think of
another way to do this.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would 
like to change.

Please make sure to update tests as appropriate.

## Licenses

-   [MIT](https://choosealicense.com/licenses/mit/)
-   [Unlicense](https://choosealicense.com/licenses/unlicense/)
