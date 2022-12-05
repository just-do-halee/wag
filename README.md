# **`WAG`**

Go like sync.WaitGroup implementation in Rust. (sync/async)

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/wag/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/wag.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/wag?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[ci-url]: https://github.com/just-do-halee/wag/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/wag
[license-url]: https://github.com/just-do-halee/wag

| [Examples](./tests/) | [Docs](https://docs.rs/wag) | [Latest Note](./CHANGELOG.md) |

```toml
wag = "0.1.0"
```

## **`How to use,`**

```rust
use wag::WaitGroup;

let wg = WaitGroup::new();
```

```rust
for _ in 0..10 {
    let w = wg.add();

    thread::spawn(move || {
        // ...
        w.done();
    });

});
wg.wait(); // or wg.async_wait().await;
```

```rust
for w in wg.adds::<10>() {

    thread::spawn(move || {
        // ...
        w.done();
    });

});
wg.wait(); // or wg.async_wait().await;
```

```rust
let [w1, w2, w3] = wg.adds();

thread::spawn(move || {
    // ...
    w1.done();
});

thread::spawn(move || {
    // ...
    w2.done();
});

thread::spawn(move || {
    // ...
    w3.done();
});

wg.wait(); // or wg.async_wait().await;
```

```rust
wg.adds_iter::<10>().enumerate().for_each(|(i, w)| {

    thread::spawn(move || {
        // ... with i
        w.done();
    });

});
wg.wait(); // or wg.async_wait().await;
```

