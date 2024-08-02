![crates.io](https://img.shields.io/crates/v/gin-tonic.svg)

# gin-tonic

`gin-tonic` offers:

- a protobuf de-/serialization (like [`prost`](http://docs.rs/prost))
- a replacement for [`prost-build`](http://docs.rs/prost-build))
- a [`tonic`](http://docs.rs/tonic) codec implementation
- a wrapper for [`tonic-build`](http://docs.rs/tonic-build) adding some extra extra features

While all this can be achieved using the mentioned crates; `gin-tonic` also offers traits for
converting any Rust type into a protobuf wire type. You are asking why?

If you want to pass a UUID via protobuf you likely end up doing:

```protobuf
message Foo {
  string my_uuid = 1;
}
```

Using [`prost-build`](http://docs.rs/prost-build) and [`tonic-build`](http://docs.rs/tonic-build) this will
generate the following Rust struct:

```rust
struct Foo {
    my_uuid: String,
}
```

As you notice the Rust type here is `String`, but in your actual code you want to use an actual
[`uuid::Uuid`](docs.rs/uuid). Now you have to do a fallible conversion into your code.

`gin-tonic` solves this by adding options to the protobuf file:

```protobuf
import "gin/proto/gin.proto";

message Foo {
  string my_uuid = 1 [(gin_tonic.v1.rust_type) = "uuid::Uuid"];
}
```

Using the `gin-tonic` code generator this generates the following Rust code:

```rust
struct Foo {
    my_uuid: uuid::Uuid,
}
```

For the UUID case `gin-tonic` offers two features:

- `uuid_string` => proto transport is `string`, parsing error is handled within wire type conversion
- `uuid_bytes` => proto transport is `bytes`, this does not require additional error handling

You can add you own types by implementing the `FromWire` and `IntoWire` traits for your type.


## Benchmarks
Currently the decoding performance is slightly better than prost, while encoding requires some work.

gin tonic:
```
decode                  time:   [699.72 ns 700.71 ns 701.81 ns]
encode                  time:   [846.49 ns 848.09 ns 849.86 ns]
```

prost:
```
decode                  time:   [778.30 ns 782.24 ns 788.19 ns]
encode                  time:   [622.77 ns 623.87 ns 625.02 ns]
```
