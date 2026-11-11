use std::mem::swap;
use std::ops::*;
#[derive(Debug)]
pub struct Ratio{
    a: i32,
    b: i32,
}
impl Ratio{
    fn update(&mut self){
        if self.b<0 {
            self.a = -self.a;
            self.b = -self.b;
        }
        let mut a=self.a.abs();
        let mut b=self.b;
        if a<b {
            swap(&mut a, &mut b);
        }
        let mut r;
        while {r=a%b;r!=0} {
            a=b;b=r;
        }
        self.a/=b;self.b/=b;
    }
}
impl Add for Ratio{
    type Output=Ratio;
    fn add(self, other:Ratio) -> Ratio{
        let mut ans=Ratio{a:self.a*other.b+other.a*self.b, b:self.b*other.b};
        ans.update();
        ans
    }
}
impl Sub for Ratio{
    type Output=Ratio;
    fn sub(self, other:Ratio) -> Ratio{
        let mut ans=Ratio{a:self.a*other.b-self.b*self.b, b:self.b*other.b};
        ans.update();
        ans
    }
}
impl Mul for Ratio{
    type Output=Ratio;
    fn mul(self, other:Ratio) -> Ratio{
        let mut ans=Ratio{a:self.a*other.a, b:self.b*other.b};
        ans.update();
        ans
    }
}
impl Div for Ratio{
    type Output=Ratio;
    fn div(self, other:Ratio) -> Ratio{
        let mut ans=Ratio{a:self.a*other.b, b:self.b/other.a};
        ans.update();
        ans
    }
}

impl PartialEq<Ratio> for Ratio {
    fn eq(&self, other: &Ratio) -> bool {
        self.a==other.a && self.b==other.b
    }
}
impl From<i32> for Ratio{
    fn from(a:i32) -> Ratio{
        Ratio{a:a,b:1}
    }
}
#[test]
fn test(){
    //Add
    assert_eq!(Ratio::from(1) + Ratio::from(1), Ratio::from(2));
    //Sub
    assert_eq!(Ratio::from(1) - Ratio::from(1), Ratio::from(0));
    //Mul
    assert_eq!(Ratio::from(2) * Ratio::from(2), Ratio::from(4));
    //Div
    assert_eq!(Ratio::from(2) / Ratio::from(2), Ratio::from(1));
}