
use crate::{SiUnit,Vector};
use core::ops::Add;
#[derive(Debug)]
///Cartesian coordinates struct
pub struct Cartesian{
    ///x coordinates
    pub x:f64,
    ///y coordinates
    pub y:f64,
    ///Numerator Vector of SiUnits
    pub num: Vec<SiUnit>,
    ///Denominator Vector of SiUnits
    pub den: Vec<SiUnit>
}
///Transform Vector to Cartesian
/// ```rust
/// let v = Vector{
///     value:Value{
///         magnitude:1_f64,
///         si_units_num: Vec::from([SiUnit::Kilogram,SiUnit::Metres])
///         si_units_den: Vec::from([SiUnit::Seconds,SiUnit::Seconds])
///     }
///     theta:crate::PI
/// };
/// let cart:Cartesian = v.into()
///println!("{}",cart);
/// ```
impl From<Vector> for Cartesian{
    fn from(other: Vector) -> Self {
        Self{x:other.value.magnitude*other.theta.cos(),
            y:other.value.magnitude*other.theta.sin(),
            num:other.value.si_units_num,
            den:other.value.si_units_den}
    }
}


/// ```rust
/// let c1 = Cartesian{x: 1,y: 0,num: Vec::<SiUnit>::from([]),den: Vec::<SiUnit>::from([])}
/// let c2 = Cartesian{x: 1,y: 0,num: Vec::<SiUnit>::from([]),den: Vec::<SiUnit>::from([])}
/// println!("{}",c1+c2);
impl Add<Cartesian> for Cartesian {
    type Output = Cartesian;
    fn add(self, rhs: Cartesian) -> Self::Output {
        if self.num.len() == rhs.num.len(){
            for i in 0..self.num.len(){
                if !(self.num[i] == rhs.num[i]){
                    println!("Paniccc");
                    panic!("Units not identical");
                }
            }
        }else {
            panic!("Units not identical");
        }
        if self.den.len() == rhs.den.len() {
            for i in 0..self.den.len(){
                if !(self.den[i] == rhs.den[i]){
                    panic!("Units not identical");
                }
            }
        }
        Self{x: self.x + rhs.x,y: self.y + rhs.y,..self}
    }
}
impl Clone for Cartesian{
    fn clone(&self) -> Self {
        Self{x: self.x,y:self.y,num:self.num.clone(),den:self.den.clone()}
    }
}
