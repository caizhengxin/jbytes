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
- [ ] `HashMap`
- [ ] `HashSet`
- [x] `MacAddress`
- [x] `std::net::Ipv4Addr`
- [x] `std::net::Ipv6Addr`
- [x] `std::net::IpAddr`
- [x] `NetAddress`
- [ ] `HexString`
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
- [x] `untake`: 表示读取数据不移动位置，后续可以继续从该位置读取数据，eg: [untake example](./tests/test_modifier_untake.rs)。
- [x] `encode_value`: value处理表达式，eg: [encode_value example](./tests/test_modifier_value.rs)。
- [x] `decode_value`: value处理表达式，eg: [decode_value example](./tests/test_modifier_value.rs)。
- [x] `variable_name`: 指定整型类型缓存变量，并通过`get_variable_name`修饰符在其他`Struct/Enum`使用，eg: [variable_name example](./tests/test_modifier_variable_name.rs)。
- [x] `skip`: 表示跳过该字段的`encode/decode`函数，类型需要实现`Default`trait，eg：[skip example](./tests/test_modifier_skip.rs)。
- [x] `skip_encode`: 表示跳过该字段的`encode`函数，eg：[skip_encode example](./tests/test_modifier_skip.rs)。
- [x] `skip_decode`: 表示跳过该字段的`decode`函数，类型需要实现`Default`trait，eg：[skip_decode example](./tests/test_modifier_skip.rs)。
- [x] `if_expr=<bool expr>`: 指定`if`条件表达式，支持`Option<T>`类型，eg: [if_expr example](./tests/test_modifier_if_expr.rs)。
- [x] `encode_with`: 自定义encode函数，eg: [encode_with example](./tests/test_modifier_with.rs)。
- [x] `decode_with`: 自定义decode函数，eg: [decode_with example](./tests/test_modifier_with.rs)。
- [x] `with`: 自定义encode/decode函数，eg: [with example](./tests/test_modifier_with_1.rs)。
- [x] `with_args`: 自定义encode/decode函数参数，eg: [with_args example](./tests/test_modifier_with_args.rs)。


- [x] `linend|end_with=<string|bytes>`: 指定结束位置, 支持`String/&str/&[u8]/HashMap`等类型.
- [x] `key|starts_with`: 指定精准匹配关键字, 必须配合`linend`使用, 支持`string/&str/&[u8]`等类型.
- [x] `split`: 指定分隔符, 常常用于`Key: Value`这种内容, 支持`HashMap`类型, eg: [split example](./tests/test_type_hashmap.rs)
- [ ] `check_value`：主要用于检查结果是否正常, 异常会返回错误
- [x] `default`: eg: [default example](./crates/jdefault-rs/tests/test_jppe.rs)

> 容器类型修饰符，比如：Vec/HashMap/HashSet等

- [x] `count=<num|variable>`: 表示容器元素数量，支持`Vec/HashMap/HashSet`类型，eg：[count example](./tests/test_modifier_count.rs)。
- [x] `try_count=<num|variable>`: 表示容器元素数量, 如果不足，不会返回解析错误，支持`Vec/HashMap/HashSet`类型，eg：[try_count example](./tests/test_modifier_try_count.rs)。
- [x] `byte_count_outside=<1..8>`：表示容器元素数量，类似`byte_count`，该修饰符应用`Vec/HashMap/HashSet`等类型，eg：[byte_count_outside example](./tests/test_modifier_bytecount_outside.rs)。

> enum branch

- [x] `branch`: 指定枚举(Enum)类型分支条件: [branch example](./tests/test_type_enum.rs)
- [x] `branch`: 指定枚举(Enum)类型分支条件: [branch_option example](./tests/test_modifier_branch_option.rs)
- [x] `branch_default|enum_default`: 指定枚举(Enum)类型默认值, eg: [branch_default example](./tests/test_type_enum.rs)
- [x] `branch_bits`: 指定枚举(Enum)分支判断条件, eg: [branch_bits example](./tests/test_type_enum_bits.rs)
- [x] `branch_range`: 指定枚举(Enum)分支判断条件范围, eg: [branch_range example](./tests/test_type_enum_range.rs)
- [x] `branch_value`: 指定枚举(Enum)分支判断条件, eg: [branch_value example](./tests/test_type_enum_value.rs)
