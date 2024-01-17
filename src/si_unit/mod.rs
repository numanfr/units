
pub mod si_mod{
    use core::fmt;
    #[derive(Debug,Hash,PartialEq,Eq,Clone,Copy,PartialOrd,Ord)]

    pub enum SiUnit {
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
}