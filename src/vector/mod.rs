
use core::fmt;
use std::{ops::{Mul, Div, Add, Rem}, f64::consts::PI};
use crate::Value;
use crate::Cartesian;


#[derive(Debug)]
/// A struct that holds a Value struct and direction
pub struct Vector {
    ///Value Struct
    pub value:Value,
    ///Angle of Vector in radians
    pub theta: f64
}

impl  Mul<Vector> for Vector{
    type Output = Self;
    fn mul(self, rhs: Vector) -> Self::Output {
        Self{value: self.value*rhs.value,theta: (self.theta+rhs.theta).rem(2_f64*PI)}
    }
}

impl Mul<Value> for Vector {
    type Output = Vector;
    fn mul(self, rhs: Value) -> Self::Output {
        Self::Output{value: self.value * rhs, ..self}
    }
}

impl Mul<Vector> for Value {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output{value: self * rhs.value, ..rhs}
    }
}

impl Div<Vector> for Vector{
    type Output = Self;
    fn div(self, rhs: Vector) -> Self::Output {
        Self{value: self.value*rhs.value,theta: (self.theta-rhs.theta).rem(2_f64*PI)}
    }
}

impl Div<Value> for Vector{
    type Output = Self;
    fn div(self, rhs: Value) -> Self::Output {
        Self{value: self.value/rhs,..self}
    }
}

impl Div<Vector> for Value{
    type Output = Vector;
    fn div(self, rhs: Vector) -> Self::Output {
        Self::Output{value: self/rhs.value,..rhs}
    }
}


impl Add<Vector> for Vector{
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        let cs:Cartesian = self.into();
        let cr:Cartesian = rhs.into();
        let f:Self::Output = (cs+cr).into();
        f
    }
}
impl From<Cartesian> for Vector{
    fn from(value: Cartesian) -> Self {
        Self{value:Value{
            magnitude: (value.x.powi(2) + value.y.powi(2)).sqrt(),
            si_units_num: value.num,si_units_den: value.den },
            theta: Vector::get_angle(value.x, value.y) 
        }
    }
}

impl Clone for Vector{
    fn clone(&self) -> Self {
        Self{value:self.value.clone(),theta: self.theta}
    }
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{},  {} radians",self.value,self.theta)
    }
}

impl Vector {
    ///Returns angle from x,y coordinates
    pub fn get_angle(x:f64,y:f64) -> f64{
        if x.is_sign_positive() && y.is_sign_positive() {
            (y/x).atan()
        }else if x.is_sign_positive() && y.is_sign_negative() {
            (y/x).atan() + 2_f64*PI
        } else if x.is_sign_negative() && y.is_sign_positive() {
            (y/x).atan() + PI
        }else{
            (y/x).atan() + PI
        }
    }
}
