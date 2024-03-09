#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_must_use)]


use core::convert::TryInto;
use core::fmt;

use embedded_hal::i2c::I2c;

pub const INA219_ADDR: u8 = 0x40;


/** mask for bus voltage range **/
const INA219_CONFIG_BVOLTAGERANGE_MASK: u16 = 0x2000; // Bus Voltage Range Mask

/** bus voltage range values **/
const INA219_CONFIG_BVOLTAGERANGE_16V: u16 = (0x0000);
// 0-16V Range
const INA219_CONFIG_BVOLTAGERANGE_32V: u16 = (0x2000); // 0-32V Range

/** mask for gain bits **/
const INA219_CONFIG_GAIN_MASK: u16 = 0x1800; // Gain Mask

/** values for gain bits **/
const INA219_CONFIG_GAIN_1_40MV: u16 = (0x0000);
// Gain 1, 40mV Range
const INA219_CONFIG_GAIN_2_80MV: u16 = (0x0800);
// Gain 2, 80mV Range
const INA219_CONFIG_GAIN_4_160MV: u16 = (0x1000);
// Gain 4, 160mV Range
const INA219_CONFIG_GAIN_8_320MV: u16 = (0x1800); // Gain 8, 320mV Range

/** mask for bus ADC resolution bits **/
const INA219_CONFIG_BADCRES_MASK: u16 = 0x0780;

/** values for bus ADC resolution **/
const INA219_CONFIG_BADCRES_9BIT: u16 = (0x0000);
// 9-bit bus res = 0..511
const INA219_CONFIG_BADCRES_10BIT: u16 = (0x0080);
// 10-bit bus res = 0..1023
const INA219_CONFIG_BADCRES_11BIT: u16 = (0x0100);
// 11-bit bus res = 0..2047
const INA219_CONFIG_BADCRES_12BIT: u16 = (0x0180);
// 12-bit bus res = 0..4097
const INA219_CONFIG_BADCRES_12BIT_2S_1060US: u16 = (0x0480);
// 2 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_4S_2130US: u16 = (0x0500);
// 4 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_8S_4260US: u16 = (0x0580);
// 8 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_16S_8510US: u16 = (0x0600);
// 16 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_32S_17MS: u16 = (0x0680);
// 32 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_64S_34MS: u16 = (0x0700);
// 64 x 12-bit bus samples averaged together
const INA219_CONFIG_BADCRES_12BIT_128S_69MS: u16 = (0x0780); // 128 x 12-bit bus samples averaged together

/** mask for shunt ADC resolution bits **/
const INA219_CONFIG_SADCRES_MASK: u16 = 0x0078; // Shunt ADC Resolution and Averaging Mask

/** values for shunt ADC resolution **/
const INA219_CONFIG_SADCRES_9BIT_1S_84US: u16 = (0x0000);
// 1 x 9-bit shunt sample
const INA219_CONFIG_SADCRES_10BIT_1S_148US: u16 = (0x0008);
// 1 x 10-bit shunt sample
const INA219_CONFIG_SADCRES_11BIT_1S_276US: u16 = (0x0010);
// 1 x 11-bit shunt sample
const INA219_CONFIG_SADCRES_12BIT_1S_532US: u16 = (0x0018);
// 1 x 12-bit shunt sample
const INA219_CONFIG_SADCRES_12BIT_2S_1060US: u16 = (0x0048);
// 2 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_4S_2130US: u16 = (0x0050);
// 4 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_8S_4260US: u16 = (0x0058);
// 8 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_16S_8510US: u16 = (0x0060);
// 16 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_32S_17MS: u16 = (0x0068);
// 32 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_64S_34MS: u16 = (0x0070);
// 64 x 12-bit shunt samples averaged together
const INA219_CONFIG_SADCRES_12BIT_128S_69MS: u16 = (0x0078); // 128 x 12-bit shunt samples averaged together

/** mask for operating mode bits **/
const INA219_CONFIG_MODE_MASK: u16 = 0x0007; // Operating Mode Mask

