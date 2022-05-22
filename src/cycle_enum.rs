use std::convert::TryFrom;

pub trait VariantCount {
    fn variant_count() -> usize;
}

pub trait CycleEnum: Into<u8> + TryFrom<u8> + VariantCount {
    fn next(self) -> Self {
        Self::try_from(self.into() + 1).unwrap_or_else(|_| Self::try_from(0).ok().unwrap())
    }

    fn prev(self) -> Self {
        let i: u8 = self.into();
        Self::try_from(if i > 0 {
            i - 1
        } else {
            Self::variant_count() as u8 - 1
        })
        .ok()
        .unwrap()
    }
}
