use units::*;

fn main(){
    let acc = Vector{value: DerivedQuantities::Acceleration.get_value(),theta: 0_f64};
    println!("{:?}",acc);
}