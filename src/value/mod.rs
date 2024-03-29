use core::fmt;
use std::ops::{Mul, Div, Add};

use crate::si_unit::SiUnit;

#[derive(Debug)]
/// Struct that represents a value with units
pub struct Value{
    ///Magnitude of Value
    pub magnitude: f64,
    ///Numerator
    pub si_units_num: Vec<SiUnit>,
    ///Denominator
    pub si_units_den: Vec<SiUnit>

}
impl Value {
///checks if two Values are the same
/// ```rust
/// let v1 = DerivedUnits::Hertz.get_value();
/// let v2 = DerivedUnits::Newtons.get_value();
/// assert_eq!(v1.same(v2),False);
/// 
    pub fn same(&self,other: &Value) -> bool{
        let new_self = self.clone();
        let new_other = other.clone();
        let unit = new_self / new_other;
        return unit.si_units_num.len() == 0 && unit.si_units_den.len() == 0;
    }
///sets magnitude of Value struct
/// ```rust
/// let v1 = DerivedUnits::Hertz.get_value().set_magnitude(50);
/// println!("{}",v1);
    pub fn set_magnitude(self,new_mag:f64) -> Self{
        Self{magnitude: new_mag,..self}
    }
///Adds a unit to the numerator
/// ```rust
/// let v1 = DerivedUnits::Hertz.get_value().add_num(SiUnit::Kilogram);
/// println!("{}",v1);

    pub fn add_num(mut self,unit:SiUnit) -> Self{
        self.si_units_num.push(unit);
        self.simplify()
    }
///Adds a unit to the denominator
/// ```rust
/// let v1 = DerivedUnits::Hertz.get_value().add_den(SiUnit::Kilogram);
/// println!("{}",v1);
    pub fn add_den(mut self,unit:SiUnit) -> Self{
        self.si_units_den.push(unit);
        self.simplify()
    }

    /// Simplifies a Value struct by deleting units existing in both numerator and denominator
    fn simplify(mut self) -> Self{ 
        self.si_units_num.sort();
        self.si_units_den.sort();
        let (num,den) = Value::remove_duplicates(self.si_units_num, self.si_units_den);
        Self{magnitude: self.magnitude,si_units_num: num,si_units_den: den}
    }
        /// Simplifies a Value struct by deleting one unit existing in both numerator and denominator
    fn remove_one_duplicate(mut x:Vec<SiUnit>,mut y:Vec<SiUnit>) -> Result<(Vec<SiUnit>,Vec<SiUnit>),()>{
        for i in 0..x.len(){
            let e = &x[i];
            match y.binary_search(e) {
                Ok(r) => {
                    x.remove(i);
                    y.remove(r);
                    
                    return Ok((x,y));
                },
                Err(_) => ()
            }
        };
        Err(())
    }
    /// Simplifies a Value struct by deleting units existing in both numerator and denominator
    fn remove_duplicates(mut x:Vec<SiUnit>,mut y:Vec<SiUnit>) -> (Vec<SiUnit>,Vec<SiUnit>){
        for _ in 0..=x.len(){
            match Value::remove_one_duplicate(x.clone(), y.clone()){
                Ok(r) => {
                    x = r.0;
                    y = r.1;
                },
                Err(_) => {return (x,y);}
            }
        }
    
        (x,y)
    }
    
    /// Allows raising Value to int power;
    pub fn powi(self,pow:i32) -> Self{

        let new_mag = self.magnitude.powi(pow);
        let mut new_num = Vec::<SiUnit>::new();
        let mut new_den = Vec::<SiUnit>::new();

        for _ in 0..pow{
            new_num.append(&mut self.si_units_num.clone());
            new_den.append(&mut self.si_units_den.clone());

        }
        Self{magnitude: new_mag,si_units_num: new_num,si_units_den: new_den}
    }

    /// Preform addition in a safe way
    pub fn safe_add(self,rhs:Value) -> Result<Value,&'static str>{
        if self.same(&rhs){
            Ok(self + rhs)
        }else {
            Err("Units not Identical")
        }
    }

}

impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} {:?}/ {:?} ",self.magnitude,self.si_units_num,self.si_units_den)
    }
}

impl Mul<Value> for Value{
    type Output = Value;
    fn mul(self, rhs: Value) -> Self::Output {
        let mut new_num = self.si_units_num;
        new_num.extend(rhs.si_units_num);
        let mut new_den= self.si_units_den;
        new_den.extend(rhs.si_units_den);
        let new_mag = self.magnitude*rhs.magnitude;
        Self{magnitude: new_mag,si_units_num: new_num,si_units_den: new_den}.simplify()

    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        let mut new_num = self.si_units_num;
        new_num.extend(rhs.si_units_den);
        let mut new_den= self.si_units_den;
        new_den.extend(rhs.si_units_num);
        let new_mag = self.magnitude/rhs.magnitude;
        Self{magnitude: new_mag,si_units_num: new_num,si_units_den: new_den}.simplify()
    }
}

impl Mul<f64> for Value{
    type Output = Value;
    fn mul(self, rhs: f64) -> Self::Output {
        Self{magnitude: self.magnitude * rhs,si_units_num: self.si_units_num,si_units_den: self.si_units_den}
    }
}

impl Mul<f32> for Value{
    type Output = Value;
    fn mul(self, rhs: f32) -> Self::Output {
        Self{magnitude: self.magnitude * rhs as f64,si_units_num: self.si_units_num,si_units_den: self.si_units_den}
    }
}

impl Mul<i32> for Value{
    type Output = Value;
    fn mul(self, rhs: i32) -> Self::Output {
        Self{magnitude: self.magnitude * rhs as f64,si_units_num: self.si_units_num,si_units_den: self.si_units_den}
    }
}

impl Mul<i64> for Value{
    type Output = Value;
    fn mul(self, rhs: i64) -> Self::Output {
        Self{magnitude: self.magnitude * rhs as f64,si_units_num: self.si_units_num,si_units_den: self.si_units_den}
    }
}

impl Div<i32> for Value{
    type Output = Value;
    fn div(self, rhs: i32) -> Self::Output {
        Self{magnitude: self.magnitude/rhs as f64,..self}
    }
}

impl Div<f64> for Value{
    type Output = Value;
    fn div(self, rhs: f64) -> Self::Output {
        Self{magnitude: self.magnitude/rhs,..self}
    }
} 
impl Add<Value> for Value{
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        if self.si_units_num.len() == rhs.si_units_num.len() && self.si_units_den.len() == rhs.si_units_den.len(){
            for unit in &self.si_units_num{
                if !rhs.si_units_num.contains(unit){
                    panic!("Units not Identical");
                }
            }
            for unit in &self.si_units_den{
                if !rhs.si_units_den.contains(unit){
                    panic!("Units not Identical");
                }
            }
            return Self{magnitude: self.magnitude + rhs.magnitude, ..self};
        }
        else{
            panic!("Units not Identical");
        }
    }
}

impl Clone for Value{
    fn clone(&self) -> Self {
        Self{magnitude: self.magnitude,si_units_num: self.si_units_num.clone(),si_units_den: self.si_units_den.clone()}
    }
}

