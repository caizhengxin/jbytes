# jbytes

## 安装

### Cargo.toml

```toml
[dependencies]
jbytes = { version="0.1.0", features = ["derive"] }
```

Or

```toml
[dependencies]
jbytes = { version="0.1.0", features = ["derive", "serde"] }
```

## 例子

### Bytes例子

```rust
use jbytes::prelude::*;


fn main() {
    let bytes = Bytes::new(b"\x01\x02\x03");
    assert_eq!(bytes.take_be_u16().unwrap(), 0x0102);
    assert_eq!(bytes.take_be_u16().is_err(), true);
}
```

### Buffer例子

```rust
use jbytes::prelude::*;


fn main() {
    let mut buffer = Buffer::new();
    assert_eq!(buffer.push_be_u16(1).unwrap(), 2);
    assert_eq!(buffer.push(b"\x01\x02\x03").unwrap(), 3);
    assert_eq!(*buffer, b"\x00\x01\x01\x02\x03");
}
```

### 简单例子

```rust
use jbytes::{ByteEncode, ByteDecode};


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
pub struct SimpleExample {
    pub length: u16,
    // 这里是指定动态长度，也可以指定固定数值，比如：`#[jbytes(length=3)]`
    // 还可以不指定`length`, 指定`byte_count=<1..8>`表示提前取走几个字节根据字节序转为长度数值
    #[jbytes(length="length")]
    pub value: String,
    pub cmd: u8,
    // 这里指定了branch, 表示根据cmd的值进行枚举类型(enum)模式匹配, 同样也可以不指定branch, 指定`byte_count=<1..8>`修饰符
    #[jbytes(branch="cmd")]
    pub body: SimpleExampleBody,
}


#[derive(Debug, PartialEq, Eq, ByteEncode, ByteDecode)]
#[repr(u8)]
pub enum SimpleExampleBody {
    Read {
        address: u8,
    } = 1,                    // 这里表示当前面的`cmd`字段为1，则会进入该分支解析
    Write {
        address: u8,
        value: [u8; 3],
    },                        // 这里如果不指定，默认是递增的关系为2
    #[jbytes(branch_default)]
    Unknown,                  // 这里由于指定了默认分支，所以会被映射为`_ => { ... }`, 如果没有指定，Unknown序号为3，其他则会返回解析错误
}


