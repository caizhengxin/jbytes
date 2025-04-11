# jbytes

[![Crates.io](https://img.shields.io/crates/v/jbytes)](https://crates.io/crates/jbytes)
[![Crates.io](https://img.shields.io/crates/d/jbytes)](https://crates.io/crates/jbytes)
[![License](https://img.shields.io/crates/l/jbytes)](LICENSE-MIT)

This is a Rust-based implementation of byte stream structured serialization/deserialization general library, can be applied to network packet parsing, network packet group package, network communication, file content parsing, etc., feel good small partners please click like 👍~

## Install

### Cargo.toml

```toml
[dependencies]
jbytes = { version="0.3.0", features = ["derive"] }
```

no_std:

```toml
[dependencies]
jbytes = { version="0.3.0", default-features = false, features = ["derive"] } # default use alloc.
```

## Example

### Bytes Example

```rust
use jbytes::prelude::*;


fn main() {
    let bytes = Bytes::new(b"\x01\x02\x03");
    assert_eq!(bytes.take_be_u16(), Ok(0x0102));
    assert_eq!(bytes.take_be_u16().is_err(), true);
}
```

### Buffer Example

```rust
use jbytes::prelude::*;


fn buffer_example(buffer: &mut Buffer) -> JResult<()>  {
    buffer.push_be_u16(1)?;
    buffer.push(b"\x01\x02\x03")?;

    Ok(())
}


fn main() {
    let mut buffer = Buffer::new();
    if buffer_example(&mut buffer).is_ok() {
        assert_eq!(*buffer, b"\x00\x01\x01\x02\x03");
    }
}
```

### Simple Example

```rust
use jbytes::{ByteEncode, ByteDecode};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    #[jbytes(length="length")]
    pub value: String,
    pub cmd: u8,
    #[jbytes(branch="cmd")]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub enum SimpleExampleBody {
    #[jbytes(branch_value=1)]  // Set 1
    Read {
        address: u8,
    },
    Write {
        address: u8,
        value: [u8; 3],
    },                        // Increment to 2
    #[jbytes(branch_default)]
    Unknown,                  // _ => { ... }
}


fn main() -> JResult<()> {
    let input = b"\x00\x03\x31\x32\x33\x01\x05";
    let value: SimpleExample = jbytes::decode(input)?;
    assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(*jbytes::encode(value)?, input);
}
```

### default value example

```toml
[dependencies]
jbytes = { version="0.3.0", features = ["derive", "jdefault"] }
```

```rust
use jbytes::{ByteEncode, ByteDecode, Jdefault};
use jbytes::prelude::*;


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
pub struct SimpleExample {
    #[jbytes(byte_count=1, default="\"123\".to_string()")]
    pub value: String,
    #[jbytes(byte_count=1)]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode, Jdefault)]
pub enum SimpleExampleBody {
    #[jbytes(branch_value=1)]
    Read {
        address: u8,
    },
    Write {
        address: u8,
        value: [u8; 3],
    },
    #[jbytes(branch_default)]
    Unknown {
        #[jbytes(default=10)]
        value: u8,
    },
}


fn main() -> JResult<()> {
    let value = SimpleExample::default();
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });

    assert_eq!(*jbytes::encode(value)?, b"\x03\x31\x32\x33\x03\x0a");

    let value: SimpleExample = jbytes::decode(b"\x03\x31\x32\x33\x03\x0a")?;
    assert_eq!(value, SimpleExample {
        value: "123".to_string(),
        body: SimpleExampleBody::Unknown { value: 10 },
    });
}
```

### Other example

- [TCP socket example](./examples/socket_example.rs)
- [Ethernet parsing example](./examples/packet_ethernet_example.rs)
- [IPv4 parsing example](./examples/packet_ipv4_example.rs)
- [TCP parsing example](./examples/packet_tcp_example.rs)
- [HTTP parsing example](./examples/packet_http_example.rs)
- [HTTP parsing example2](./examples/packet_http_example_2.rs)
- [HTTP parsing example3](./examples/packet_http_example_3.rs)
- [Packet parsing example](./examples/packet_parse_example.rs): Ethernet/IPv4/TCP/UDP

## DataType

- [x] `u8/u16/u32/u64/usize/u128`
- [x] `i8/i16/i32/i64/isize/i128`
- [x] `bool`
- [x] `char`
- [x] `f32/f64`
- [x] `String`
- [x] `&str`
- [x] `&[u8]`
- [x] `array[T; N]`
- [x] `tuple`
- [x] `Vec<T>`
- [x] `Option<T>`
- [x] `Struct`
- [x] `Enum`
- [x] `PhantomData`
- [x] `HashMap`
- [x] `HashSet`
- [x] `MacAddress`
- [x] `std::net::Ipv4Addr`
- [x] `std::net::Ipv6Addr`
- [x] `std::net::IpAddr`
- [x] `NetAddress`
- [x] `HexString`
- [ ] `DateTime`
- [ ] `Bit`

## Macro modifier attribute

### ContainerAttrModifiers

It is used to modify the global definition of a struct/enum, indicating that all the fields in struct/enum follow. You can also use 'FieldAttrModifiers' to modify a single content.

> Universal modifier

- [x] `byteorder=<"BE"|"LE"|variable(BE=0,LE=1)>`: Specifies byte order, BE(big-endian)/LE(little-endian), eg: [byteorder example](./tests/test_modifier_byteorder.rs).
- [x] `encode_with=<func>`: Specifies custom encode function, eg: [encode_with example](./tests/test_modifier_with2.rs).
- [x] `decode_with=<func>`: Specifies custom decode function, eg: [decode_with example](./tests/test_modifier_with2.rs).
- [x] `with=<mod>`: Specifies custom encode/decode function, eg: [with example](./tests/test_modifier_with2_1.rs).
- [x] `get_variable_name=<variable>`: Get cache variable, must be used with 'variable_name', can be used for different struct or enum type passing, eg: [variable_name_example](./tests/test_modifier_variable_name.rs).

> Enum type modifier

- [x] `byte_count_disable`: Disable the default reading of 1 byte to implement the match enumeration branch.
- [ ] `branch_enum`

### FieldAttrModifiers

It is used to modify a field in the struct/enum.

- [x] `byteorder=<"BE"|"LE"|variable(BE=0,LE=1)>`: Specifies byte order, BE(big-endian)/LE(little-endian), eg: [byteorder example](./tests/test_modifier_byteorder.rs).
- [x] `length=<num|variable>`: Specifies read data length, Support `int/&str/String/&[u8]/Vec/..` Type, eg: [length example](./tests/test_modifier_length.rs).
- [x] `offset=<num|variable>`: Specifies n positions forward from the current position to offset the data flow, eg: [offset example](./tests/test_modifier_offset.rs).
- [x] `full=<int>`: Specifies the encode encoding fill value, which defaults to 0 and is often used to fill the encode encoding after the offset, eg: [full example](./tests/test_modifier_full.rs).
- [x] `byte_count=<1..8>`: Specifies the number of bytes to be converted into an integer, representing the byte stream length to be read later, eg: [byte_count example](./tests/test_modifier_bytecount.rs).
- [x] `remaining`: Takes all remaining bytes, eg: [remaining example](./tests/test_modifier_remaining.rs).
- [x] `untake`: Specifies the data read position does not move, and data can continue to be read from this position, eg: [untake example](./tests/test_modifier_untake.rs).
- [x] `encode_value=<expr>`: Specifies the value handler expression for encode function, eg: [encode_value example](./tests/test_modifier_value.rs).
- [x] `decode_value=<expr>`: Specifies the value handler expression for decode function, eg: [decode_value example](./tests/test_modifier_value.rs).
- [x] `variable_name=<variable>`: Specifies the integer type cache variable and uses it in other Struct/Enum via the `get_variable_name` modifier, eg: [variable_name example](./tests/test_modifier_variable_name.rs).
- [x] `skip`: Skip the 'encode/decode' function for this field, the type needs to implement the 'Default' trait, eg: [skip example](./tests/test_modifier_skip.rs).
- [x] `skip_encode`: Skip the `encode` function for this field, eg: [skip_encode example](./tests/test_modifier_skip.rs).
- [x] `skip_decode`: Skip the 'decode' function for this field, the type needs to implement the 'Default' trait, eg: [skip_decode example](./tests/test_modifier_skip.rs).
- [x] `if_expr=<bool expr>`: Specifies `if` condition expression, Support `Option<T>` Type, eg: [if_expr example](./tests/test_modifier_if_expr.rs).
- [x] `encode_with=<func>`: Specifies custom encode function, eg: [encode_with example](./tests/test_modifier_with.rs).
- [x] `decode_with=<func>`: Specifies custom decode function, eg: [decode_with example](./tests/test_modifier_with.rs).
- [x] `with=<mod>`: Specifies custom encode/decode function, eg: [with example](./tests/test_modifier_with_1.rs).
- [x] `with_args=<variable>`: Specifies custom encode/decode function parameter, eg: [with_args example](./tests/test_modifier_with_args.rs).
- [x] `linend|end_with=<string|bytes>`: Specifies end position, Support `String/&str/&[u8]/HashMap/..` Type, eg: [linend](./tests/test_modifier_key.rs).
- [x] `key|starts_with`: Specifies the exact matching keyword, Support `string/&str/&[u8]/..` Type, eg: [key example](./tests/test_modifier_key.rs).
- [x] `split`: Specifies the delimiter, often used for things like 'Key: Value', and supports the HashMap type, eg: [split example](./tests/test_type_hashmap.rs)
- [x] `from_str`: Specifies that the conversion is done by the `FromStr` type, eg: [from_str example](./tests/test_modifier_from_str.rs).
- [x] `from_str=<type>`: Specifies that the conversion is done by the `Type::FromStr` type, eg: [from_str example](./tests/test_modifier_from_str.rs).
- [x] `check_value`: Check whether the result is normal. If an exception occurs, an error is returned, eg: [check_value example](./tests/test_modifier_check_value.rs).

> Container type modifier, eg: Vec/HashMap/HashSet etc.

- [x] `count=<num|variable>`: Specifies the number of container elements, Support `Vec/HashMap/HashSet` type, eg: [count example](./tests/test_modifier_count.rs).
- [x] `try_count=<num|variable>`: Specifies max the number of container elements, if insufficient, no parsing error is returned, Support `Vec/HashMap/HashSet` Type, eg: [try_count example](./tests/test_modifier_try_count.rs).
- [x] `byte_count_outside=<1..8>`: Specifies the number of container elements, Similar `byte_count`, Support`Vec/HashMap/HashSet/..` type, eg: [byte_count_outside example](./tests/test_modifier_bytecount_outside.rs).

> enum branch modifier

- [x] `branch`: Specifies the enumeration (Enum) type branching condition, eg: [branch example](./tests/test_modifier_branch.rs).
- [x] `branch_value`: Specifies an enumeration (Enum) type branch matching condition, eg: [branch_value example](./tests/test_type_modifier_branch_value.rs).
- [x] `branch_range`: Specifies an enumeration (Enum) type branch matching range, eg: [branch_range example](./tests/test_type_modifier_branch_range.rs).
- [x] `branch_bits`: Specifies an enumeration (Enum) type branch matching condition, eg: [branch_bits example](./tests/test_type_modifier_branch_bits.rs).
- [x] `branch_default`: Specifies an enumeration (Enum) type default condition, eg: [branch_default example](./tests/test_modifier_branch_default.rs).
