use crate::figure::*;
pub trait BaseTransfer {
    fn transfer(figure: &mut Figure) -> bool;
}