#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

use crate::error::PhysicError;
use collection_literals::collection;
use hashbrown::HashMap;
use lazy_static::lazy_static;

use alloc::string::{ToString,String};

//ElectricCurrent is a measurement of a flow of electric charge stored as an int64 nano Ampere.
pub type ElectricCurrent = i64;
pub const NanoAmpere: ElectricCurrent = 1;
pub const MicroAmpere: ElectricCurrent = 1000 * NanoAmpere;
pub const MilliAmpere: ElectricCurrent = 1000 * MicroAmpere;
pub const Ampere: ElectricCurrent = 1000 * MilliAmpere;
pub const KiloAmpere: ElectricCurrent = 1000 * Ampere;
pub const MegaAmpere: ElectricCurrent = 1000 * KiloAmpere;
pub const GigaAmpere: ElectricCurrent = 1000 * MegaAmpere;
pub const maxElectricCurrent: ElectricCurrent = 9223372036854775807 * NanoAmpere;
pub const minElectricCurrent: ElectricCurrent = -9223372036854775807 * NanoAmpere;

//ElectricPotential is a measurement of electric potential stored as an int64 nano Volt.
pub type ElectricPotential = i64;
// Volt is W/A, kg⋅m²/s³/A.
pub const NanoVolt: ElectricPotential = 1;
pub const MicroVolt: ElectricPotential = 1000 * NanoVolt;
pub const MilliVolt: ElectricPotential = 1000 * MicroVolt;
pub const Volt: ElectricPotential = 1000 * MilliVolt;
pub const KiloVolt: ElectricPotential = 1000 * Volt;
pub const MegaVolt: ElectricPotential = 1000 * KiloVolt;
pub const GigaVolt: ElectricPotential = 1000 * MegaVolt;
pub const maxElectricPotential: ElectricPotential = 9223372036854775807 * NanoVolt;
pub const minElectricPotential: ElectricPotential = -9223372036854775807 * NanoVolt;

//ElectricResistance is a measurement of the difficulty to pass an electric current through a conductor stored as an int64 nano Ohm.
pub type ElectricResistance = i64;
// Ohm is V/A, kg⋅m²/s³/A².
pub const NanoOhm: ElectricResistance = 1;
pub const MicroOhm: ElectricResistance = 1000 * NanoOhm;
pub const MilliOhm: ElectricResistance = 1000 * MicroOhm;
pub const Ohm: ElectricResistance = 1000 * MilliOhm;
pub const KiloOhm: ElectricResistance = 1000 * Ohm;
pub const MegaOhm: ElectricResistance = 1000 * KiloOhm;
pub const GigaOhm: ElectricResistance = 1000 * MegaOhm;
pub const maxElectricResistance: ElectricResistance = 9223372036854775807 * NanoOhm;
pub const minElectricResistance: ElectricResistance = -9223372036854775807 * NanoOhm;

//Power is a measurement of  Power stored as a nano watts.
pub type Power = i64;
// Watt is unit of Power J/s, kg⋅m²⋅s⁻³
pub const NanoWatt: Power = 1;
pub const MicroWatt: Power = 1000 * NanoWatt;
pub const MilliWatt: Power = 1000 * MicroWatt;
pub const Watt: Power = 1000 * MilliWatt;
pub const KiloWatt: Power = 1000 * Watt;
pub const MegaWatt: Power = 1000 * KiloWatt;
pub const GigaWatt: Power = 1000 * MegaWatt;
pub const maxPower: Power = 9223372036854775807 * NanoWatt;
pub const minPower: Power = -9223372036854775807 * NanoWatt;


pub trait ToStringPhysic_potential {
    fn to_string_physic_potential(self) -> String;
}

impl ToStringPhysic_potential for ElectricPotential {
    fn to_string_physic_potential(self) -> String {
        return nanoAsString(self) + "V";
    }
}

pub trait ToStringPhysic_power {
    fn to_string_physic_power(self) -> String;
}

impl ToStringPhysic_power for Power {
    fn to_string_physic_power(self) -> String {
        return nanoAsString(self) + "W";
    }
}

pub trait ToStringPhysic_current {
    fn to_string_physic_current(self) -> String;
}

impl ToStringPhysic_current for ElectricCurrent {
    fn to_string_physic_current(self) -> String {
        return nanoAsString(self) + "A";
    }
}

pub trait ToStringPhysic_resistance {
    fn to_string_physic_resistance(self) -> String;
}

