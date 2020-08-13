# ring-vec

[![License](https://img.shields.io/crates/l/ring-vec.svg)](https://github.com/irrustible/ring-vec/blob/main/LICENSE)
[![Package](https://img.shields.io/crates/v/ring-vec.svg)](https://crates.io/crates/ring-vec)
[![Documentation](https://docs.rs/ring-vec/badge.svg)](https://docs.rs/ring-vec)

A zero-dependency, no-std compatible, producer-consumer, fixed-size, item-oriented ring buffer backed by a vector.

Requires `alloc`, for the vector.

## Status

Brand new, with basic tests, seems to work.

## Usage

```rust
    #[test]
    fn works() {
        let mut q: RingVec<usize> = RingVec::new(1);
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.push(1), Ok(()));
        assert_eq!(q.peek(), Some(&1));
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.push(2), Ok(()));
        assert_eq!(q.peek(), Some(&2));
        assert_eq!(q.push(3), Err(3));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.push(4), Ok(()));
        assert_eq!(q.peek(), Some(&4));
        assert_eq!(q.pop(), Some(4));
        assert_eq!(q.peek(), None);
        assert_eq!(q.pop(), None);
        assert_eq!(q.peek(), None);
    }
```


## Copyright and License

    Copyright (c) 2020 James Laver, ring-vec contributors.
    
    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.

