use crate::figure::Figure;

pub trait BaseTransfer {
    fn transfer(&self, figure: &mut Figure) -> bool;
}