impl ToStringPhysic_resistance for ElectricResistance {
    fn to_string_physic_resistance(self) -> String {
        return nanoAsString(self) + "Ω";
    }
}

fn nanoAsString(mut v: i64) -> String {
    let mut sign: String = String::from("");
    if v < 0 {
        if v == -9223372036854775808 {
            v = v + 1;
        }
        sign = String::from("-");
        v = -v;
    }
    let mut frac: i32 = Default::default();
    let mut base: i32 = Default::default();
    let mut precision: i64 = Default::default();
    let mut unit: String = String::from("");
    let value_option = Option::Some(v);
    match value_option {
        Some(v) if v >= 999999500000000001 => {
            precision = v % 1000000000000000;
            base = (v / 1000000000000000) as i32;
            if precision > 500000000000000 {
                base = base + 1;
            }
            frac = base % 1000;
            base = base / 1000;
            unit = String::from("G");
        }
        Some(v) if v >= 999999500000001 => {
            precision = v % 1000000000000;
            base = (v / 1000000000000) as i32;
            if precision > 500000000000 {
                base = base + 1;
            }
            frac = base % 1000;
            base = base / 1000;
            unit = String::from("M");
        }
        Some(v) if v >= 999999500001 => {
            precision = v % 1000000000;
            base = (v / 1000000000) as i32;
            if precision > 500000000 {
                base = base + 1;
            }
            frac = base % 1000;
            base = base / 1000;
            unit = String::from("k");
        }
        Some(v) if v >= 999999501 => {
            precision = v % 1000000;
            base = (v / 1000000) as i32;
            if precision > 500000 {
                base = base + 1;
            }
            frac = base % 1000;
            base = base / 1000;
            unit = String::from("");
        }
        Some(v) if v >= 1000000 => {
            precision = v % 1000;
            base = (v / 1000) as i32;
            if precision > 500 {
                base = base + 1;
            }
            frac = base % 1000;
            base = base / 1000;
            unit = String::from("m");
        }
        Some(v) if v >= 1000 => {
            frac = (v as i32) % 1000;
            base = (v as i32) / 1000;
            unit = String::from("µ");
        }
        Some(v) if 0 < v && v < 1000 => {
            base = v as i32;
            unit = String::from("n");
        }
        Some(v) if v == 0 => {
            return String::from("0");
        }
        None => {}
        _ => panic!(),
    }

    if frac == 0 {
        return sign + &base.to_string() + &unit;
    }
    return sign + &base.to_string() + &String::from(".") + &prefixZeros(3, frac) + &unit;
}

fn prefixZeros(digits: i32, v: i32) -> String {
    let mut s = v.to_string();
    let mut str_len = s.len() as i32;
    while str_len < digits {
        s = String::from("0") + &s;
        str_len += 1;
    }
    return s;
}

pub const maxInt64: i64 = 9223372036854775807;

#[derive(Clone, Copy, Default)]
pub struct decimal {
    base: u64,
    exp: i32,
    neg: bool,
}

// Positive powers of 10 in the form such that powerOF10[index] = 10^index.
lazy_static!{
    pub static ref PowerOfTen: HashMap<u64,u64> =  {
        let powerOfTen: HashMap<u64, u64> = collection! {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            4 => 10000,
            5 => 100000,
            6 => 1000000,
            7 => 10000000,
            8 => 100000000,
            9 => 1000000000,
            10 => 10000000000,
            11 => 100000000000,
            12 => 1000000000000,
            13 => 10000000000000,
            14 => 100000000000000,
            15 => 1000000000000000,
            16 => 10000000000000000,
            17 => 100000000000000000,
            18 => 1000000000000000000,
    };
    powerOfTen
};

}