fn main() {
    let input = b"\x00\x03\x31\x32\x33\x01\x05";
    let value: SimpleExample = jbytes::decode(input).unwrap();
    assert_eq!(value, SimpleExample { length: 3, value: "123".to_string(), cmd: 1, body: SimpleExampleBody::Read { address: 5 } });
    assert_eq!(*jbytes::encode(value).unwrap(), input);
}
```

### 默认值例子

```rust
```

### 其他例子

- [TCP通信例子](./examples/socket_example.rs)
- [以太网解析例子](./examples/packet_ethernet_example.rs)
- [IPv4解析例子](./examples/packet_ipv4_example.rs)
- [TCP解析例子](./examples/packet_tcp_example.rs)
- [HTTP解析例子](./examples/packet_http_example.rs)
- [HTTP解析例子2](./examples/packet_http_example_2.rs)
- [HTTP解析例子3](./examples/packet_http_example_3.rs)
- [解析例子](./examples/packet_parse_example.rs)：包含Ethernet/IPv4/TCP/UDP

## 数据类型

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

## 宏修饰属性

### ContainerAttrModifiers

主要用于修饰某个struct/enum全局定义，表示struct/enum里面的字段都遵循，也可以通过`FieldAttrModifiers`修饰单个内容。

> 通用修饰符

- [x] `byteorder=<"BE"|"LE">`: 表示字节序，BE(大端字节序)/LE(小端字节序)，eg: [byteorder example](./tests/test_modifier_byteorder.rs)。
- [x] `encode_with=<func>`: 自定义encode函数，eg: [encode_with example](./tests/test_modifier_with2.rs)。
- [x] `decode_with=<func>`: 自定义decode函数，eg: [decode_with example](./tests/test_modifier_with2.rs)。
- [x] `with=<mod>`: 自定义encode/decode函数，eg: [with example](./tests/test_modifier_with2_1.rs)。
- [x] `get_variable_name=<variable>`: 获取缓存变量, 必须配合`variable_name`使用，可以用于不同struct或enum类型传递, eg: [variable_name_example](./tests/test_modifier_variable_name.rs)。

> 枚举分支修饰符

- [ ] `branch_enum`

### FieldAttrModifiers

主要用于修饰struct/enum里面某个字段内容

- [x] `byteorder=<"BE"|"LE"|variable(BE=0,LE=1)>`: 表示字节序，BE(大端字节序)/LE(小端字节序)，eg: [byteorder example](./tests/test_modifier_byteorder.rs)。
- [x] `length=<num|variable>`: 表示读取数据的长度，支持`int/&str/String/&[u8]/Vec`类型，eg: [length example](./tests/test_modifier_length.rs)。
- [x] `offset=<num|variable>`: 表示从当前位置向前前进n个位置，实现数据流的位置偏移，eg: [offset example](./tests/test_modifier_offset.rs)。
- [x] `full=<int>`: 表示用于`encode`编码填充值, 默认为0, 常常用于offset偏移之后进行`encode`编码填充, eg: [full example](./tests/test_modifier_full.rs)。
- [x] `byte_count=<1..8>`: 表示取几个字节转成整型，代表后续需要读取的字节流长度，eg：[byte_count example](./tests/test_modifier_bytecount.rs)。
- [x] `remaining`: 表示取走剩余所有字节，eg：[remaining example](./tests/test_modifier_remaining.rs)。
- [x] `untake`: 表示读取数据不移动位置，后续可以继续从该位置读取数据，eg: [untake example](./tests/test_modifier_untake.rs)。
- [x] `encode_value=<expr>`: value处理表达式，eg: [encode_value example](./tests/test_modifier_value.rs)。
- [x] `decode_value=<expr>`: value处理表达式，eg: [decode_value example](./tests/test_modifier_value.rs)。
- [x] `variable_name=<variable>`: 指定整型类型缓存变量，并通过`get_variable_name`修饰符在其他`Struct/Enum`使用，eg: [variable_name example](./tests/test_modifier_variable_name.rs)。
- [x] `skip`: 表示跳过该字段的`encode/decode`函数，类型需要实现`Default`trait，eg：[skip example](./tests/test_modifier_skip.rs)。
- [x] `skip_encode`: 表示跳过该字段的`encode`函数，eg：[skip_encode example](./tests/test_modifier_skip.rs)。
- [x] `skip_decode`: 表示跳过该字段的`decode`函数，类型需要实现`Default`trait，eg：[skip_decode example](./tests/test_modifier_skip.rs)。
- [x] `if_expr=<bool expr>`: 指定`if`条件表达式，支持`Option<T>`类型，eg: [if_expr example](./tests/test_modifier_if_expr.rs)。
- [x] `encode_with=<func>`: 自定义encode函数，eg: [encode_with example](./tests/test_modifier_with.rs)。
- [x] `decode_with=<func>`: 自定义decode函数，eg: [decode_with example](./tests/test_modifier_with.rs)。
- [x] `with=<mod>`: 自定义encode/decode函数，eg: [with example](./tests/test_modifier_with_1.rs)。
- [x] `with_args=<variable>`: 自定义encode/decode函数参数，eg: [with_args example](./tests/test_modifier_with_args.rs)。
- [x] `linend|end_with=<string|bytes>`：指定结束位置，支持`String/&str/&[u8]/HashMap`等类型，eg：[linend](./tests/test_modifier_key.rs)。
- [x] `key|starts_with`：指定精准匹配关键字，必须配合`linend`使用，支持`string/&str/&[u8]`等类型，eg：[key example](./tests/test_modifier_key.rs)。
- [x] `split`: 指定分隔符, 常常用于`Key: Value`这种内容, 支持`HashMap`类型, eg: [split example](./tests/test_type_hashmap.rs)
- [x] `from_str`: 表示通过`FromStr`类型进行转换，eg：[from_str example](./tests/test_modifier_from_str.rs)。
- [x] `from_str=<type>`：表示通过`Type::FromStr`类型进行转换，eg：[from_str example](./tests/test_modifier_from_str.rs)。
- [x] `check_value`：主要用于检查结果是否正常，如果异常会返回错误，eg：[check_value example](./tests/test_modifier_check_value.rs)。

> 容器类型修饰符，比如：Vec/HashMap/HashSet等

- [x] `count=<num|variable>`: 表示容器元素数量，支持`Vec/HashMap/HashSet`类型，eg：[count example](./tests/test_modifier_count.rs)。
- [x] `try_count=<num|variable>`: 表示容器元素数量, 如果不足，不会返回解析错误，支持`Vec/HashMap/HashSet`类型，eg：[try_count example](./tests/test_modifier_try_count.rs)。
- [x] `byte_count_outside=<1..8>`：表示容器元素数量，类似`byte_count`，该修饰符应用`Vec/HashMap/HashSet`等类型，eg：[byte_count_outside example](./tests/test_modifier_bytecount_outside.rs)。

> enum branch

- [x] `branch`: 指定枚举(Enum)类型分支条件，eg: [branch example](./tests/test_modifier_branch.rs)。
- [x] `branch_value`: 指定枚举(Enum)分支判断条件, eg: [branch_value example](./tests/test_type_modifier_branch_value.rs)。
- [x] `branch_range`: 指定枚举(Enum)分支判断条件范围, eg: [branch_range example](./tests/test_type_modifier_branch_range.rs)。
- [x] `branch_bits`: 指定枚举(Enum)分支判断条件, eg: [branch_bits example](./tests/test_type_modifier_branch_bits.rs)。
- [x] `branch_default`: 指定枚举(Enum)类型默认值, eg: [branch_default example](./tests/test_modifier_branch_default.rs)。
