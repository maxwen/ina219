use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PhysicError {
    BothPlusAndMinussymbols,
    MultipleMinusSymbols,
    MultiplePlusSymbols,
    MultipleDecimalPoints,
    ErrNotANumber,
    ErrOverFlowsInt64Negative,
    ErrOverFlowsInt64,
    UnexpectedEndOfString(String),
    NotUnitErr(String),
    IncorrectUnitErr(String),
    UnknownUnitPrefixErr(String, String),
    MaxValueErr(String),
    MinValueErr(String),
    NotNumberUnitErr(String),
    Null,
}

impl fmt::Display for PhysicError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PhysicError::BothPlusAndMinussymbols => write!(fmt, "contains both plus and minus symbol"),
            PhysicError::ErrNotANumber => write!(fmt,"Err Not a Number"),
            PhysicError::ErrOverFlowsInt64 => write!(fmt,"Err Over Flows Int64"),
            PhysicError::ErrOverFlowsInt64Negative => write!(fmt,"Err Over Flows Int64 Negative"),
            PhysicError::IncorrectUnitErr(ref unit) => write!(fmt,"unknown unit provided; need {}",unit),
            PhysicError::MaxValueErr(ref max_val) => write!(fmt,"maximum value is {}",max_val),
            PhysicError::MinValueErr(ref min_val) => write!(fmt,"minimum value is {}",min_val),
            PhysicError::MultipleDecimalPoints => write!(fmt,"contains multiple decimal points"),
            PhysicError::MultipleMinusSymbols => write!(fmt,"contains multiple minus symbols"),
            PhysicError::MultiplePlusSymbols => write!(fmt,"contains multiple plus symbols"),
            PhysicError::NotNumberUnitErr(ref unit) =>write!(fmt,"does not contain number or unit {}",unit),
            PhysicError::NotUnitErr(ref unit) => write!(fmt,"no unit provided; need {}",unit),
            PhysicError::Null => write!(fmt,""),
            PhysicError::UnexpectedEndOfString(ref unexpect) => write!(fmt,"unexpected end of string -> {}",unexpect),
            PhysicError::UnknownUnitPrefixErr(ref provide,ref need ) => write!(fmt,"unknown unit prefix; valid prefixes for {} are {}",provide,need),
        }
    }
    
    
}