/** values for operating mode **/
const INA219_CONFIG_MODE_POWERDOWN: u16 = 0x00;
/**< power down */
const INA219_CONFIG_MODE_SVOLT_TRIGGERED: u16 = 0x01;
/**< shunt voltage triggered */
const INA219_CONFIG_MODE_BVOLT_TRIGGERED: u16 = 0x02;
/**< bus voltage triggered */
const INA219_CONFIG_MODE_SANDBVOLT_TRIGGERED: u16 = 0x03;
/**< shunt and bus voltage triggered */
const INA219_CONFIG_MODE_ADCOFF: u16 = 0x04;
/**< ADC off */
const INA219_CONFIG_MODE_SVOLT_CONTINUOUS: u16 = 0x05;
/**< shunt voltage continuous */
const INA219_CONFIG_MODE_BVOLT_CONTINUOUS: u16 = 0x06;
/**< bus voltage continuous */
const INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS: u16 = 0x07;

pub struct PowerMonitor {
    Shunt: f32,
    Voltage: f32,
    Current: f32,
    Power: f32,
}

impl PowerMonitor {
    pub fn new(shunt: f32, voltage: f32, current: f32, power: f32) -> PowerMonitor {
        PowerMonitor {
            Shunt: shunt,
            Voltage: voltage,
            Current: current,
            Power: power,
        }
    }
}

impl fmt::Display for PowerMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Voltage V = {},\nShunt_Voltage mV = {},\nCurrent mA = {},\nPower mW = {}",
            self.Voltage, self.Shunt, self.Current, self.Power
        )
    }
}

impl fmt::Debug for PowerMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: PowerMonitor \n{{\n\tVoltage V= {},\n\tShunt_Voltage mV = {},\n\tCurrent mA = {},\n\tPower mW = {} \n}}",
               self.Voltage,
               self.Shunt,
               self.Current,
               self.Power
        )
    }
}

pub struct Register;

impl Register {
    pub const Configuration: u8 = 0x00;
    pub const ShuntVoltage: u8 = 0x01;
    pub const BusVoltage: u8 = 0x02;
    pub const Power: u8 = 0x03;
    pub const Current: u8 = 0x04;
    pub const Calibration: u8 = 0x05;
}

pub struct INA219<I2C> {
    i2c: I2C,
    cal_value: u16,
    currentDivider_mA: i16,
    powerMultiplier_mW: f32,
}

impl<I2C: I2c> INA219<I2C>
{
    pub fn new(i2c: I2C) -> INA219<I2C> {
        INA219 {
            i2c,
            cal_value: 0,
            currentDivider_mA: 0,
            powerMultiplier_mW: 0.0,
        }
    }

    pub fn init(&mut self) -> Result<(), I2C::Error> {
        self.setCalibration_16V_400mA()
    }

