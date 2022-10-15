# ina219

[![crates.io](https://img.shields.io/crates/v/ina219_rs.svg)](https://crates.io/crates/ina219_rs) [![Rust](https://github.com/sndnvaps/ina219/actions/workflows/rust.yml/badge.svg)](https://github.com/sndnvaps/ina219/actions/workflows/rust.yml)

[INA219](http://www.ti.com/product/INA219) current/power monitor driver for Rust

## Example

```bash
cargo build --example values --target=aarch64-unknown-linux-musl
cargo build --example raw_values --target=aarch64-unknown-linux-musl
cargo build --example physic_values --target=aarch64-unknown-linux-musl
```

## support features

1. ina219 feature contain physic
2. physic

# Add this line to Cargo.toml for full feature support

```toml
ina219_rs = { version = "0.3.2", features = ["ina219"] }
```

```rust
//main.rs
extern crate linux_embedded_hal as hal;

extern crate ina219_rs as ina219;

use hal::I2cdev;
use ina219::physic;

use ina219::ina219::{INA219,Opts};

fn main() {

    let device = I2cdev::new("/dev/i2c-1").unwrap();
    let opt = Opts::new(0x42,100 * physic::MilliOhm,1 * physic::Ampere);
    //let opt = Opts::default();
    let mut ina = INA219::new(device,opt);
    ina.init().unwrap();
    let pm = ina.sense().unwrap();
    println!("{:?}",pm);
 /* output
 Debug: PowerMonitor
{
        Voltage = 8.228V,
        Shunt_Voltage = 534µV,
        Current = 1.750A,
        Power = 744mW
}
 */

```

## Only support <strong>physic</strong> featute

```toml
[dependencies.ina219_rs]
version = "0.3.2"
default-features = false # 不包含默认的features,而是通过下面的方式来指定
features = ["physic"]
```

```rust
//main.rs

extern crate ina219_rs as ina219;
use ina219::{
    physic, physic::PhysicElectricCurrentSet, physic::PhysicElectricPotentialSet,
    physic::PhysicPowerSet, physic::ToStringPhysic_current, physic::ToStringPhysic_potential,
    physic::ToStringPhysic_power,
};

fn main() {
    let current_test = physic::ElectricCurrent::setCurrent("+15mA");
    match current_test {
        Ok(v) => println!(
            "current_set is {:?}",
            (v as physic::ElectricCurrent).to_string_physic_current()
        ),
        Err(e) => println!("current_set error = {:?}", e),
    }

    let power_test = physic::Power::setPower("150mW");
    match power_test {
        Ok(p) => println!(
            "Power_set is {:?}",
            (p as physic::Power).to_string_physic_power()
        ),
        Err(e) => println!("Power_set error = {:?}", e),
    }

    let power_test_v1 = physic::Power::setPower("250W");
    match power_test_v1 {
        Ok(p) => println!(
            "Power_set is {:?}",
            (p as physic::Power).to_string_physic_power()
        ),
        Err(e) => println!("Power_set error = {:?}", e),
    }

    let voltage_test = physic::ElectricPotential::setVoltage("100V");
    match voltage_test {
        Ok(v) => println!(
            "voltage_set is {:?}",
            (v as physic::ElectricPotential).to_string_physic_potential()
        ),
        Err(e) => println!("voltage_set error = {:?}", e),
    }

    let voltage_test_v1 = physic::ElectricPotential::setVoltage("100mV");
    match voltage_test_v1 {
        Ok(v) => println!(
            "voltage_set is {:?}",
            (v as physic::ElectricPotential).to_string_physic_potential()
        ),
        Err(e) => println!("voltage_set error = {:?}", e),
    }
}

```