// Converts from decimal to int64.
//
// Scale is combined with the decimal exponent to maximise the resolution and is
// in powers of ten.
//
// Returns true if the value overflowed.
fn dtoi(d: decimal, scale: i32) -> (i64, bool) {
    // Get the total magnitude of the number.
    // a^x * b^y = a*b^(x+y) since scale is of the order unity this becomes
    // 1^x * b^y = b^(x+y).
    // mag must be positive to use as index in to powerOf10 array.
    let mut u = d.base;
    //println!("dtoi -> d.base = {:?}", d.base);
    let mut mag = d.exp + scale;
    //println!("dtoi -> mag = {:?}", mag);
    if mag < 0 {
        mag = -mag;
    }
    let mut n: i64 = 0;
    if mag > 18 {
        return (0, true);
    }
    // scale == 9 means unit == 0
    if scale == 9 && 0 < d.base && d.base < 1000 {
        mag = d.exp + scale - 1;
        u *= PowerOfTen.get(&(mag as u64)).unwrap();
        n = u as i64;
        if d.neg {
            n = -n;
        }
        return (n, false);
    }
    let val = Option::Some(mag);
    match val {
        Some(val) if val < 0 => {
            u = (u + PowerOfTen.get(&(mag as u64)).unwrap() / 2) / PowerOfTen.get(&(mag as u64)).unwrap();
        }
        Some(val) if val == 0 => {
            if (u as i64) > maxInt64 {
                return (0, true);
            }
        }
        _ => {
            let check = u * PowerOfTen.get(&(mag as u64)).unwrap();
            if check / PowerOfTen.get(&(mag as u64)).unwrap() != u || check as i64 > maxInt64 {
                return (0, true);
            }
            u *= PowerOfTen.get(&(mag as u64)).unwrap();
        }
    }
    n = u as i64;
    if d.neg {
        n = -n;
    }
    //println!("dtoi -> before result -> n -> {:?}", n);
    return (n, false);
}

//Reverse the string
fn reverse(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars().rev() {
        output.push(c)
    }
    output
}

// Converts a string to a decimal form. The return int is how many bytes of the
// string are considered numeric. The string may contain +-0 prefixes and
// arbitrary suffixes as trailing non number characters are ignored.
// Significant digits are stored without leading or trailing zeros, rather a
// base and exponent is used. Significant digits are stored as uint64, max size
// of significant digits is int64
fn atod(s: &str) -> (decimal, usize, PhysicError) {
    let mut start: usize = 0;
    let mut end = s.len();
    let mut dp: usize = 0;
    let mut last: usize = 0;
    let mut d = decimal::default();

    let mut seenDigit: bool = false;
    let mut seenZero: bool = false;
    let mut isPoint: bool = false;
    let mut seenPlus: bool = false;

    let bytes = s.as_bytes();
    // Strip leading zeros, +/- and mark DP.
    for (i, &item) in bytes.iter().enumerate() {
        let val = Option::Some(item);
        match val {
            Some(val) if val == b'-' => {
                if seenDigit {
                    end = i;
                    break;
                }
                if seenPlus {
                    return (d, 0, PhysicError::BothPlusAndMinussymbols);
                }
                if d.neg {
                    return (d, 0, PhysicError::MultipleMinusSymbols);
                }
                d.neg = true;
                start += 1;
            }
            Some(val) if val == b'+' => {
                if seenDigit {
                    end = i;
                    break;
                }
                if d.neg {
                    return (d, 0, PhysicError::BothPlusAndMinussymbols);
                }
                if seenPlus {
                    return (d, 0, PhysicError::MultiplePlusSymbols);
                }
                seenPlus = true;
                start += 1;
            }
            Some(val) if val == b'.' => {
                if isPoint {
                    return (d, 0, PhysicError::MultipleDecimalPoints);
                }
                isPoint = true;
                dp = i;
                if !seenDigit {
                    start += 1;
                }
            }
            Some(val) if val == b'0' => {
                if !seenDigit {
                    start += 1;
                }
                seenZero = true;
            }
            Some(val) if val >= b'1' && val <= b'9' => {
                seenDigit = true;
            }
            _ => {
                if !seenDigit && !seenZero {
                    return (d, 0, PhysicError::ErrNotANumber);
                }
                end = i;
            }
        }
    }
    last = end;
    seenDigit = false;
    let mut exp: i32 = 0;

    // Strip non significant zeros to find base exponent.
    let bytes_part2 = &s[start..end];

    //revert the string
    let bytes_part2_t = reverse(bytes_part2);

    let bytes_part2_temp = bytes_part2_t.as_bytes();

    for (i, &item) in bytes_part2_temp.iter().enumerate() {
        let val = Option::Some(item);
        match val {
            Some(val) if val >= b'1' && val <= b'9' => {
                seenDigit = true;
            }
            Some(val) if val == b'.' => {
                if !seenDigit {
                    end -= 1;
                }
            }
            Some(val) if val == b'0' => {
                if !seenDigit {
                    if i > dp {
                        end -= 1;
                    }
                    if i <= dp || dp == 0 {
                        exp += 1;
                    }
                }
            }
            _ => {
                last -= 1;
                end -= 1;
            }
        }
    }

    let bytes_part3 = &bytes[start..end];
    for (_, &item) in bytes_part3.iter().enumerate() {
        if item >= b'0' && item <= b'9' {
            // *10 is decimal shift left.
            d.base *= 10;
            //Convert ascii digit into number
            let check = d.base + ((item - b'0') as u64);
            // Check should always be larger than u unless we have overflowed.
            // Similarly if check > max it will overflow when converted to int64.
            if check < d.base || (check as i64) > maxInt64 {
                if d.neg {
                    return (d, 0, PhysicError::ErrOverFlowsInt64Negative);
                }
                return (d, 0, PhysicError::ErrOverFlowsInt64);
            }
            d.base = check;
            // println!("atod -> the last d.base -> {:?}", d.base);
        } else if item != b'.' {
            return (d, 0, PhysicError::ErrNotANumber);
        }
    }
    if !isPoint {
        d.exp = exp;
    } else {
        if dp > start && dp < end {
            // Decimal Point is in the middle of a number.
            end -= 1;
        }
        // Find the exponent based on decimal point distance from left and the
        // length of the number.
        d.exp = ((dp - start) - (end - start)) as i32;
        if dp <= start {
            // Account for numbers of the form 1 > n < -1 eg 0.0001.
            d.exp += 1;
        }
    }

    return (d, last, PhysicError::Null);
}

