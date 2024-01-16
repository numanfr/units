use core::fmt;
use std::ops::{Mul, Div, Add};

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy,PartialOrd,Ord)]
enum SiUnit {
    Metres,
    Seconds,
    Kilogram,
    Ampere,
    Kelvin,
    Mole,
    Candela
}

impl fmt::Display for SiUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiUnit::Kilogram => write!(f,"Kilogram"),
            SiUnit::Metres => write!(f,"Metres"),
            SiUnit::Seconds => write!(f,"Seconds"),
            SiUnit::Ampere => write!(f,"Ampere"),
            SiUnit::Kelvin => write!(f,"Kelvin"),
            SiUnit::Mole => write!(f,"Mole"),
            SiUnit::Candela => write!(f,"Candela")
        }
        
    }
}

enum DerivedUnit {
    Hertz,
    Newtons,
    Pascals,
    Joules,
    Watts,
    Volts,
    Coulombs,
    Sieverts
}

impl DerivedUnit {
    fn get_value(&self) -> Value{
        match *self {
            DerivedUnit::Hertz => Value{magnitude: 1_f64,si_units_num: Vec::from([]),si_units_den: Vec::from([SiUnit::Seconds])},
            DerivedUnit::Newtons  => DerivedQuantities::Force.get_value(),
            DerivedUnit::Pascals => DerivedQuantities::Force.get_value() / DerivedQuantities::Area.get_value(),
            DerivedUnit::Joules => DerivedQuantities::Force.get_value() * DerivedQuantities::Area.get_value(),
            DerivedUnit::Watts => DerivedUnit::Joules.get_value() / DerivedQuantities::Time.get_value(),
            DerivedUnit::Volts => DerivedUnit::Watts.get_value().add_den(SiUnit::Ampere),
            DerivedUnit::Coulombs => DerivedQuantities::Time.get_value().add_num(SiUnit::Seconds),
            DerivedUnit::Sieverts => DerivedUnit::Joules.get_value() / DerivedQuantities::Mass.get_value()

        }
    }
}

enum DerivedQuantities {
    Speed,
    Velocity,
    Accleration,
    Area,
    Volume,
    Mass,
    Force,
    Time,
    Scalar,
    Distance
}

impl DerivedQuantities {
    fn get_value(&self) -> Value {
        match *self {
            DerivedQuantities::Speed => Value{magnitude: 1_f64,si_units_num: Vec::from([SiUnit::Metres]),si_units_den: Vec::from([SiUnit::Seconds])},
            DerivedQuantities::Velocity  => DerivedQuantities::Speed.get_value(),
            DerivedQuantities::Accleration => DerivedQuantities::Velocity.get_value().add_den(SiUnit::Seconds),
            DerivedQuantities::Area => Value{magnitude: 1_f64,si_units_num: Vec::from([SiUnit::Metres,SiUnit::Metres]),si_units_den: Vec::from([])},
            DerivedQuantities::Volume => DerivedQuantities::Area.get_value().add_num(SiUnit::Metres),
            DerivedQuantities::Mass => Value{magnitude: 1_f64,si_units_num: Vec::from([SiUnit::Kilogram]),si_units_den: Vec::from([])},
            DerivedQuantities::Force => DerivedQuantities::Accleration.get_value()*DerivedQuantities::Mass.get_value(),
            DerivedQuantities::Time => Value{magnitude: 1_f64,si_units_num: Vec::from([]),si_units_den: Vec::from([SiUnit::Seconds])},
            DerivedQuantities::Scalar => Value{magnitude: 1_f64, si_units_num: Vec::from([]),si_units_den: Vec::from([])},
            DerivedQuantities::Distance => DerivedQuantities::Scalar.get_value().add_num(SiUnit::Metres)
        }
    }
}

enum SiConstant {
    SpeedOfLight,
    PlanckConstant,
    ElementaryCharge,
    BoltzmannConstant,
    AvogadroConstant,
    LuminousEfficacy,
    HyperfineTransitionFrequency,
    GravitationalConstant
}

impl SiConstant {
    fn get_value(&self) -> Value{
        match *self {
            SiConstant::SpeedOfLight => Value{magnitude: 299_792_458 as f64,si_units_num: Vec::from([SiUnit::Metres]),si_units_den: Vec::from([SiUnit::Seconds])},
            SiConstant::PlanckConstant => Value{magnitude: 6.626e-34,si_units_num: Vec::from([SiUnit::Kilogram,SiUnit::Metres,SiUnit::Metres]), si_units_den: Vec::from([SiUnit::Seconds,SiUnit::Seconds,SiUnit::Seconds])},
            SiConstant::ElementaryCharge => Value{magnitude: 1.602176634e-19,si_units_num: Vec::from([SiUnit::Ampere, SiUnit::Seconds]),si_units_den: Vec::from([])},
            SiConstant::BoltzmannConstant => DerivedUnit::Joules.get_value().add_den(SiUnit::Kelvin) * 1.380649e-23,
            SiConstant::AvogadroConstant =>  DerivedQuantities::Scalar.get_value().add_den(SiUnit::Mole) * 6.02214076e23_f64,
            SiConstant::LuminousEfficacy => Value{magnitude: 683_f64,si_units_num: Vec::from([SiUnit::Candela,SiUnit::Seconds,SiUnit::Seconds,SiUnit::Seconds]),si_units_den: Vec::from([SiUnit::Kilogram,SiUnit::Metres,SiUnit::Metres])},
            SiConstant::HyperfineTransitionFrequency => DerivedUnit::Hertz.get_value() * 9192631770_i64,
            SiConstant::GravitationalConstant => DerivedQuantities::Force.get_value() * DerivedQuantities::Area.get_value().add_den(SiUnit::Kilogram).add_den(SiUnit::Kilogram) * 6.67430e-11
        }
    }
}

struct Value{
    magnitude: f64,
    si_units_num: Vec<SiUnit>,
    si_units_den: Vec<SiUnit>

}

impl Value {

    fn same(&self,other: &Value) -> bool{
        let new_self = self.clone();
        let new_other = other.clone();
        let unit = new_self / new_other;
        return unit.si_units_num.len() == 0 && unit.si_units_den.len() == 0;
    }

    fn set_magnitude(self,new_mag:f64) -> Self{
        Self{magnitude: new_mag,..self}
    }

    fn add_num(mut self,unit:SiUnit) -> Self{
        self.si_units_num.push(unit);
        self.simplify()
    }
    fn add_den(mut self,unit:SiUnit) -> Self{
        self.si_units_den.push(unit);
        self.simplify()
    }
    fn simplify(mut self) -> Self{
        
        self.si_units_num.sort();
        self.si_units_den.sort();
        let (num,den) = Value::remove_duplicates(self.si_units_num, self.si_units_den);
        Self{magnitude: self.magnitude,si_units_num: num,si_units_den: den}
    }
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



fn main(){
    let earth_mass: Value = DerivedQuantities::Mass.get_value().set_magnitude(5.972e24_f64);
    let earth_radius = DerivedQuantities::Distance.get_value().set_magnitude(6_371_000_f64);
    let g:Value = SiConstant::GravitationalConstant.get_value() * earth_mass /earth_radius.clone() / earth_radius;
    let test = g.same(&DerivedQuantities::Accleration.get_value());
    println!("{} {}",g,test);
}

//Test wwweeerrr