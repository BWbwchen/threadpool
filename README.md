# threadpool

An ultra simple thread pool implementation in Rust.

This project implements the `spawn` with static lifetime and `scope_spawn` with scoped lifetime.

## Todo
- [ ] `join` function.
- [ ] more unit test
- [ ] more example in [src/bin/](src/bin/)

## Reference
1. [Rust Course](https://course.rs/advance-practice1/intro.html).
2. [Crossbeam](https://github.com/crossbeam-rs/crossbeam/blob/1127ee1dfd4838fcf53cfd1c033c9b21c9e4feb3/src/lib.rs#L48-L54)
