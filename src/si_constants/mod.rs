
use crate::Value;
use crate::DerivedQuantities;
use crate::DerivedUnit;
use crate::SiUnit;
///Siconstants
pub enum SiConstant {
    ///Speed of light constant
    SpeedOfLight,
    ///Planck constant
    PlanckConstant,
    ///Elementary charge constant
    ElementaryCharge,
    ///Boltzmann Constant
    BoltzmannConstant,
    ///Avogadro Constant
    AvogadroConstant,
    ///Luminous Efficacy
    LuminousEfficacy,
    ///Hyperfine Transition Frequency
    HyperfineTransitionFrequency,
    ///Gravitational Constant
    GravitationalConstant
}


impl SiConstant {
    /// Returns Value struct with a correct units
    /// ```rust
    /// let c::Value = SiConstant::SpeedOfLight.get_value();
    /// println!("{}",c)
    /// ```
    pub fn get_value(&self) -> Value{
        match *self {
            SiConstant::SpeedOfLight => Value{magnitude: 299_792_458 as f64,si_units_num: Vec::from([SiUnit::Metre]),si_units_den: Vec::from([SiUnit::Second])},
            SiConstant::PlanckConstant => Value{magnitude: 6.626e-34,si_units_num: Vec::from([SiUnit::Kilogram,SiUnit::Metre,SiUnit::Metre]), si_units_den: Vec::from([SiUnit::Second,SiUnit::Second,SiUnit::Second])},
            SiConstant::ElementaryCharge => Value{magnitude: 1.602176634e-19,si_units_num: Vec::from([SiUnit::Ampere, SiUnit::Second]),si_units_den: Vec::from([])},
            SiConstant::BoltzmannConstant => DerivedUnit::Joules.get_value().add_den(SiUnit::Kelvin) * 1.380649e-23,
            SiConstant::AvogadroConstant =>  DerivedQuantities::Scalar.get_value().add_den(SiUnit::Mole) * 6.02214076e23_f64,
            SiConstant::LuminousEfficacy => Value{magnitude: 683_f64,si_units_num: Vec::from([SiUnit::Candela,SiUnit::Second,SiUnit::Second,SiUnit::Second]),si_units_den: Vec::from([SiUnit::Kilogram,SiUnit::Metre,SiUnit::Metre])},
            SiConstant::HyperfineTransitionFrequency => DerivedUnit::Hertz.get_value() * 9192631770_i64,
            SiConstant::GravitationalConstant => DerivedQuantities::Force.get_value() * DerivedQuantities::Area.get_value().add_den(SiUnit::Kilogram).add_den(SiUnit::Kilogram) * 6.67430e-11
        }
    }
}

