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