// valueOfUnitString is a helper for converting a string and a prefix in to a
// physic unit. It can be used when characters of the units do not conflict with
// any of the SI prefixes.
fn valueOfUnitString(s: &str, base: prefix) -> (i64, usize, PhysicError) {
    let str_copy = s.clone();
    let (d, mut n, err) = atod(str_copy);
    if err != PhysicError::Null {
        return (0, n, err);
    }
    //let mut si: prefix = Unit;
    let mut scale: i32 = 0;
    if n != str_copy.len() {
        let r = &str_copy[n..];
        if r.len() == 0 {
            return (0, 0, PhysicError::UnexpectedEndOfString(r.to_string()));
        }
        let r_vec = r.as_bytes().to_vec();
        //println!("valuesOfUnitString -> r_vec -> {:?}", r_vec);
        //最终结果是其转换为u8
        //let markPoint = r_vec.len();
        let r_byte: u8 = r_vec[0];
        // println!("valuesOfUnitString -> r_byte -> {:?}", r_byte);
        // let mut siSize: usize = 0;
        let (si, siSize) = parseSIPrefix(r_byte);
        scale = si - base;
        /*
        println!(
            "valuesOfUnitString -> si,siSize,scale -> {:?},{:?},{:?}",
            si, siSize, scale
        );
        */
        n += siSize;
    }
    let (v, overflow) = dtoi(d, scale);
    if overflow {
        if d.neg {
            return (-maxInt64, 0, PhysicError::ErrOverFlowsInt64Negative);
        }
        return (maxInt64, 0, PhysicError::ErrOverFlowsInt64);
    }
    return (v, n, PhysicError::Null);
}

pub type prefix = i32;

pub const Pico: prefix = -12;
pub const Nano: prefix = -9;
pub const Micro: prefix = -6;
pub const Milli: prefix = -3;
pub const Unit: prefix = 0;
pub const Deca: prefix = 1;
pub const Hecto: prefix = 2;
pub const Kilo: prefix = 3;
pub const Mega: prefix = 6;
pub const Giga: prefix = 9;
pub const Tera: prefix = 12;

fn parseSIPrefix(r: u8) -> (prefix, usize) {
    let r_copy = r.clone();
    let val = Option::Some(r_copy);
    match val {
        Some(val) if val == b'p' => {
            return (Pico, "p".len());
        }
        Some(val) if val == b'n' => {
            return (Nano, "n".len());
        }
        Some(val) if val == b'u' => {
            return (Micro, "u".len());
        }
        // \xB5 => µ
        Some(val) if val == b'\xB5' => {
            return (Micro, "µ".len());
        }
        Some(val) if val == b'm' => {
            return (Milli, "m".len());
        }
        Some(val) if val == b'k' => {
            return (Kilo, "k".len());
        }
        Some(val) if val == b'M' => {
            return (Mega, "M".len());
        }
        Some(val) if val == b'G' => {
            return (Giga, "G".len());
        }
        Some(val) if val == b'T' => {
            return (Tera, "T".len());
        }
        None => {
            return (Unit, 0);
        }
        _ => {
            return (Unit, 0);
        }
    }
}