    fn setCalibration_32V_2A(&mut self) -> Result<(), I2C::Error> {
        // By default we use a pretty huge range for the input voltage,
        // which probably isn't the most appropriate choice for system
        // that don't use a lot of power.  But all of the calculations
        // are shown below if you want to change the settings.  You will
        // also need to change any relevant register settings, such as
        // setting the VBUS_MAX to 16V instead of 32V, etc.

        // VBUS_MAX = 32V             (Assumes 32V, can also be set to 16V)
        // VSHUNT_MAX = 0.32          (Assumes Gain 8, 320mV, can also be 0.16, 0.08,
        // 0.04) RSHUNT = 0.1               (Resistor value in ohms)

        // 1. Determine max possible current
        // MaxPossible_I = VSHUNT_MAX / RSHUNT
        // MaxPossible_I = 3.2A

        // 2. Determine max expected current
        // MaxExpected_I = 2.0A

        // 3. Calculate possible range of LSBs (Min = 15-bit, Max = 12-bit)
        // MinimumLSB = MaxExpected_I/32767
        // MinimumLSB = 0.000061              (61uA per bit)
        // MaximumLSB = MaxExpected_I/4096
        // MaximumLSB = 0,000488              (488uA per bit)

        // 4. Choose an LSB between the min and max values
        //    (Preferrably a roundish number close to MinLSB)
        // CurrentLSB = 0.0001 (100uA per bit)

        // 5. Compute the calibration register
        // Cal = trunc (0.04096 / (Current_LSB * RSHUNT))
        // Cal = 4096 (0x1000)

        self.cal_value = 4096;

        // 6. Calculate the power LSB
        // PowerLSB = 20 * CurrentLSB
        // PowerLSB = 0.002 (2mW per bit)

        // 7. Compute the maximum current and shunt voltage values before overflow
        //
        // Max_Current = Current_LSB * 32767
        // Max_Current = 3.2767A before overflow
        //
        // If Max_Current > Max_Possible_I then
        //    Max_Current_Before_Overflow = MaxPossible_I
        // Else
        //    Max_Current_Before_Overflow = Max_Current
        // End If
        //
        // Max_ShuntVoltage = Max_Current_Before_Overflow * RSHUNT
        // Max_ShuntVoltage = 0.32V
        //
        // If Max_ShuntVoltage >= VSHUNT_MAX
        //    Max_ShuntVoltage_Before_Overflow = VSHUNT_MAX
        // Else
        //    Max_ShuntVoltage_Before_Overflow = Max_ShuntVoltage
        // End If

        // 8. Compute the Maximum Power
        // MaximumPower = Max_Current_Before_Overflow * VBUS_MAX
        // MaximumPower = 3.2 * 32V
        // MaximumPower = 102.4W

        // Set multipliers to convert raw current/power values
        self.currentDivider_mA = 10; // Current LSB = 100uA per bit (1000/100 = 10)
        self.powerMultiplier_mW = 2.0; // Power LSB = 1mW per bit (2/1)

        let buf = self.cal_value.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[Register::Calibration, buf[0], buf[1]],
        );

        let config = INA219_CONFIG_BVOLTAGERANGE_32V |
            INA219_CONFIG_GAIN_8_320MV | INA219_CONFIG_BADCRES_12BIT |
            INA219_CONFIG_SADCRES_12BIT_1S_532US |
            INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS;

