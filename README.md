# gin-tonic

`gin-tonic` offers:

- a protobuf de-/serialization (like [`prost`](docs.rs/prost))
- a [`tonic`](docs.rs/tonic) codec implementation
- a replacement for [`tonic-build`](docs.rs/tonic-build)

While all this can be achieved using the mentioned crates; `gin-tonic` also offers traits for
converting any Rust type into a protobuf wire type.

An example you can pass a UUID via protobuf as:

```protobuf
message Foo {
  string my_uuid = 1;
}
```

Using [`prost`](docs.rs/prost) and [`tonic-build`](docs.rs/tonic-build) this will generate a Rust struct:

```rust
struct Foo {
    my_uuid: String,
}
```

As you notice the Rust type here is a string, but in your actual code you want to use to uuid as
actual [`uuid::Uuid`](docs.rs/uuid). Now you have to do a fallible conversion.

`gin-tonic` solves this by annotating the protobuf file:

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