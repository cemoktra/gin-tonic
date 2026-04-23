![crates.io](https://img.shields.io/crates/v/gin-tonic.svg)

# gin-tonic

`gin-tonic` is a Rust protobuf library that lets you use your own types directly on the wire — no manual conversion boilerplate.

It provides:

- Protobuf serialization and deserialization (like [`prost`](https://docs.rs/prost))
- A code generator replacing [`prost-build`](https://docs.rs/prost-build)
- A [`tonic`](https://docs.rs/tonic) codec implementation
- A wrapper for [`tonic-build`](https://docs.rs/tonic-build) with extra features
- A `Scalar` trait to map any Rust type directly to a protobuf wire type

## The problem with other libraries

When you use a UUID in a protobuf message with `prost`, you write:

```protobuf
message Foo {
  string my_uuid = 1;
}
```

This generates:

```rust
struct Foo {
    my_uuid: String,
}
```

Your code wants `uuid::Uuid`, so you end up writing conversions everywhere — and handling parse errors at every call site.

## The gin-tonic approach

Annotate your `.proto` file with the Rust type you want:

```protobuf
import "gin/proto/gin.proto";

message Foo {
  string my_uuid = 1 [(gin_tonic.v1.rust_type) = "uuid::Uuid"];
}
```

The `gin-tonic` code generator produces:

```rust
struct Foo {
    my_uuid: uuid::Uuid,
}
```

The conversion is handled once, inside the `Scalar` trait implementation — not scattered across your codebase.

## Built-in UUID support

Two feature flags cover the UUID case out of the box:

| Feature | Wire type | Notes |
|---|---|---|
| `uuid_string` | `string` | Parse errors handled in the wire type conversion |
| `uuid_bytes` | `bytes` | No parse errors; 16-byte fixed representation |

## Custom types

Implement `Scalar` for any type to use it as a protobuf field:

```rust
impl gin_tonic_core::Scalar<gin_tonic_core::scalars::ProtoString> for MyType {
    const WIRE_TYPE: u8 = gin_tonic_core::WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl gin_tonic_core::Encode) {
        encoder.encode_str(&self.to_string());
    }

    fn decode(decoder: &mut impl gin_tonic_core::Decode) -> Result<Self, gin_tonic_core::ProtoError> {
        decoder.decode_string()?.parse().map_err(Into::into)
    }
}
```

## Benchmarks

Measured against prost 0.13.1 on an equivalent message with a UUID, 10 IP addresses, a string, and a nested map with 5 entries.

**gin-tonic:**
```
gin_encode    time: [269.91 ns 270.22 ns 270.79 ns]
gin_decode    time: [658.58 ns 658.75 ns 658.93 ns]
```

**prost** (including `From` conversions to idiomatic Rust types):
```
prost_encode  time: [564.87 ns 566.21 ns 568.18 ns]
prost_decode  time: [667.34 ns 671.05 ns 675.87 ns]
```

Decode performance is on par with prost while encoding is roughly **2× faster**.

## Crates

| Crate | Description |
|---|---|
| `gin-tonic` | Main crate — re-exports everything, includes code generator and tonic codec |
| `gin-tonic-core` | Runtime traits and serialization primitives |
| `gin-tonic-derive` | Derive macros (`Message`, `Enumeration`, `OneOf`) |
