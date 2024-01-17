pub mod si_constants_mod {

    use crate::Value;
    use crate::DerivedQuantities;
    use crate::DerivedUnit;
    use crate::SiUnit;

    pub enum SiConstant {
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
        pub fn get_value(&self) -> Value{
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
    
}