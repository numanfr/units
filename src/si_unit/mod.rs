
use core::fmt;
#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy,PartialOrd,Ord)]

/// Si units enum 
pub enum SiUnit {

    ///Metres
    Metres,

    ///Seconds
    Seconds,

    ///Kilogram
    Kilogram,

    ///Ampere
    Ampere,

    ///Kelvin
    Kelvin,

    ///Mole
    Mole,

    ///Candela
    Candela
}


///Implement Display
///
/// ```rust
/// let kg = SiUnit::Kilogram;
/// println!("{}", kg);
/// ```
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
