use crate::prove::proof::{Condition, Proof};
use crate::figure::Figure;
pub struct Rule{
    //TODO: put here many bools telling which Transfer is enabled
}
impl Rule{
    pub fn run(&self, figure: &Figure) -> bool{
        let flag=false;
        //TODO: iter all the Transfers
        flag
    }
}
pub fn solve(figure: &Figure, target: Box<dyn Condition>, rule: Rule) -> (bool, Proof){
    //Main Progress
    while rule.run(figure) {
        for i in 0..figure.conditions.len(){
            if figure.conditions[i] == target{
                return (true, figure.proofs[i].into());
            }
        }
    }
    (false, Proof{conditions: vec![], transfers: vec![]})
}