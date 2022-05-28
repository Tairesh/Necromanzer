use std::convert::TryFrom;

pub trait CycleEnum: Into<usize> + TryFrom<usize> + Default {
    fn variants_count() -> usize;

    fn next(self) -> Self {
        Self::try_from(self.into() + 1).unwrap_or_else(|_| Self::try_from(0).unwrap_or_default())
    }

    fn prev(self) -> Self {
        let i: usize = self.into();
        Self::try_from(if i > 0 {
            i - 1
        } else {
            Self::variants_count() - 1
        })
        .unwrap_or_default()
    }
}
