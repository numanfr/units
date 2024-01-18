
use crate::DerivedQuantities;
use crate::SiUnit;
use crate::Value;

/// Enum of derived units 
#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy,PartialOrd,Ord)]
pub enum DerivedUnit {
    ///Hertz
    Hertz,
    ///Newtons
    Newtons,
    ///Pascals
    Pascals,
    ///Joules
    Joules,
    ///Watts
    Watts,
    ///Volts
    Volts,
    ///Coulombs
    Coulombs,
    ///Sieverts
    Sieverts
}

impl DerivedUnit {
/// Get a Value struct from from a derived quantitiy
/// ```rust
/// let hearing_limit = DerivedUnit::Hertz.get_value().set_magnitude(20_000);
/// println!("{}",hearing_limit);
/// ```
    pub fn get_value(&self) -> Value{
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

