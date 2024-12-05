#[cfg(feature = "std")]
#[allow(unused_imports)]
pub use std::{
    ops,
    cmp,
    hash,
    fmt,
    format,
    str,
    str::FromStr,
    string::{String, ToString},
    vec,
    vec::Vec,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    cell::RefCell,
    collections::BTreeMap,
    mem::MaybeUninit,
    marker::{PhantomData, PhantomPinned},
};
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub use core::{
    ops,
    cmp,
    fmt,
    hash,
    str,
    str::FromStr,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    cell::RefCell,
    mem::MaybeUninit,
    marker::{PhantomData, PhantomPinned},
};
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
    collections::BTreeMap,
};
