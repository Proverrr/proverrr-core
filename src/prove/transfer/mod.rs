use crate::figure::{Figure, ID};
use crate::prove::proof::Proof;

pub mod base_transfer;
fn update_proof(figure: &mut Figure,from: &Vec<ID>, to:&Vec<ID>){
    let mut ans: Proof;
    for i in from {
        ans=ans+figure.proofs[i];
    }
    for i in to {
        figure.proofs[i]=ans;
    }
}
#[derive(PartialEq)]
pub enum TransferType{
    
}