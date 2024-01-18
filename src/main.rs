use si_vectors::*;

fn main(){
    let acc:Cartesian = Vector{value: DerivedQuantities::Acceleration.get_value(),theta: 0_f64}.into();
    let weight:Cartesian = Vector{value: DerivedQuantities::Acceleration.get_value(),theta: PI/2.0}.into();
    let res = acc.clone()+weight.clone();
    println!("{:#?} {:#?} {:#?}",acc,weight,res);
}