        let buf = config.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[
                Register::Configuration,
                buf[0],
                buf[1],
            ],
        )
    }

    fn setCalibration_32V_1A(&mut self) -> Result<(), I2C::Error> {
        // By default we use a pretty huge range for the input voltage,
        // which probably isn't the most appropriate choice for system
        // that don't use a lot of power.  But all of the calculations
        // are shown below if you want to change the settings.  You will
        // also need to change any relevant register settings, such as
        // setting the VBUS_MAX to 16V instead of 32V, etc.

        // VBUS_MAX = 32V		(Assumes 32V, can also be set to 16V)
        // VSHUNT_MAX = 0.32	(Assumes Gain 8, 320mV, can also be 0.16, 0.08, 0.04)
        // RSHUNT = 0.1			(Resistor value in ohms)

        // 1. Determine max possible current
        // MaxPossible_I = VSHUNT_MAX / RSHUNT
        // MaxPossible_I = 3.2A

        // 2. Determine max expected current
        // MaxExpected_I = 1.0A

        // 3. Calculate possible range of LSBs (Min = 15-bit, Max = 12-bit)
        // MinimumLSB = MaxExpected_I/32767
        // MinimumLSB = 0.0000305             (30.5uA per bit)
        // MaximumLSB = MaxExpected_I/4096
        // MaximumLSB = 0.000244              (244uA per bit)

        // 4. Choose an LSB between the min and max values
        //    (Preferrably a roundish number close to MinLSB)
        // CurrentLSB = 0.0000400 (40uA per bit)

        // 5. Compute the calibration register
        // Cal = trunc (0.04096 / (Current_LSB * RSHUNT))
        // Cal = 10240 (0x2800)

        self.cal_value = 10240;

        // 6. Calculate the power LSB
        // PowerLSB = 20 * CurrentLSB
        // PowerLSB = 0.0008 (800uW per bit)

        // 7. Compute the maximum current and shunt voltage values before overflow
        //
        // Max_Current = Current_LSB * 32767
        // Max_Current = 1.31068A before overflow
        //
        // If Max_Current > Max_Possible_I then
        //    Max_Current_Before_Overflow = MaxPossible_I
        // Else
        //    Max_Current_Before_Overflow = Max_Current
        // End If
        //
        // ... In this case, we're good though since Max_Current is less than
        // MaxPossible_I
        //
        // Max_ShuntVoltage = Max_Current_Before_Overflow * RSHUNT
        // Max_ShuntVoltage = 0.131068V
        //
        // If Max_ShuntVoltage >= VSHUNT_MAX
        //    Max_ShuntVoltage_Before_Overflow = VSHUNT_MAX
        // Else
        //    Max_ShuntVoltage_Before_Overflow = Max_ShuntVoltage
        // End If

        // 8. Compute the Maximum Power
        // MaximumPower = Max_Current_Before_Overflow * VBUS_MAX
        // MaximumPower = 1.31068 * 32V
        // MaximumPower = 41.94176W


        // Set multipliers to convert raw current/power values
        self.currentDivider_mA = 25; // Current LSB = 100uA per bit (1000/100 = 10)
        self.powerMultiplier_mW = 0.8; // Power LSB = 1mW per bit (2/1)

        let buf = self.cal_value.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[Register::Calibration, buf[0], buf[1]],
        );

        let config = INA219_CONFIG_BVOLTAGERANGE_32V |
            INA219_CONFIG_GAIN_8_320MV | INA219_CONFIG_BADCRES_12BIT |
            INA219_CONFIG_SADCRES_12BIT_1S_532US |
            INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS;

        let buf = config.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[
                Register::Configuration,
                buf[0],
                buf[1],
            ],
        )
    }

    fn setCalibration_16V_400mA(&mut self) -> Result<(), I2C::Error> {
        // Calibration which uses the highest precision for
        // current measurement (0.1mA), at the expense of
        // only supporting 16V at 400mA max.

        // VBUS_MAX = 16V
        // VSHUNT_MAX = 0.04          (Assumes Gain 1, 40mV)
        // RSHUNT = 0.1               (Resistor value in ohms)

        // 1. Determine max possible current
        // MaxPossible_I = VSHUNT_MAX / RSHUNT
        // MaxPossible_I = 0.4A

        // 2. Determine max expected current
        // MaxExpected_I = 0.4A

        // 3. Calculate possible range of LSBs (Min = 15-bit, Max = 12-bit)
        // MinimumLSB = MaxExpected_I/32767
        // MinimumLSB = 0.0000122              (12uA per bit)
        // MaximumLSB = MaxExpected_I/4096
        // MaximumLSB = 0.0000977              (98uA per bit)

        // 4. Choose an LSB between the min and max values
        //    (Preferrably a roundish number close to MinLSB)
        // CurrentLSB = 0.00005 (50uA per bit)

        // 5. Compute the calibration register
        // Cal = trunc (0.04096 / (Current_LSB * RSHUNT))
        // Cal = 8192 (0x2000)

        self.cal_value = 8192;

        // 6. Calculate the power LSB
        // PowerLSB = 20 * CurrentLSB
        // PowerLSB = 0.001 (1mW per bit)

        // 7. Compute the maximum current and shunt voltage values before overflow
        //
        // Max_Current = Current_LSB * 32767
        // Max_Current = 1.63835A before overflow
        //
        // If Max_Current > Max_Possible_I then
        //    Max_Current_Before_Overflow = MaxPossible_I
        // Else
        //    Max_Current_Before_Overflow = Max_Current
        // End If
        //
        // Max_Current_Before_Overflow = MaxPossible_I
        // Max_Current_Before_Overflow = 0.4
        //
        // Max_ShuntVoltage = Max_Current_Before_Overflow * RSHUNT
        // Max_ShuntVoltage = 0.04V
        //
        // If Max_ShuntVoltage >= VSHUNT_MAX
        //    Max_ShuntVoltage_Before_Overflow = VSHUNT_MAX
        // Else
        //    Max_ShuntVoltage_Before_Overflow = Max_ShuntVoltage
        // End If
        //
        // Max_ShuntVoltage_Before_Overflow = VSHUNT_MAX
        // Max_ShuntVoltage_Before_Overflow = 0.04V

        // 8. Compute the Maximum Power
        // MaximumPower = Max_Current_Before_Overflow * VBUS_MAX
        // MaximumPower = 0.4 * 16V
        // MaximumPower = 6.4W

        // Set multipliers to convert raw current/power values
        self.currentDivider_mA = 20; // Current LSB = 100uA per bit (1000/100 = 10)
        self.powerMultiplier_mW = 1.0; // Power LSB = 1mW per bit (2/1)

        let buf = self.cal_value.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[Register::Calibration, buf[0], buf[1]],
        );

        let config = INA219_CONFIG_BVOLTAGERANGE_16V |
            INA219_CONFIG_GAIN_1_40MV | INA219_CONFIG_BADCRES_12BIT |
            INA219_CONFIG_SADCRES_12BIT_1S_532US |
            INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS;

        let buf = config.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[
                Register::Configuration,
                buf[0],
                buf[1],
            ],
        )
    }


    pub fn shunt_voltage_raw(&mut self) -> Result<i16, I2C::Error> {
        let value = self.read(Register::ShuntVoltage)?;
        Ok(value as i16)
    }

    pub fn voltage_raw(&mut self) -> Result<i16, I2C::Error> {
        let value = self.read(Register::BusVoltage)?;
        Ok(((value >> 3) * 4) as i16)
    }

    pub fn power_raw(&mut self) -> Result<i16, I2C::Error> {
        //need to calibrate first
        let buf = self.cal_value.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[Register::Calibration, buf[0], buf[1]],
        );

        let value = self.read(Register::Power)?;
        Ok(value as i16)
    }

    pub fn current_raw(&mut self) -> Result<i16, I2C::Error> {
        //need to calibrate first
        let buf = self.cal_value.to_be_bytes();
        self.i2c.write(
            INA219_ADDR,
            &[Register::Calibration, buf[0], buf[1]],
        );
        let value = self.read(Register::Current)?;
        Ok(value as i16)
    }

    pub fn getShuntVoltage_mV(&mut self) -> Result<f32, I2C::Error> {
        let value = self.shunt_voltage_raw()?;
        Ok(value as f32 * 0.01)
    }

    pub fn getBusVoltage_V(&mut self) -> Result<f32, I2C::Error> {
        let value = self.voltage_raw()?;
        Ok(value as f32 * 0.001)
    }

    pub fn getCurrent_mA(&mut self) -> Result<f32, I2C::Error> {
        let value = self.current_raw()?;
        Ok((value / self.currentDivider_mA) as f32)
    }

    pub fn getPower_mW(&mut self) -> Result<f32, I2C::Error> {
        let value = self.power_raw()?;
        Ok(value as f32 * self.powerMultiplier_mW)
    }

    pub fn sense(&mut self) -> Result<PowerMonitor, I2C::Error> {
        let shunt = self.getShuntVoltage_mV().unwrap();
        let voltage = self.getBusVoltage_V().unwrap();
        let current = self.getCurrent_mA().unwrap();
        let power = self.getPower_mW().unwrap();
        let pm = PowerMonitor::new(shunt, voltage, current, power);
        Ok(pm)
    }

    fn read(&mut self, register: u8) -> Result<u16, I2C::Error> {
        let mut buf: [u8; 2] = [0x00; 2];
        self.i2c.write(INA219_ADDR, &[register])?;
        self.i2c.read(INA219_ADDR, &mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
}