pub trait PhysicPowerSet {
    type T;
    type E;
    fn setPower(str: &str) -> Result<Self::T, Self::E>;
}

// Set sets the Power to the value represented by s. Units are to
// be provided in "W" with an optional SI prefix: "p", "n", "u", "µ", "m", "k",
// "M", "G" or "T".
impl PhysicPowerSet for Power {
    type T = Power;
    type E = PhysicError;
    fn setPower(str: &str) -> Result<Self::T, Self::E> {
        let (v, n, err) = valueOfUnitString(str, Nano);
        let err_copy = err.clone();
        let str_copy = str.clone();
        if err != PhysicError::Null {
            let val = Option::Some(err);
            match val {
                Some(val) if val == PhysicError::ErrNotANumber => {
                    if str.contains("W") || str.contains("w") {
                        return Err(PhysicError::ErrNotANumber);
                    } else {
                        return Err(PhysicError::NotNumberUnitErr("W".to_string()));
                    }
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64 => {
                    return Err(PhysicError::MaxValueErr(maxPower.to_string_physic_power()));
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64Negative => {
                    return Err(PhysicError::MinValueErr(minPower.to_string_physic_power()));
                }
                _ => return Err(err_copy),
            }
        } else {
            let r = &str_copy[n..];
            let val = Option::Some(r);
            match val {
                Some(val) if val.contains("W") || val.contains("w") => {
                    let p: Power = v as Power;
                    //println!("Current_Set -> {:?}", p.to_string_physic_power());
                    Ok::<Self::T, Self::E>(p);
                }
                Some(val) if val == "" => {
                    return Err(PhysicError::NotUnitErr("W".to_string()));
                }
                _ => {
                    if r.len() != 0 {
                        return Err(PhysicError::UnknownUnitPrefixErr(
                            r.to_string(),
                            "p,n,u,µ,m,k,M,G or T".to_string(),
                        ));
                    }
                    return Err(PhysicError::IncorrectUnitErr("W".to_string()));
                }
            }
            Ok(v as Power)
        }
    }
}

pub trait PhysicElectricCurrentSet {
    type T;
    type E;
    fn setCurrent(str: &str) -> Result<Self::T, Self::E>;
}

// Set sets the ElectricCurrent to the value represented by s. Units are to
// be provided in "A" with an optional SI prefix: "p", "n", "u", "µ", "m", "k",
// "M", "G" or "T".
impl PhysicElectricCurrentSet for ElectricCurrent {
    type T = ElectricCurrent;
    type E = PhysicError;
    fn setCurrent(str: &str) -> Result<Self::T, Self::E> {
        let (v, n, err) = valueOfUnitString(str, Nano);
        let err_copy = err.clone();
        let str_copy = str.clone();
        if err != PhysicError::Null {
            let val = Option::Some(err);
            match val {
                Some(val) if val == PhysicError::ErrNotANumber => {
                    if str.contains("A") || str.contains("a") {
                        return Err(PhysicError::ErrNotANumber);
                    } else {
                        return Err(PhysicError::NotNumberUnitErr("A".to_string()));
                    }
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64 => {
                    return Err(PhysicError::MaxValueErr(
                        maxElectricCurrent.to_string_physic_current(),
                    ));
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64Negative => {
                    return Err(PhysicError::MinValueErr(
                        minElectricCurrent.to_string_physic_current(),
                    ));
                }
                _ => return Err(err_copy),
            }
        } else {
            let r = &str_copy[n..];
            let val = Option::Some(r);
            match val {
                Some(val) if val.contains("A") || val.contains("a") => {
                    let c: ElectricCurrent = v as ElectricCurrent;
                    //println!("Current_Set -> {:?}", c.to_string_physic_current());
                    Ok::<Self::T, Self::E>(c);
                }
                Some(val) if val == "" => {
                    return Err(PhysicError::NotUnitErr("A".to_string()));
                }
                _ => {
                    if r.len() != 0 {
                        return Err(PhysicError::UnknownUnitPrefixErr(
                            r.to_string(),
                            "p,n,u,µ,m,k,M,G or T".to_string(),
                        ));
                    }
                    return Err(PhysicError::IncorrectUnitErr("A".to_string()));
                }
            }
            Ok(v as ElectricCurrent)
        }
    }
}

pub trait PhysicElectricResistanceSet {
    type T;
    type E;
    fn setResistance(str: &str) -> Result<Self::T, Self::E>;
}

// Set sets the ElectricResistance to the value represented by s. Units are to
// be provided in "Ohm", or "Ω" with an optional SI prefix: "p", "n", "u", "µ",
// "m", "k", "M", "G" or "T".
impl PhysicElectricResistanceSet for ElectricResistance {
    type T = ElectricResistance;
    type E = PhysicError;
    fn setResistance(str: &str) -> Result<Self::T, Self::E> {
        let (v, n, err) = valueOfUnitString(str, Nano);
        let err_copy = err.clone();
        let str_copy = str.clone();
        if err != PhysicError::Null {
            let val = Option::Some(err);
            match val {
                Some(val) if val == PhysicError::ErrNotANumber => {
                    if str.contains("Ohm") || str.contains("Ohm") || str.contains("Ω") {
                        return Err(PhysicError::ErrNotANumber);
                    } else {
                        return Err(PhysicError::NotNumberUnitErr("Ohm or Ω".to_string()));
                    }
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64 => {
                    return Err(PhysicError::MaxValueErr(
                        maxElectricResistance.to_string_physic_resistance(),
                    ));
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64Negative => {
                    return Err(PhysicError::MinValueErr(
                        minElectricResistance.to_string_physic_resistance(),
                    ));
                }
                _ => return Err(err_copy),
            }
        } else {
            let r = &str_copy[n..];
            let val = Option::Some(r);
            match val {
                Some(val) if val.contains("Ohm") || val.contains("ohm") || val.contains("Ω") => {
                    let c: ElectricResistance = v as ElectricResistance;
                    Ok::<Self::T, Self::E>(c);
                }
                Some(val) if val == "" => {
                    return Err(PhysicError::NotUnitErr("Ohm or Ω".to_string()));
                }
                _ => {
                    if r.len() != 0 {
                        return Err(PhysicError::UnknownUnitPrefixErr(
                            r.to_string(),
                            "p,n,u,µ,m,k,M,G or T".to_string(),
                        ));
                    }
                    return Err(PhysicError::IncorrectUnitErr("Ohm or Ω".to_string()));
                }
            }
            Ok(v as ElectricResistance)
        }
    }
}

pub trait PhysicElectricPotentialSet {
    type T;
    type E;
    fn setVoltage(str: &str) -> Result<Self::T, Self::E>;
}

// Set sets the ElectricPotential to the value represented by s. Units are to
// be provided in "V" with an optional SI prefix: "p", "n", "u", "µ", "m", "k",
// "M", "G" or "T".
impl PhysicElectricPotentialSet for ElectricPotential {
    type T = ElectricPotential;
    type E = PhysicError;
    fn setVoltage(str: &str) -> Result<Self::T, Self::E> {
        let (v, n, err) = valueOfUnitString(str, Nano);
        let err_copy = err.clone();
        let str_copy = str.clone();
        if err != PhysicError::Null {
            let val = Option::Some(err);
            match val {
                Some(val) if val == PhysicError::ErrNotANumber => {
                    if str.contains("V") || str.contains("v") {
                        return Err(PhysicError::ErrNotANumber);
                    } else {
                        return Err(PhysicError::NotNumberUnitErr("V".to_string()));
                    }
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64 => {
                    return Err(PhysicError::MaxValueErr(
                        maxElectricPotential.to_string_physic_potential(),
                    ));
                }
                Some(val) if val == PhysicError::ErrOverFlowsInt64Negative => {
                    return Err(PhysicError::MinValueErr(
                        minElectricPotential.to_string_physic_potential(),
                    ));
                }
                _ => return Err(err_copy),
            }
        } else {
            let r = &str_copy[n..];
            let val = Option::Some(r);
            match val {
                Some(val) if val.contains("V") || val.contains("v") => {
                    let c: ElectricPotential = v as ElectricPotential;
                    Ok::<Self::T, Self::E>(c);
                }
                Some(val) if val == "" => {
                    return Err(PhysicError::NotUnitErr("V,v".to_string()));
                }
                _ => {
                    if r.len() != 0 {
                        return Err(PhysicError::UnknownUnitPrefixErr(
                            r.to_string(),
                            "p,n,u,µ,m,k,M,G or T".to_string(),
                        ));
                    }
                    return Err(PhysicError::IncorrectUnitErr("V".to_string()));
                }
            }
            Ok(v as ElectricPotential)
        }
    }
}
