use crate::value::Value;
use crate::si_unit::SiUnit;
pub enum DerivedQuantities {
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
    pub fn get_value(&self) -> Value {
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
