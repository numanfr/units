pub mod cartesian_mod {
    pub struct Cartesian{
        x:f64,
        y:f64,
        num: Vec<SiUnit>,
        den: Vec<SiUnit>
    }
    
    impl From<Vector> for Cartesian{
        fn from(other: Vector) -> Self {
            Self{x:other.value.magnitude*other.theta.cos(),y:other.value.magnitude*other.theta.sin(),num:other.value.si_units_num,den:other.value.si_units_den}
        }
    }
    
    impl Add<Cartesian> for Cartesian {
        type Output = Cartesian;
        fn add(self, rhs: Cartesian) -> Self::Output {
            if self.num.len() == rhs.num.len(){
                for i in 0..self.num.len(){
                    if !(self.num[i] == rhs.num[i]){
                        panic!("Units not identical");
                    }
                }
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
    
}