use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ina219Error {
    #[error("Sense Resistor Value Invalid, found `{0}`")]
    SenseResistorValueInvalid(String),
    #[error("Max Current Invalid,found `{0}`")]
    MaxCurrentInvalid(i64),
}
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PhysicError {
    #[error("contains both plus and minus symbol")]
    BothPlusAndMinussymbols,
    #[error("contains multiple minus symbols")]
    MultipleMinusSymbols,
    #[error("contains multiple plus symbols")]
    MultiplePlusSymbols,
    #[error("contains multiple decimal points")]
    MultipleDecimalPoints,
    #[error("Err Not a Number")]
    ErrNotANumber,
    #[error("Err Over Flows Int64 Negative")]
    ErrOverFlowsInt64Negative,
    #[error("Err Over Flows Int64")]
    ErrOverFlowsInt64,
    #[error("unexpected end of string -> {0}")]
    UnexpectedEndOfString(String),
    #[error("no unit provided; need {0}")]
    NotUnitErr(String),
    #[error("unknown unit provided; need {0}")]
    IncorrectUnitErr(String),
    #[error("unknown unit prefix; valid prefixes for {0} are {1}")]
    UnknownUnitPrefixErr(String, String),
    #[error("maximum value is {0}")]
    MaxValueErr(String),
    #[error("minimum value is {0}")]
    MinValueErr(String),
    #[error("does not contain number or unit {0}")]
    NotNumberUnitErr(String),
    #[error("")]
    Null,
}
