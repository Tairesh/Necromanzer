use std::convert::TryFrom;

pub fn next<S: Into<u8> + TryFrom<u8>>(val: S) -> S {
    let i: u8 = val.into();
    S::try_from(i + 1).unwrap_or_else(|_| S::try_from(0).ok().unwrap())
}

pub fn prev<S: Into<u8> + TryFrom<u8>>(val: S, last: u8) -> S {
    let i: u8 = val.into();
    S::try_from(if i > 0 { i - 1 } else { last }).ok().unwrap()
}
