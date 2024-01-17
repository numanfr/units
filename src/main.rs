pub mod si_unit;
pub mod derived_quantities;
pub mod value;
pub mod derived_units;
pub mod si_constants;
pub mod vector;
pub mod cartesian;

use std::f64::consts::PI;
use crate::derived_quantities::DerivedQuantities;
use crate::si_unit::SiUnit;
use crate::value::Value;
use crate::derived_units::DerivedUnit;
use crate::si_constants::SiConstant;
use crate::vector::Vector;
use crate::cartesian::Cartesian;





fn main(){
    let earth_mass: Value = DerivedQuantities::Mass.get_value().set_magnitude(5.972e24_f64);
    let earth_radius = DerivedQuantities::Distance.get_value().set_magnitude(6_371_000_f64);
    let g:Value = SiConstant::GravitationalConstant.get_value() * earth_mass /earth_radius.clone() / earth_radius;
    let test = g.same(&DerivedQuantities::Accleration.get_value());
    println!("{} {}",g,test);

    let car1 = Vector{value:DerivedQuantities::Scalar.get_value(), theta:0_f64};
    let car2 = Vector{value:DerivedQuantities::Scalar.get_value(), theta: 1.5*PI};
    let res = car1+car2;
    println!("{:?} {}",res, res.theta.to_degrees());

    let super_heavy_truck = Vector{value:DerivedQuantities::Scalar.get_value().set_magnitude(1e8_f64), theta:-1_f64*PI+0_f64};
    let res = super_heavy_truck+res;
    println!("{:?} {}",res, res.theta.to_degrees());

    let one_unit = Vector{value:DerivedQuantities::Scalar.get_value().set_magnitude(1_f64), theta:0_f64.to_radians()};
    let other_unit = Vector{value:DerivedQuantities::Scalar.get_value().set_magnitude(2_f64), theta:-180.0_f64.to_radians()};
    let res = other_unit + one_unit;
    println!("{:?} {}",res, res.theta.to_degrees());


}

//Test wwweeerrr