# jget

[![Crates.io](https://img.shields.io/crates/v/jget)](https://crates.io/crates/jget)
[![Crates.io](https://img.shields.io/crates/d/jget)](https://crates.io/crates/jget)
[![License](https://img.shields.io/crates/l/jget)](LICENSE)

## Cargo.toml

```toml
[dependencies]
jget = { version = "0.4.1", features = ["derive"] }
```

Or

```toml
[dependencies]
jbytes = { version = "0", features = ["derive", "jget"] }
```

## Usage

### Simple example

```rust
use jget::Jget;


#[derive(Debug, Jget)]
pub enum SimpleEnumExample {
    Read {
        #[jget(get_option)]
        value: u8,
    },
    Write {
        #[jget(get_option)]
        value: u8,
        #[jget(get_option)]
        data: u16,
    }
}


fn main() {
    let value = SimpleEnumExample::Read { value: 1 };
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), None);

    let value = SimpleEnumExample::Write { value: 1, data: 2 };
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), Some(2));
}
```

### Struct/Enum example

```rust
use jget::Jget;


#[derive(Debug, Jget)]
pub struct SimpleStructExample {
    #[jget(get="body.get_value():Option<u8>, body.get_data():Option<u16>")]
    pub body: SimpleEnumExample,
}


#[derive(Debug, Jget)]
pub enum SimpleEnumExample {
    Read {
        #[jget(get_option)]
        value: u8,
    },
    Write {
        #[jget(get_option)]
        value: u8,
        #[jget(get_option)]
        data: u16,
    }
}


fn main() {
    let value = SimpleStructExample{ body: SimpleEnumExample::Read { value: 1 } };
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), None);

    let value = SimpleStructExample { body: SimpleEnumExample::Write { value: 1, data: 2 }};
    assert_eq!(value.get_value(), Some(1));
    assert_eq!(value.get_data(), Some(2));
}
```

### Enum/Struct

```rust
use jget_derive::Jget;


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample {
    pub value: u16,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub struct SimpleStructExample2 {
    pub value: u16,
    pub data: u16,
}


#[derive(Debug, PartialEq, Eq, Jget)]
pub enum SimpleEnumExample {
    #[jget(get="value:u16")]
    V1(SimpleStructExample),
    #[jget(get="value:u16", get_option="data:Option<u16>")]
    V2(SimpleStructExample2),
}


#[test]
fn test_enum_struct() {
    let value = SimpleEnumExample::V2(SimpleStructExample2 { value: 1, data: 2 });
    assert_eq!(value.get_value(), 1);
    assert_eq!(value.get_data(), Some(2));
}
```