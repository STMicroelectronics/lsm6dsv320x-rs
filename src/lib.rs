#![no_std]
pub mod prelude;
pub mod register;

use core::fmt::Debug;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_hal::spi::SpiDevice;
use half::f16;

use prelude::*;

use st_mems_bus::{BusOperation, MemBankFunctions, i2c::*, spi::*, *};

/// Driver for the LSM6DSV320X sensor.
///
/// The struct takes a bus and a timer hardware object to write to the
/// registers.
/// The bus is generalized over the BusOperation trait, allowing the use
/// of I2C or SPI protocols; this also allows the user to implement sharing
/// techniques to share the underlying bus.
pub struct Lsm6dsv320x<B: BusOperation, T: DelayNs> {
    pub bus: B,
    pub tim: T,
}

#[derive(Debug, PartialEq)]
pub enum Error<B> {
    Bus(B),          // Error at the bus level
    UnexpectedValue, // Unexpected value read from a register
    InvalidConfiguration,
    FailedToReadMemBank,
    FailedToSetMemBank(MemBank),
    HwNoResponse,
}

impl<P, T> Lsm6dsv320x<I2cBus<P>, T>
where
    P: I2c,
    T: DelayNs,
{
    /// Constructor method for using the I2C bus.
    pub fn new_i2c(i2c: P, address: I2CAddress, tim: T) -> Self {
        // Initialize the I2C bus with the COMPONENT address
        let bus = st_mems_bus::i2c::I2cBus::new(i2c, address as SevenBitAddress);
        Self { bus, tim }
    }
}
impl<B, T> Lsm6dsv320x<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    /// Constructor method using a generic Bus that implements BusOperation
    pub fn from_bus(bus: B, tim: T) -> Self {
        Self { bus, tim }
    }
}

impl<P, T> Lsm6dsv320x<SpiBus<P>, T>
where
    P: SpiDevice,
    T: DelayNs,
{
    /// Constructor method for using the SPI bus.
    pub fn new_spi(spi: P, tim: T) -> Self {
        // Initialize the SPI bus
        let bus = st_mems_bus::spi::SpiBus::new(spi);
        Self { bus, tim }
    }
}

impl<B: BusOperation, T: DelayNs> MemBankFunctions<MemBank> for Lsm6dsv320x<B, T> {
    type Error = Error<B::Error>;
    /// Change memory bank.
    ///
    /// It change the address space indexed.
    /// <div class="warning">Ensure to return to MainPage after each change!</div>
    fn mem_bank_set(&mut self, val: MemBank) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access =
            FuncCfgAccess::read(self).map_err(|_| Error::FailedToReadMemBank)?;
        func_cfg_access.set_shub_reg_access(((val as u8) & 0x02) >> 1);
        func_cfg_access.set_emb_func_reg_access((val as u8) & 0x01);
        func_cfg_access
            .write(self)
            .map_err(|_| Error::FailedToSetMemBank(val))
    }

    /// Get the current memory bank set.
    ///
    /// If different from MainMemBank, a switch is required to guarantee the correct working
    /// of the driver.
    fn mem_bank_get(&mut self) -> Result<MemBank, Error<B::Error>> {
        let func_cfg_access = FuncCfgAccess::read(self).map_err(|_| Error::FailedToReadMemBank)?;

        let value =
            (func_cfg_access.shub_reg_access() << 1) + func_cfg_access.emb_func_reg_access();
        let val = match value {
            0 => MemBank::MainMemBank,
            1 => MemBank::EmbedFuncMemBank,
            2 => MemBank::SensorHubMemBank,
            _ => MemBank::MainMemBank,
        };
        Ok(val)
    }
}

impl<B: BusOperation, T: DelayNs> EmbAdvFunctions for Lsm6dsv320x<B, T> {
    type Error = Error<B::Error>;
    /// Write buffer in a page.
    ///
    /// # Arguments
    ///
    /// * `address`: Address where the page write begins.
    /// * `buf`: Buffer to write in a page.
    /// * `len`: Length of the buffer.
    fn ln_pg_write(&mut self, address: u16, buf: &[u8], len: u8) -> Result<(), Error<B::Error>> {
        let mut msb = ((address >> 8) & 0x0F) as u8;
        let mut lsb = (address & 0xFF) as u8;

        MemBank::operate_over_embed(self, |state| {
            // Set page write
            let mut page_rw = PageRw::read(state)?;
            page_rw.set_page_read(0);
            page_rw.set_page_write(1);
            page_rw.write(state)?;

            // Select page
            let mut page_sel = PageSel::read(state)?;
            page_sel.set_page_sel(msb);
            page_sel.write(state)?;

            // Set page address
            let mut page_address = PageAddress::new();
            page_address.set_page_addr(lsb);
            page_address.write(state)?;

            for i in 0..len {
                state.write_to_register(
                    EmbReg::PageValue as u8,
                    &buf[i as usize..(i as usize + 1)],
                )?;

                lsb = lsb.wrapping_add(1);
                // Check if page wrap
                if lsb == 0x00 {
                    msb += 1;
                    page_sel = PageSel::read(state)?;
                    page_sel.set_page_sel(msb);
                    page_sel.write(state)?;
                }
            }

            // Reset page selection
            page_sel = PageSel::read(state)?;
            page_sel.set_page_sel(0);
            page_sel.write(state)?;

            // Unset page write
            page_rw = PageRw::read(state)?;
            page_rw.set_page_read(0);
            page_rw.set_page_write(0);
            page_rw.write(state)
        })
    }

    /// Read buffer in a page.
    ///
    /// # Arguments
    ///
    /// * `address`: The address to read from.
    /// * `buf`: Write buffer in a page.
    /// * `len`: Length of the buffer.
    fn ln_pg_read(&mut self, address: u16, buf: &mut [u8], len: u8) -> Result<(), Error<B::Error>> {
        let mut msb = ((address >> 8) & 0x0F) as u8;
        let mut lsb = (address & 0xFF) as u8;

        MemBank::operate_over_embed(self, |state| {
            // Set page read
            let mut page_rw = PageRw::read(state)?;
            page_rw.set_page_read(1);
            page_rw.set_page_write(0);
            page_rw.write(state)?;

            // Select page
            let mut page_sel = PageSel::read(state)?;
            page_sel.set_page_sel(msb);
            page_sel.write(state)?;

            // Set page address
            let page_address = PageAddress::new().with_page_addr(lsb);
            page_address.write(state)?;

            for i in 0..len {
                state.read_from_register(
                    EmbReg::PageValue as u8,
                    &mut buf[i as usize..(i as usize + 1)],
                )?;

                lsb = lsb.wrapping_add(1);
                // Check if page wrap
                if lsb == 0x00 {
                    msb += 1;
                    page_sel = PageSel::read(state)?;
                    page_sel.set_page_sel(msb);
                    page_sel.write(state)?;
                }
            }

            // Reset page selection
            page_sel = PageSel::read(state)?;
            page_sel.set_page_sel(0);
            page_sel.write(state)?;

            // Unset page read
            page_rw = PageRw::read(state)?;
            page_rw.set_page_read(0);
            page_rw.set_page_write(0);
            page_rw.write(state)
        })
    }
}

impl<B: BusOperation, T: DelayNs> Lsm6dsv320x<B, T> {
    #[inline]
    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<B::Error>> {
        self.bus
            .write_byte_read_bytes(&[reg], buf)
            .map_err(Error::Bus)?;

        Ok(())
    }

    #[inline]
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), Error<B::Error>> {
        let size = buf.len();
        let mut write_size: usize;
        let mut tmp: [u8; CHUNK_SIZE + 1] = [0; CHUNK_SIZE + 1];
        for i in (0..size).step_by(CHUNK_SIZE - 1) {
            write_size = if size - i >= CHUNK_SIZE - 1 {
                CHUNK_SIZE - 1
            } else {
                size - i
            };
            tmp[0] = reg + (i / (CHUNK_SIZE - 1)) as u8;
            tmp[1..(write_size + 1)].copy_from_slice(&buf[i..(write_size + i)]);
            self.bus
                .write_bytes(&tmp[..1 + write_size])
                .map_err(Error::Bus)?;
        }
        Ok(())
    }

    /// Enables accelerometer user offset correction block; it is valid for the low-pass path.
    ///
    /// # Arguments
    ///
    /// * `val`: 1/0 Enables/Disables accelerometer user offset correction block.
    pub fn xl_offset_on_out_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl9 = Ctrl9::read(self)?;
        ctrl9.set_usr_off_on_out(val);
        ctrl9.write(self)
    }

    /// Get the settings of accelerometer user offset correction block; it is valid for the low-pass path.
    pub fn xl_offset_on_out_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl9::read(self).map(|ctrl9| ctrl9.usr_off_on_out())
    }

    /// Set the accelerometer user offset correction values (in mg).
    /// Value ranges depend on the USR_OFF_W bit in CTRL9:
    /// - If USR_OFF_W = 1: range is ±15.875 mg with 0.125 mg precision.
    /// - If USR_OFF_W = 0: range is ±0.9921875 mg with 0.0078125 mg precision.
    /// The USR_OFF_W bit is automatically enabled based on the input values (precision is shared across
    /// all axes).
    pub fn xl_offset_mg_set(&mut self, val: XlOffsetMg) -> Result<(), Error<B::Error>> {
        let mut z_ofs_usr = ZOfsUsr::read(self)?;
        let mut y_ofs_usr = YOfsUsr::read(self)?;
        let mut x_ofs_usr = XOfsUsr::read(self)?;
        let mut ctrl9 = Ctrl9::read(self)?;

        if val.x_mg < (0.0078125 * 127.0)
            && val.x_mg > (0.0078125 * -127.0)
            && val.y_mg < (0.0078125 * 127.0)
            && val.y_mg > (0.0078125 * -127.0)
            && val.z_mg < (0.0078125 * 127.0)
            && val.z_mg > (0.0078125 * -127.0)
        {
            ctrl9.set_usr_off_w(0);

            let tmp = val.z_mg / 0.0078125;
            z_ofs_usr.set_z_ofs_usr(tmp as i8);

            let tmp = val.y_mg / 0.0078125;
            y_ofs_usr.set_y_ofs_usr(tmp as i8);

            let tmp = val.x_mg / 0.0078125;
            x_ofs_usr.set_x_ofs_usr(tmp as i8);
        } else if val.x_mg < (0.125 * 127.0)
            && val.x_mg > (0.125 * -127.0)
            && val.y_mg < (0.125 * 127.0)
            && val.y_mg > (0.125 * -127.0)
            && val.z_mg < (0.125 * 127.0)
            && val.z_mg > (0.125 * -127.0)
        {
            ctrl9.set_usr_off_w(1);

            let tmp = val.z_mg / 0.125;
            z_ofs_usr.set_z_ofs_usr(tmp as i8);

            let tmp = val.y_mg / 0.125;
            y_ofs_usr.set_y_ofs_usr(tmp as i8);

            let tmp = val.x_mg / 0.125;
            x_ofs_usr.set_x_ofs_usr(tmp as i8);
        } else {
            // out of limit
            ctrl9.set_usr_off_w(1);
            z_ofs_usr.set_z_ofs_usr(0xFFu8 as i8);
            y_ofs_usr.set_y_ofs_usr(0xFFu8 as i8);
            x_ofs_usr.set_x_ofs_usr(0xFFu8 as i8);
        }

        z_ofs_usr.write(self)?;
        y_ofs_usr.write(self)?;
        x_ofs_usr.write(self)?;
        ctrl9.write(self)?;

        Ok(())
    }

    /// Get the accelerometer user offset correction values (in mg).
    pub fn xl_offset_mg_get(&mut self) -> Result<XlOffsetMg, Error<B::Error>> {
        let ctrl9 = Ctrl9::read(self)?;
        let z_ofs_usr = ZOfsUsr::read(self).map(|ofs| ofs.z_ofs_usr())?;
        let y_ofs_usr = YOfsUsr::read(self).map(|ofs| ofs.y_ofs_usr())?;
        let x_ofs_usr = XOfsUsr::read(self).map(|ofs| ofs.x_ofs_usr())?;

        let scale_factor = if ctrl9.usr_off_w() == 0 {
            0.0078125
        } else {
            0.125
        };

        let val = XlOffsetMg {
            z_mg: z_ofs_usr as f32 * scale_factor,
            y_mg: y_ofs_usr as f32 * scale_factor,
            x_mg: x_ofs_usr as f32 * scale_factor,
        };

        Ok(val)
    }

    /// Set the HG Accelerometer user offset correction values (in mg).
    pub fn hg_xl_offset_mg_set(&mut self, val: XlOffsetMg) -> Result<(), Error<B::Error>> {
        let mut ctrl1_xl_hg = Ctrl1XlHg::read(self)?;

        let mut ofs_usr = XlHgXYZOfsUsr::default();

        if (val.x_mg < (0.25 * 127.0))
            && (val.x_mg > (0.25 * -127.0))
            && (val.y_mg < (0.25 * 127.0))
            && (val.y_mg > (0.25 * -127.0))
            && (val.z_mg < (0.25 * 127.0))
            && (val.z_mg > (0.25 * -127.0))
        {
            ctrl1_xl_hg.set_hg_usr_off_on_out(1);

            ofs_usr.z = (val.z_mg / 0.25) as i8;
            ofs_usr.y = (val.y_mg / 0.25) as i8;
            ofs_usr.x = (val.x_mg / 0.25) as i8;
        } else {
            // out of limit
            ctrl1_xl_hg.set_hg_usr_off_on_out(0);
            ofs_usr.x = 0xFFu8 as i8;
            ofs_usr.y = 0xFFu8 as i8;
            ofs_usr.z = 0xFFu8 as i8;
        }

        ofs_usr.write(self)?;
        ctrl1_xl_hg.write(self)?;

        Ok(())
    }

    /// Get the HG Accelerometer user offset correction values in mg.
    pub fn hg_xl_offset_mg_get(&mut self) -> Result<XlOffsetMg, Error<B::Error>> {
        let ctrl1_xl_hg = Ctrl1XlHg::read(self)?;

        let ofs_usr_arr = XlHgXYZOfsUsr::read(self)?;

        let mut val = XlOffsetMg {
            z_mg: 0.0,
            y_mg: 0.0,
            x_mg: 0.0,
        };

        if ctrl1_xl_hg.hg_usr_off_on_out() == 0 {
            Ok(val)
        } else {
            val.z_mg = ofs_usr_arr.z as f32 * 0.25;
            val.y_mg = ofs_usr_arr.y as f32 * 0.25;
            val.x_mg = ofs_usr_arr.x as f32 * 0.25;
            Ok(val)
        }
    }

    /// Perform reboot of the device
    ///
    /// Loads the trimming parameters.
    /// Requires 30 ms to complete.
    pub fn reboot(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;

        // Cannot reboot if sw_reset has not completed.
        if ctrl3.sw_reset() == 1 {
            return Err(Error::HwNoResponse);
        }

        /* Save current data rates */
        let xl_data_rate = self.xl_data_rate_get()?;
        let gy_data_rate = self.gy_data_rate_get()?;
        let (hg_xl_data_rate, reg_out_en) = self.hg_xl_data_rate_get()?;

        /* 1. Set the low-g accelerometer, high-g accelerometer, and gyroscope in power-down mode */
        self.xl_data_rate_set(DataRate::Off)?;
        self.gy_data_rate_set(DataRate::Off)?;
        self.hg_xl_data_rate_set(HgXlDataRate::Off, 0)?;

        /* 2. Set the BOOT bit of the CTRL3 register to 1. */
        ctrl3.set_boot(1);
        ctrl3.write(self)?;

        /* 3. Wait 30 ms. */
        self.tim.delay_ms(30);

        /* Restore data rates */
        self.xl_data_rate_set(xl_data_rate)?;
        self.gy_data_rate_set(gy_data_rate)?;
        self.hg_xl_data_rate_set(hg_xl_data_rate, reg_out_en)
    }

    /// Perform s/w reset of the device.
    ///
    /// The software reset procedure takes approximately 150 µs;
    /// this function handles the wait internally (one reading after 150 us
    /// for a maximum of 3 attempts).
    ///
    /// reset to default value the control registers:
    /// - FUNC_CFG_ACCESS (01h)
    /// - ODR_TRIG_CFG (06h) through ALL_INT_SRC (1Dh)
    /// - TIMESTAMP0 (40h) through TIMESTAMP3 (43h)
    /// - WAKE_UP_SRC (45h) through D6D_SRC (47h)
    /// - HG_WAKE_UP_SRC (4Ch) through CTRL1_XL_HG (4Eh)
    /// - FUNCTIONS_ENABLE (50h) through UI_HANDSHAKE_CTRL (64h)
    /// - CTRL_EIS (6Bh) through Z_OFS_USR (75h)
    /// - FIFO_DATA_OUT_TAG (78h)
    pub fn sw_reset(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::default();
        let mut retry: u8 = 0;

        /* 1. Set the low-g accelerometer, high-g accelerometer, and gyroscope in power-down mode */
        self.xl_data_rate_set(DataRate::Off)?;
        self.gy_data_rate_set(DataRate::Off)?;
        self.hg_xl_data_rate_set(HgXlDataRate::Off, 0)?;

        /* 2. Set the SW_RESET bit of the CTRL3 register to 1. */
        ctrl3.set_sw_reset(1);
        ctrl3.write(self)?;

        /* 3. Poll the SW_RESET bit of the CTRL3 register until it returns to 0. */
        loop {
            ctrl3 = Ctrl3::read(self)?;

            if ctrl3.sw_reset() == 0 {
                return Ok(());
            }

            retry += 1;
            if retry > 3 {
                break;
            }

            self.tim.delay_us(150);
        }

        Err(Error::HwNoResponse)
    }

    /// Perform power-on-reset of the device
    ///
    /// Performs a full reset of the device, including boot, software reset,
    /// and resettng embedded functions and internal filters.
    pub fn sw_por(&mut self) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::default();

        /* 1. Set the SW_POR bit of the FUNC_CFG_ACCESS register to 1. */
        func_cfg_access.set_sw_por(1);
        func_cfg_access.write(self)?;

        /* 2. Wait 30 ms. */
        self.tim.delay_ms(30);

        Ok(())
    }

    /// Reset of the device.
    ///
    /// Reset type choosen using `Reset` enum.
    #[deprecated(since = "2.0.0", note = "please use sw_reset")]
    pub fn reset_set(&mut self, val: Reset) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::read(self)?;
        let mut ctrl3 = Ctrl3::read(self)?;

        ctrl3.set_boot(if val == Reset::RestoreCalParam { 1 } else { 0 });
        ctrl3.set_sw_reset(if val == Reset::RestoreCtrlRegs { 1 } else { 0 });
        func_cfg_access.set_sw_por(if val == Reset::GlobalRst { 1 } else { 0 });

        ctrl3.write(self)?;
        func_cfg_access.write(self)
    }

    #[deprecated(since = "2.0.0", note = "please use sw_reset")]
    pub fn reset_get(&mut self) -> Result<Reset, Error<B::Error>> {
        let ctrl3 = Ctrl3::read(self)?;
        let func_cfg_access = FuncCfgAccess::read(self)?;

        let reset_value = (ctrl3.sw_reset() << 2) + (ctrl3.boot() << 1) + func_cfg_access.sw_por();

        let val = Reset::try_from(reset_value).unwrap_or(Reset::GlobalRst);
        Ok(val)
    }

    /// Get the device ID.
    ///
    /// Return the value contained in the WhoAmI Register.
    pub fn device_id_get(&mut self) -> Result<u8, Error<B::Error>> {
        WhoAmI::read(self).map(|whoami| whoami.id())
    }

    /// Set the accelerometer output data rate (ODR).
    ///
    /// When not set to Off it starts values reading.
    pub fn xl_data_rate_set(&mut self, val: DataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_odr_xl((val as u8) & 0x0F);
        ctrl1.write(self)?;

        let sel = (val as u8 >> 4) & 0xF;
        if sel != 0 {
            let mut haodr = HaodrCfg::read(self)?;
            haodr.set_haodr_sel(sel);
            haodr.write(self)?;
        }

        Ok(())
    }

    /// Get the accelerometer output data rate (ODR).
    ///
    /// The DataRate enum contains info for High-accuracy settings
    pub fn xl_data_rate_get(&mut self) -> Result<DataRate, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;
        let haodr = HaodrCfg::read(self)?;

        let sel = haodr.haodr_sel();

        let val = ctrl1.odr_xl() + (sel << 4);
        let val = DataRate::try_from(val).unwrap_or_default();

        Ok(val)
    }

    /// Set the HG Accelerometer output data rate (ODR). If reg_out_en == 1 it enables the reading
    /// of HG accelerometer from registers: UiOutxLAOisHg to UiOutxHAOisHg.
    /// (using hg_acceleration_raw_get function)
    pub fn hg_xl_data_rate_set(
        &mut self,
        val: HgXlDataRate,
        reg_out_en: u8,
    ) -> Result<(), Error<B::Error>> {
        let mut ctrl1_xl_hg = Ctrl1XlHg::read(self)?;
        ctrl1_xl_hg.set_odr_xl_hg((val as u8) & 0x07);
        ctrl1_xl_hg.set_xl_hg_regout_en(reg_out_en & 0x01);
        ctrl1_xl_hg.write(self)
    }

    /// Get the Hg Accelerometer output data rate (ODR) selection.
    pub fn hg_xl_data_rate_get(&mut self) -> Result<(HgXlDataRate, u8), Error<B::Error>> {
        let ctrl1_xl_hg = Ctrl1XlHg::read(self)?;
        let reg_out_en = ctrl1_xl_hg.xl_hg_regout_en();
        let odr_val = HgXlDataRate::try_from(ctrl1_xl_hg.odr_xl_hg()).unwrap_or_default();
        Ok((odr_val, reg_out_en))
    }

    /// Accelerometer operating mode selection.
    pub fn xl_mode_set(&mut self, val: XlMode) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_op_mode_xl((val as u8) & 0x07);
        ctrl1.write(self)
    }

    /// Get the actual accelerometer operating mode.
    pub fn xl_mode_get(&mut self) -> Result<XlMode, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;

        let mode = XlMode::try_from(ctrl1.op_mode_xl()).unwrap_or_default();
        Ok(mode)
    }

    /// Set the gyroscope output data rate (ODR).
    pub fn gy_data_rate_set(&mut self, val: DataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_odr_g((val as u8) & 0x0F);
        ctrl2.write(self)?;

        let sel = ((val as u8) >> 4) & 0x0F;
        if sel != 0 {
            let mut haodr = HaodrCfg::read(self)?;
            haodr.set_haodr_sel(sel);
            haodr.write(self)?;
        }
        Ok(())
    }

    /// Get the actual gyroscope output data rate (ODR).
    pub fn gy_data_rate_get(&mut self) -> Result<DataRate, Error<B::Error>> {
        let ctrl2 = Ctrl2::read(self)?;
        let haodr = HaodrCfg::read(self)?;

        let sel = haodr.haodr_sel();

        let val = ctrl2.odr_g() + (sel << 4);
        let val = DataRate::try_from(val).unwrap_or_default();

        Ok(val)
    }

    /// Set the gyroscope operating mode.
    pub fn gy_mode_set(&mut self, val: GyMode) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_op_mode_g((val as u8) & 0x07);
        ctrl2.write(self)
    }

    /// Get actual gyroscope operating mode.
    pub fn gy_mode_get(&mut self) -> Result<GyMode, Error<B::Error>> {
        let ctrl2 = Ctrl2::read(self)?;

        let mode = GyMode::try_from(ctrl2.op_mode_g()).unwrap_or_default();
        Ok(mode)
    }

    /// Enable/Disable the auto increment setting.
    ///
    /// If val == 1 it Enable automatic increment of the register address during
    /// multiple-byte access with a serial interface; enabled by default.
    pub fn auto_increment_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_if_inc(val);
        ctrl3.write(self)
    }

    /// Get the actual auto increment setting
    ///
    /// Register address automatically incremented during a multiple byte access
    /// with a serial interface (enable by default).
    pub fn auto_increment_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl3::read(self).map(|ctlr3| ctlr3.if_inc())
    }

    /// Enable/Disable Block Data Update (BDU)
    ///
    /// If active the output registers are not updated until LSB and MSB have been read.
    pub fn block_data_update_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_bdu(val);
        ctrl3.write(self)
    }

    /// Get actual settings of Block Data Update (BDU)
    pub fn block_data_update_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl3::read(self).map(|ctrl3| ctrl3.bdu())
    }

    /// Configure ODR trigger.
    ///
    /// Number of data generated in reference period when ODR-triggered mode is set.
    /// Allowed values: 0 (default) or 4 to 255.
    pub fn odr_trig_cfg_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        if (1..=3).contains(&val) {
            return Err(Error::UnexpectedValue);
        }

        let mut odr_trig = OdrTrigCfg::read(self)?;
        odr_trig.set_odr_trig_nodr(val);
        odr_trig.write(self)
    }

    /// Get the actual ODR trigger.
    ///
    /// Number of data generated in reference period when ODR-triggered mode is set.
    pub fn odr_trig_cfg_get(&mut self) -> Result<u8, Error<B::Error>> {
        OdrTrigCfg::read(self).map(|odr_trig_cfg| odr_trig_cfg.odr_trig_nodr())
    }

    /// Switch between pulsed and latched mode.
    ///
    /// Pulsed data-ready mode with ~75 us.
    pub fn data_ready_mode_set(&mut self, val: DataReadyMode) -> Result<(), Error<B::Error>> {
        let mut ctrl4 = Ctrl4::read(self)?;
        ctrl4.set_drdy_pulsed((val as u8) & 0x1);
        ctrl4.write(self)
    }

    /// Get actual pulsed data-ready mode
    pub fn data_ready_mode_get(&mut self) -> Result<DataReadyMode, Error<B::Error>> {
        let ctrl4 = Ctrl4::read(self)?;

        let mode = DataReadyMode::try_from(ctrl4.drdy_pulsed()).unwrap_or_default();
        Ok(mode)
    }

    /// Enables/disable interrupt and switch between latched/pulsed interrupts
    pub fn interrupt_enable_set(&mut self, val: InterruptMode) -> Result<(), Error<B::Error>> {
        let mut func = FunctionsEnable::read(self)?;
        func.set_interrupts_enable(val.enable);
        func.write(self)?;

        let mut tap_cfg = TapCfg0::read(self)?;
        tap_cfg.set_lir(val.lir);
        tap_cfg.write(self)
    }

    /// Get the interrupt Mode
    ///
    /// Enable/disabled and latched/pulsed information.
    pub fn interrupt_enable_get(&mut self) -> Result<InterruptMode, Error<B::Error>> {
        let func = FunctionsEnable::read(self)?;
        let cfg = TapCfg0::read(self)?;

        let val = InterruptMode {
            enable: func.interrupts_enable(),
            lir: cfg.lir(),
        };

        Ok(val)
    }

    /// Set the Gyroscope full-scale.
    pub fn gy_full_scale_set(&mut self, val: GyFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl6 = Ctrl6::read(self)?;
        let mut ctrl2 = Ctrl2::read(self)?;
        let prev_ctrl2 = ctrl2.clone();

        // For the correct operation of the device, the user must set a
        // configuration from 001 to 101 when the gyroscope is in power-down mode.
        if ctrl2.odr_g() != (DataRate::Off as u8) {
            ctrl2.set_odr_g(DataRate::Off as u8);
            ctrl2.write(self)?;
        }

        ctrl6.set_fs_g((val as u8) & 0xf);
        ctrl6.write(self)?;

        // restore previous odr set
        if prev_ctrl2.odr_g() != (DataRate::Off as u8) {
            prev_ctrl2.write(self)?
        }

        Ok(())
    }

    /// Get the actual Gyroscope full-scale.
    pub fn gy_full_scale_get(&mut self) -> Result<GyFullScale, Error<B::Error>> {
        let ctrl6 = Ctrl6::read(self)?;

        let val = GyFullScale::try_from(ctrl6.fs_g()).unwrap_or_default();
        Ok(val)
    }

    /// Set the Accelerometer full-scale.
    pub fn xl_full_scale_set(&mut self, val: XlFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl8 = Ctrl8::read(self)?;
        ctrl8.set_fs_xl((val as u8) & 0x3);
        ctrl8.write(self)
    }

    /// Get the Accelerometer full-scale.
    pub fn xl_full_scale_get(&mut self) -> Result<XlFullScale, Error<B::Error>> {
        let ctrl8 = Ctrl8::read(self)?;
        let fs_xl = XlFullScale::try_from(ctrl8.fs_xl()).unwrap_or_default();

        Ok(fs_xl)
    }

    /// Set the High-G Accelerometer full-scale.
    pub fn hg_xl_full_scale_set(&mut self, val: HgXlFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1XlHg::read(self)?;
        ctrl1.set_fs_xl_hg(val as u8 & 0x7);
        ctrl1.write(self)
    }

    /// Get the current High-G Accelerometer full-scale.
    pub fn hg_xl_full_scale_get(&mut self) -> Result<HgXlFullScale, Error<B::Error>> {
        let ctrl1 = Ctrl1XlHg::read(self)?;
        let val = HgXlFullScale::try_from(ctrl1.fs_xl_hg()).unwrap_or_default();

        Ok(val)
    }

    /// Set the Accelerometer self-test.
    pub fn xl_self_test_set(&mut self, val: SelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl10 = Ctrl10::read(self)?;
        ctrl10.set_st_xl(val as u8 & 0x3);
        ctrl10.write(self)
    }

    /// Get the actual Accelerometer self-test.
    pub fn xl_self_test_get(&mut self) -> Result<SelfTest, Error<B::Error>> {
        let ctrl10 = Ctrl10::read(self)?;
        let val = SelfTest::try_from(ctrl10.st_xl()).unwrap_or_default();

        Ok(val)
    }

    /// Set the Gyroscope self-test.
    pub fn gy_self_test_set(&mut self, val: SelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl10 = Ctrl10::read(self)?;
        ctrl10.set_st_g((val as u8) & 0x3);
        ctrl10.write(self)
    }

    /// Get the actual Gyroscope self-test selection.
    pub fn gy_self_test_get(&mut self) -> Result<SelfTest, Error<B::Error>> {
        let reg = Ctrl10::read(self)?;
        let val = SelfTest::try_from(reg.st_g()).unwrap_or_default();

        Ok(val)
    }

    /// Set the High-G XL self-test.
    pub fn hg_xl_self_test_set(&mut self, val: SelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl2_xl_hg = Ctrl2XlHg::read(self)?;
        ctrl2_xl_hg.set_xl_hg_st((val as u8) & 0x3);
        ctrl2_xl_hg.write(self)
    }

    /// Get the actual HG XL self-test.
    pub fn hg_xl_self_test_get(&mut self) -> Result<SelfTest, Error<B::Error>> {
        let ctrl2_xl_hg = Ctrl2XlHg::read(self)?;

        let val = SelfTest::try_from(ctrl2_xl_hg.xl_hg_st()).unwrap_or_default();

        Ok(val)
    }

    /// Set the IF2 Accelerometer self-test.
    pub fn ois_xl_self_test_set(&mut self, val: SelfTest) -> Result<(), Error<B::Error>> {
        let mut if2_int_ois = If2IntOis::read(self)?;
        if2_int_ois.set_st_xl_ois((val as u8) & 0x3);
        if2_int_ois.write(self)
    }

    /// Get the actual IF2 Accelerometer self-test.
    pub fn ois_xl_self_test_get(&mut self) -> Result<SelfTest, Error<B::Error>> {
        let if2_int_ois = If2IntOis::read(self)?;

        let val = SelfTest::try_from(if2_int_ois.st_xl_ois()).unwrap_or_default();

        Ok(val)
    }

    /// Set the IF2 Gyroscope self-test.
    pub fn ois_gy_self_test_set(&mut self, val: OisGySelfTest) -> Result<(), Error<B::Error>> {
        let mut if2_int_ois = If2IntOis::read(self)?;
        if2_int_ois.set_st_g_ois((val as u8) & 0x3);
        if2_int_ois.set_st_ois_clampdis(((val as u8) & 0x04) >> 2);
        if2_int_ois.write(self)
    }

    /// Get the actual IF2 Accelerometer self-test selection.
    pub fn ois_gy_self_test_get(&mut self) -> Result<OisGySelfTest, Error<B::Error>> {
        let if2_int_ois = If2IntOis::read(self)?;

        let val = match if2_int_ois.st_g_ois() {
            0 => OisGySelfTest::Disable,
            1 => {
                if if2_int_ois.st_ois_clampdis() == 1 {
                    OisGySelfTest::ClampPos
                } else {
                    OisGySelfTest::Positive
                }
            }
            2 => {
                if if2_int_ois.st_ois_clampdis() == 1 {
                    OisGySelfTest::ClampNeg
                } else {
                    OisGySelfTest::Negative
                }
            }
            _ => OisGySelfTest::Disable,
        };

        Ok(val)
    }

    /// Set the High-g wakeup event configuration.
    ///
    /// Setup threshold and duration.
    pub fn hg_wake_up_cfg_set(&mut self, val: HgWakeUpCfg) -> Result<(), Error<B::Error>> {
        let mut hg_func = HgFunctionsEnable::read(self)?;
        hg_func.set_hg_shock_dur(val.hg_shock_dur);
        hg_func.write(self)?;

        let mut hg_wake_up_ths = HgWakeUpThs::read(self)?;
        hg_wake_up_ths.set_hg_wk_ths(val.hg_wakeup_ths);
        hg_wake_up_ths.write(self)
    }

    /// Get the actual High-g wakeup event configuration.
    pub fn hg_wake_up_cfg_get(&mut self) -> Result<HgWakeUpCfg, Error<B::Error>> {
        let hg_func = HgFunctionsEnable::read(self)?;
        let hg_wake_up_ths = HgWakeUpThs::read(self)?;

        Ok(HgWakeUpCfg {
            hg_shock_dur: hg_func.hg_shock_dur(),
            hg_wakeup_ths: hg_wake_up_ths.hg_wk_ths(),
        })
    }

    /// Set the High-g wake-up interrupt.
    pub fn hg_wu_interrupt_cfg_set(
        &mut self,
        val: HgWuInterruptCfg,
    ) -> Result<(), Error<B::Error>> {
        let mut hg_func = HgFunctionsEnable::read(self)?;
        hg_func.set_hg_interrupts_enable(val.hg_interrupts_enable);
        hg_func.set_hg_wu_change_int_sel(val.hg_wakeup_int_sel);
        hg_func.write(self)
    }

    /// Get the actual High-g wake-up interrupt.
    pub fn hg_wu_interrupt_cfg_get(&mut self) -> Result<HgWuInterruptCfg, Error<B::Error>> {
        let hg_func = HgFunctionsEnable::read(self)?;

        let val = HgWuInterruptCfg {
            hg_interrupts_enable: hg_func.hg_interrupts_enable(),
            hg_wakeup_int_sel: hg_func.hg_wu_change_int_sel(),
        };

        Ok(val)
    }

    /// Enable/disable user offset data correction driving to hg embedded functions.
    pub fn hg_emb_usr_off_correction_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = EmbFuncCfg::read(self)?;
        reg.set_hg_usr_off_on_emb_func(val & 0x1);
        reg.write(self)
    }

    /// Get the actual user offset data correction driving to hg embedded functions.
    pub fn hg_emb_usr_off_correction_get(&mut self) -> Result<u8, Error<B::Error>> {
        EmbFuncCfg::read(self).map(|cfg| cfg.hg_usr_off_on_emb_func())
    }

    /// Enable/disable user offset data correction driving to hg wake-up.
    pub fn hg_wu_usr_off_correction_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl2_xl_hg = Ctrl2XlHg::read(self)?;
        ctrl2_xl_hg.set_hg_usr_off_on_wu(val & 0x1);
        ctrl2_xl_hg.write(self)
    }

    /// Get the actual user offset data correction driving to hg wake-up
    pub fn hg_wu_usr_off_correction_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl2XlHg::read(self).map(|reg| reg.hg_usr_off_on_wu())
    }

    /// Get the High-g all event status
    ///
    /// Event includes: Wakeup (on 3 axis), wakeup change, shock, shock change.
    pub fn hg_event_get(&mut self) -> Result<HgEvent, Error<B::Error>> {
        let int_src = AllIntSrc::read(self)?;
        let wup_src = HgWakeUpSrc::read(self)?;

        let hg_event = HgEvent {
            hg_event: int_src.hg_ia(),
            hg_wakeup_z: wup_src.hg_z_wu(),
            hg_wakeup_y: wup_src.hg_y_wu(),
            hg_wakeup_x: wup_src.hg_x_wu(),
            hg_wakeup: wup_src.hg_wu_ia(),
            hg_wakeup_chg: wup_src.hg_wu_change_ia(),
            hg_shock: wup_src.hg_shock_state(),
            hg_shock_change: wup_src.hg_wu_change_ia(),
        };
        Ok(hg_event)
    }

    /// Set the signals that need to be routed on int1 pad.
    ///
    /// See `PinInt1Route` for a complete list of available events.
    pub fn pin_int1_route_set(&mut self, val: &PinInt1Route) -> Result<(), Error<B::Error>> {
        let mut int1_ctrl = Int1Ctrl::read(self)?;
        int1_ctrl.set_int1_drdy_xl(val.drdy_xl);
        int1_ctrl.set_int1_drdy_g(val.drdy_g);
        int1_ctrl.set_int1_fifo_th(val.fifo_th);
        int1_ctrl.set_int1_fifo_ovr(val.fifo_ovr);
        int1_ctrl.set_int1_fifo_full(val.fifo_full);
        int1_ctrl.set_int1_cnt_bdr(val.cnt_bdr);
        int1_ctrl.write(self)?;

        let mut md1_cfg = Md1Cfg::read(self)?;
        md1_cfg.set_int1_shub(val.shub);
        md1_cfg.set_int1_6d(val.sixd);
        md1_cfg.set_int1_single_tap(val.single_tap);
        md1_cfg.set_int1_double_tap(val.double_tap);
        md1_cfg.set_int1_wu(val.wakeup);
        md1_cfg.set_int1_ff(val.freefall);
        md1_cfg.set_int1_sleep_change(val.sleep_change);
        md1_cfg.write(self)?;

        Ok(())
    }

    /// Report the signals that are routed on int1 pad.
    /// Values returned are:
    ///     - drdy_xl
    ///     - drdy_g
    ///     - fifo_th
    ///     - cnt_bdr
    ///     - emb_func_endop
    ///     - timestamp
    ///     - single_tap
    ///     - double_tap
    ///     - wakeup
    ///     - freefall
    ///     - sleep_change
    ///
    /// The remaining are leaved default
    pub fn pin_int1_route_get(&mut self) -> Result<PinInt1Route, Error<B::Error>> {
        let int1_ctrl = Int1Ctrl::read(self)?;
        let md1_cfg = Md1Cfg::read(self)?;

        let val = PinInt1Route {
            drdy_xl: int1_ctrl.int1_drdy_xl(),
            drdy_g: int1_ctrl.int1_drdy_g(),
            fifo_th: int1_ctrl.int1_fifo_th(),
            fifo_ovr: int1_ctrl.int1_fifo_ovr(),
            fifo_full: int1_ctrl.int1_fifo_full(),
            cnt_bdr: int1_ctrl.int1_cnt_bdr(),
            shub: md1_cfg.int1_shub(),
            sixd: md1_cfg.int1_6d(),
            single_tap: md1_cfg.int1_single_tap(),
            double_tap: md1_cfg.int1_double_tap(),
            wakeup: md1_cfg.int1_wu(),
            freefall: md1_cfg.int1_ff(),
            sleep_change: md1_cfg.int1_sleep_change(),
            ..Default::default()
        };

        Ok(val)
    }

    /// Set the signals that need to be routed on int2 pad.
    pub fn pin_int2_route_set(&mut self, val: &PinInt2Route) -> Result<(), Error<B::Error>> {
        let mut int2_ctrl = Int2Ctrl::read(self)?;

        int2_ctrl.set_int2_drdy_xl(val.drdy_xl);
        int2_ctrl.set_int2_drdy_g(val.drdy_g);
        int2_ctrl.set_int2_fifo_th(val.fifo_th);
        int2_ctrl.set_int2_fifo_ovr(val.fifo_ovr);
        int2_ctrl.set_int2_fifo_full(val.fifo_full);
        int2_ctrl.set_int2_cnt_bdr(val.cnt_bdr);
        int2_ctrl.set_int2_drdy_g_eis(val.drdy_g_eis);
        int2_ctrl.set_int2_emb_func_endop(val.emb_func_endop);

        int2_ctrl.write(self)?;

        let mut ctrl4 = Ctrl4::read(self)?;
        ctrl4.set_int2_drdy_temp(val.drdy_temp);
        ctrl4.write(self)?;

        let mut md2_cfg = Md2Cfg::read(self)?;
        md2_cfg.set_int2_timestamp(val.timestamp);
        md2_cfg.set_int2_6d(val.sixd);
        md2_cfg.set_int2_single_tap(val.single_tap);
        md2_cfg.set_int2_double_tap(val.double_tap);
        md2_cfg.set_int2_wu(val.wakeup);
        md2_cfg.set_int2_ff(val.freefall);
        md2_cfg.set_int2_sleep_change(val.sleep_change);
        md2_cfg.write(self)?;

        Ok(())
    }

    /// Report the signals that are routed on int2 pad.
    pub fn pin_int2_route_get(&mut self) -> Result<PinInt2Route, Error<B::Error>> {
        let int2_ctrl = Int2Ctrl::read(self)?;
        let ctrl4 = Ctrl4::read(self)?;
        let md2_cfg = Md2Cfg::read(self)?;

        let route = PinInt2Route {
            drdy_xl: int2_ctrl.int2_drdy_xl(),
            drdy_g: int2_ctrl.int2_drdy_g(),
            fifo_th: int2_ctrl.int2_fifo_th(),
            fifo_ovr: int2_ctrl.int2_fifo_ovr(),
            fifo_full: int2_ctrl.int2_fifo_full(),
            cnt_bdr: int2_ctrl.int2_cnt_bdr(),
            drdy_g_eis: int2_ctrl.int2_drdy_g_eis(),
            emb_func_endop: int2_ctrl.int2_emb_func_endop(),
            timestamp: md2_cfg.int2_timestamp(),
            sixd: md2_cfg.int2_6d(),
            single_tap: md2_cfg.int2_single_tap(),
            double_tap: md2_cfg.int2_double_tap(),
            wakeup: md2_cfg.int2_wu(),
            freefall: md2_cfg.int2_ff(),
            sleep_change: md2_cfg.int2_sleep_change(),
            drdy_temp: ctrl4.int2_drdy_temp(),
            ..Default::default()
        };

        Ok(route)
    }

    /// Select the signals that need to be routed on int1 pad. (hg part)
    pub fn pin_int1_route_hg_set(&mut self, val: &PinIntRouteHg) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_int1_drdy_xl_hg(val.drdy_hg_xl);
        ctrl7.write(self)?;

        let mut hg_func = HgFunctionsEnable::read(self)?;
        hg_func.set_int1_hg_wu(val.hg_wakeup);
        hg_func.write(self)?;

        let mut reg_shock = InactivityThs::read(self)?;
        reg_shock.set_int1_hg_shock_change(val.hg_shock_change);
        reg_shock.write(self)?;

        Ok(())
    }

    /// Report the signals that are routed on int1 pad for hg part.
    /// Fields get are:
    ///     - drdy_hg_xl
    ///     - hg_wakeup
    ///     - hg_shock_change
    pub fn pin_int1_route_hg_get(&mut self) -> Result<PinIntRouteHg, Error<B::Error>> {
        let ctrl7 = Ctrl7::read(self)?;
        let hg_func = HgFunctionsEnable::read(self)?;
        let reg_shock = InactivityThs::read(self)?;

        let val = PinIntRouteHg {
            drdy_hg_xl: ctrl7.int1_drdy_xl_hg(),
            hg_shock_change: reg_shock.int1_hg_shock_change(),
            hg_wakeup: hg_func.int1_hg_wu(),
        };

        Ok(val)
    }

    /// Select the signals that need to be routed on int2 pad. (hg part)
    pub fn pin_int2_route_hg_set(&mut self, val: &PinIntRouteHg) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_int2_drdy_xl_hg(val.drdy_hg_xl);
        ctrl7.write(self)?;

        let mut hg_func = HgFunctionsEnable::read(self)?;
        hg_func.set_int2_hg_wu(val.hg_wakeup);
        hg_func.write(self)?;

        let mut reg_shock = InactivityThs::read(self)?;
        reg_shock.set_int2_hg_shock_change(val.hg_shock_change);
        reg_shock.write(self)
    }

    /// Report the signals that are routed on int2 pad for hg part.
    ///
    /// Fields get are:
    ///     - drdy_hg_xl
    ///     - hg_wakeup
    ///     - hg_shock_change
    pub fn pin_int2_route_hg_get(&mut self) -> Result<PinIntRouteHg, Error<B::Error>> {
        let ctrl7 = Ctrl7::read(self)?;
        let hg_func = HgFunctionsEnable::read(self)?;
        let reg_shock = InactivityThs::read(self)?;

        let val = PinIntRouteHg {
            drdy_hg_xl: ctrl7.int2_drdy_xl_hg(),
            hg_wakeup: hg_func.int2_hg_wu(),
            hg_shock_change: reg_shock.int2_hg_shock_change(),
            ..Default::default()
        };

        Ok(val)
    }

    /// Select the signals that need to be routed on int1 pad. (embedded events)
    pub fn pin_int1_route_embedded_set(
        &mut self,
        val: &PinIntRouteEmb,
    ) -> Result<(), Error<B::Error>> {
        let mut md1_cfg = Md1Cfg::read(self)?;
        md1_cfg.set_int1_emb_func(1);
        md1_cfg.write(self)?;

        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_int1 = EmbFuncInt1::read(lock)?;
            emb_func_int1.set_int1_step_detector(val.step_detector);
            emb_func_int1.set_int1_tilt(val.tilt);
            emb_func_int1.set_int1_sig_mot(val.sig_mot);
            emb_func_int1.write(lock)?;

            let mut fsm_int1 = FsmInt1::read(lock)?;
            fsm_int1.set_int1_fsm1(val.fsm1);
            fsm_int1.set_int1_fsm2(val.fsm2);
            fsm_int1.set_int1_fsm3(val.fsm3);
            fsm_int1.set_int1_fsm4(val.fsm4);
            fsm_int1.set_int1_fsm5(val.fsm5);
            fsm_int1.set_int1_fsm6(val.fsm6);
            fsm_int1.set_int1_fsm7(val.fsm7);
            fsm_int1.set_int1_fsm8(val.fsm8);
            fsm_int1.write(lock)?;

            let mut mlc_int1 = MlcInt1::read(lock)?;
            mlc_int1.set_int1_mlc1(val.mlc1);
            mlc_int1.set_int1_mlc2(val.mlc2);
            mlc_int1.set_int1_mlc3(val.mlc3);
            mlc_int1.set_int1_mlc4(val.mlc4);
            mlc_int1.set_int1_mlc5(val.mlc5);
            mlc_int1.set_int1_mlc6(val.mlc6);
            mlc_int1.set_int1_mlc7(val.mlc7);
            mlc_int1.set_int1_mlc8(val.mlc8);
            mlc_int1.write(lock)
        })
    }

    /// Report the signals that are routed on int1 pad.
    pub fn pin_int1_route_embedded_get(&mut self) -> Result<PinIntRouteEmb, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let emb_func_int1 = EmbFuncInt1::read(lock)?;
            let fsm_int1 = FsmInt1::read(lock)?;
            let mlc_int1 = MlcInt1::read(lock)?;

            Ok(PinIntRouteEmb {
                step_detector: emb_func_int1.int1_step_detector(),
                tilt: emb_func_int1.int1_tilt(),
                sig_mot: emb_func_int1.int1_sig_mot(),
                fsm1: fsm_int1.int1_fsm1(),
                fsm2: fsm_int1.int1_fsm2(),
                fsm3: fsm_int1.int1_fsm3(),
                fsm4: fsm_int1.int1_fsm4(),
                fsm5: fsm_int1.int1_fsm5(),
                fsm6: fsm_int1.int1_fsm6(),
                fsm7: fsm_int1.int1_fsm7(),
                fsm8: fsm_int1.int1_fsm8(),
                mlc1: mlc_int1.int1_mlc1(),
                mlc2: mlc_int1.int1_mlc2(),
                mlc3: mlc_int1.int1_mlc3(),
                mlc4: mlc_int1.int1_mlc4(),
                mlc5: mlc_int1.int1_mlc5(),
                mlc6: mlc_int1.int1_mlc6(),
                mlc7: mlc_int1.int1_mlc7(),
                mlc8: mlc_int1.int1_mlc8(),
            })
        })
    }

    /// Select the signals that need to be routed on int2 pad. (embedded events)
    pub fn pin_int2_route_embedded_set(
        &mut self,
        val: &PinIntRouteEmb,
    ) -> Result<(), Error<B::Error>> {
        let mut md2_cfg = Md2Cfg::read(self)?;
        md2_cfg.set_int2_emb_func(1);
        md2_cfg.write(self)?;

        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_int2 = EmbFuncInt2::read(lock)?;
            emb_func_int2.set_int2_step_detector(val.step_detector);
            emb_func_int2.set_int2_tilt(val.tilt);
            emb_func_int2.set_int2_sig_mot(val.sig_mot);
            emb_func_int2.write(lock)?;

            let mut fsm_int2 = FsmInt2::read(lock)?;
            fsm_int2.set_int2_fsm1(val.fsm1);
            fsm_int2.set_int2_fsm2(val.fsm2);
            fsm_int2.set_int2_fsm3(val.fsm3);
            fsm_int2.set_int2_fsm4(val.fsm4);
            fsm_int2.set_int2_fsm5(val.fsm5);
            fsm_int2.set_int2_fsm6(val.fsm6);
            fsm_int2.set_int2_fsm7(val.fsm7);
            fsm_int2.set_int2_fsm8(val.fsm8);
            fsm_int2.write(lock)?;

            let mut mlc_int2 = MlcInt2::read(lock)?;
            mlc_int2.set_int2_mlc1(val.mlc1);
            mlc_int2.set_int2_mlc2(val.mlc2);
            mlc_int2.set_int2_mlc3(val.mlc3);
            mlc_int2.set_int2_mlc4(val.mlc4);
            mlc_int2.set_int2_mlc5(val.mlc5);
            mlc_int2.set_int2_mlc6(val.mlc6);
            mlc_int2.set_int2_mlc7(val.mlc7);
            mlc_int2.set_int2_mlc8(val.mlc8);
            mlc_int2.write(lock)
        })
    }

    /// Report the signals that are routed on int2 pad.
    pub fn pin_int2_route_embedded_get(&mut self) -> Result<PinIntRouteEmb, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let emb_func_int2 = EmbFuncInt2::read(lock)?;
            let fsm_int2 = FsmInt2::read(lock)?;
            let mlc_int2 = MlcInt2::read(lock)?;

            Ok(PinIntRouteEmb {
                step_detector: emb_func_int2.int2_step_detector(),
                tilt: emb_func_int2.int2_tilt(),
                sig_mot: emb_func_int2.int2_sig_mot(),
                fsm1: fsm_int2.int2_fsm1(),
                fsm2: fsm_int2.int2_fsm2(),
                fsm3: fsm_int2.int2_fsm3(),
                fsm4: fsm_int2.int2_fsm4(),
                fsm5: fsm_int2.int2_fsm5(),
                fsm6: fsm_int2.int2_fsm6(),
                fsm7: fsm_int2.int2_fsm7(),
                fsm8: fsm_int2.int2_fsm8(),
                mlc1: mlc_int2.int2_mlc1(),
                mlc2: mlc_int2.int2_mlc2(),
                mlc3: mlc_int2.int2_mlc3(),
                mlc4: mlc_int2.int2_mlc4(),
                mlc5: mlc_int2.int2_mlc5(),
                mlc6: mlc_int2.int2_mlc6(),
                mlc7: mlc_int2.int2_mlc7(),
                mlc8: mlc_int2.int2_mlc8(),
            })
        })
    }

    /// Get the status of all the interrupt sources.
    ///
    /// Values NOT set includes:
    ///     - emb_func_stand_by,
    ///     - emb_func_time_exceed
    ///     - sh_endop
    ///     - sh_target*_nack
    ///     - sh_wr_once
    pub fn all_sources_get(&mut self) -> Result<AllSources, Error<B::Error>> {
        let mut functions_enable = FunctionsEnable::read(self)?;
        functions_enable.set_dis_rst_lir_all_int(1);
        functions_enable.write(self)?;

        let mut buff: [u8; 4] = [0; 4];
        self.read_from_register(Reg::FifoStatus1 as u8, &mut buff)?;

        let fifo_status2 = FifoStatusReg::from_bits(u16::from_le_bytes([buff[0], buff[1]]));
        let all_int_src = AllIntSrc::from_bits(buff[2]);
        let status_reg = StatusReg::from_bits(buff[3]);

        let mut functions_enable = FunctionsEnable::read(self)?;
        functions_enable.set_dis_rst_lir_all_int(0);
        functions_enable.write(self)?;

        let mut buff: [u8; 8] = [0; 8];
        self.read_from_register(Reg::UiStatusRegOis as u8, &mut buff)?;

        let status_reg_ois = UiStatusRegOis::from_bits(buff[0]);
        let wake_up_src = WakeUpSrc::from_bits(buff[1]);
        let tap_src = TapSrc::from_bits(buff[2]);
        let d6d_src = D6dSrc::from_bits(buff[3]);
        let emb_func_status_mainpage = EmbFuncStatusMainpage::from_bits(buff[5]);
        let fsm_status_mainpage = FsmStatusMainpage::from_bits(buff[6]);
        let mlc_status_mainpage = MlcStatusMainpage::from_bits(buff[7]);

        let val = AllSources {
            fifo_ovr: fifo_status2.fifo_ovr_ia(),
            fifo_bdr: fifo_status2.counter_bdr_ia(),
            fifo_full: fifo_status2.fifo_full_ia(),
            fifo_th: fifo_status2.fifo_wtm_ia(),
            hg: all_int_src.hg_ia(),
            free_fall: all_int_src.ff_ia(),
            wake_up: all_int_src.wu_ia(),
            six_d: all_int_src.d6d_ia(),
            drdy_xl: status_reg.xlda(),
            drdy_gy: status_reg.gda(),
            drdy_temp: status_reg.tda(),
            drdy_xlhgda: status_reg.xlhgda(),
            drdy_eis: status_reg.gda_eis(),
            drdy_ois: status_reg.ois_drdy(),
            gy_settling: status_reg_ois.gyro_settling(),
            timestamp: status_reg.timestamp_endcount(),
            sleep_change: wake_up_src.sleep_change_ia(),
            wake_up_x: wake_up_src.x_wu(),
            wake_up_y: wake_up_src.y_wu(),
            wake_up_z: wake_up_src.z_wu(),
            sleep_state: wake_up_src.sleep_state(),
            tap_x: tap_src.x_tap(),
            tap_y: tap_src.y_tap(),
            tap_z: tap_src.z_tap(),
            tap_sign: tap_src.tap_sign(),
            double_tap: tap_src.double_tap(),
            single_tap: tap_src.single_tap(),
            six_d_zl: d6d_src.zl(),
            six_d_zh: d6d_src.zh(),
            six_d_yl: d6d_src.yl(),
            six_d_yh: d6d_src.yh(),
            six_d_xl: d6d_src.xl(),
            six_d_xh: d6d_src.xh(),
            step_detector: emb_func_status_mainpage.is_step_det(),
            tilt: emb_func_status_mainpage.is_tilt(),
            sig_mot: emb_func_status_mainpage.is_sigmot(),
            fsm_lc: emb_func_status_mainpage.is_fsm_lc(),
            fsm1: fsm_status_mainpage.is_fsm1(),
            fsm2: fsm_status_mainpage.is_fsm2(),
            fsm3: fsm_status_mainpage.is_fsm3(),
            fsm4: fsm_status_mainpage.is_fsm4(),
            fsm5: fsm_status_mainpage.is_fsm5(),
            fsm6: fsm_status_mainpage.is_fsm6(),
            fsm7: fsm_status_mainpage.is_fsm7(),
            fsm8: fsm_status_mainpage.is_fsm8(),
            mlc1: mlc_status_mainpage.is_mlc1(),
            mlc2: mlc_status_mainpage.is_mlc2(),
            mlc3: mlc_status_mainpage.is_mlc3(),
            mlc4: mlc_status_mainpage.is_mlc4(),
            mlc5: mlc_status_mainpage.is_mlc5(),
            mlc6: mlc_status_mainpage.is_mlc6(),
            mlc7: mlc_status_mainpage.is_mlc7(),
            mlc8: mlc_status_mainpage.is_mlc8(),
            ..Default::default()
        };

        Ok(val)
    }

    /// Get Flag data ready
    ///
    /// Return status about: hgxl, xl, gy, temp; data ready
    pub fn flag_data_ready_get(&mut self) -> Result<DataReady, Error<B::Error>> {
        let status = StatusReg::read(self)?;

        Ok(DataReady {
            drdy_hgxl: status.xlhgda(),
            drdy_xl: status.xlda(),
            drdy_gy: status.gda(),
            drdy_temp: status.tda(),
        })
    }

    /// Set Mask status bit reset
    pub fn int_ack_mask_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            IntAckMask::new().with_iack_mask(val).write(lock)
        })
    }

    /// Get the actual Mask status bit reset
    pub fn int_ack_mask_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            IntAckMask::read(lock).map(|reg| reg.iack_mask())
        })
    }

    /// Get the Temperature raw data
    pub fn temperature_raw_get(&mut self) -> Result<i16, Error<B::Error>> {
        OutTemp::read(self).map(|reg| reg.0)
    }

    /// Get the Angular rate raw data.
    pub fn angular_rate_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let arr = OutXYZG::read(self)?;
        Ok([arr.x, arr.y, arr.z])
    }

    /// Get the OIS Angular rate raw data.
    ///
    /// Data comes through IF2
    pub fn ois_angular_rate_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let arr = If2OutXYZGOIS::read(self)?;

        Ok([arr.x, arr.y, arr.z])
    }

    /// Get angular rate raw data for OIS gyro or the EIS gyro channel.
    pub fn ois_eis_angular_rate_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let arr = UiOutXYZGOisEis::read(self)?;

        Ok([arr.x, arr.y, arr.z])
    }

    /// Get the Linear acceleration raw data for OIS channel mode.
    pub fn ois_acceleration_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let arr = UiOutXYZAOisHg::read(self)?;
        Ok([arr.x, arr.y, arr.z])
    }

    /// Get the Linear acceleration raw data.
    pub fn acceleration_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = OutXYZA::read(self)?;

        Ok([val.x, val.y, val.z])
    }

    /// Get the High-G linear acceleration raw data.
    ///
    /// Require to enable reg_out_en parameter in `hg_xl_data_rate_set` function.
    pub fn hg_acceleration_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = UiOutXYZAOisHg::read(self)?;

        Ok([val.x, val.y, val.z])
    }

    /// Get the SFLP gbias raw array.
    pub fn sflp_gbias_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = MemBank::operate_over_embed(self, |lock| {
            SflpGbiasXYZ::read(lock).map(|arr| [arr.x, arr.y, arr.z])
        })?;

        Ok(val)
    }

    /// Get the SFLP gravity raw array.
    pub fn sflp_gravity_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = MemBank::operate_over_embed(self, |lock| {
            SflpGravXYZ::read(lock).map(|arr| [arr.x, arr.y, arr.z])
        })?;

        Ok(val)
    }

    /// Get the SFLP quaternions raw array.
    pub fn sflp_quaternion_raw_get(&mut self) -> Result<[u16; 4], Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            SflpQuatWXYZ::read(lock).map(|arr| [arr.w, arr.x, arr.y, arr.z])
        })
    }

    // Reset SFLP Game Rotation Vector Logic (6x).
    //
    // If val set to 1: SFLP game algorithm initialization request
    pub fn sflp_game_rotation_reset(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_init_a = EmbFuncInitA::read(lock)?;
            emb_func_init_a.set_sflp_game_init(val);
            emb_func_init_a.write(lock)
        })
    }

    /// Get the SFLP quaternions array.
    ///
    /// After the conversion bit to float.
    pub fn sflp_quaternion_get(&mut self) -> Result<Quaternion, Error<B::Error>> {
        let quat = self.sflp_quaternion_raw_get()?;
        let quat_w = crate::from_quaternion_lsb_to_float(quat[0]);
        let quat_x = crate::from_quaternion_lsb_to_float(quat[1]);
        let quat_y = crate::from_quaternion_lsb_to_float(quat[2]);
        let quat_z = crate::from_quaternion_lsb_to_float(quat[3]);
        Ok(Quaternion {
            quat_w,
            quat_x,
            quat_y,
            quat_z,
        })
    }

    /// Get the difference in percentage of the effective ODR (and timestamp rate)
    /// with respect to the typical. Step: 0.13%. 8-bit format, 2's complement.
    pub fn odr_cal_reg_get(&mut self) -> Result<i8, Error<B::Error>> {
        InternalFreq::read(self).map(|reg| reg.freq_fine() as i8)
    }

    /// Disable/Enable Embedded functions.
    ///
    /// If val equals to 1 disable the embedded functions
    pub fn disable_embedded_function_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut emb_func_cfg = EmbFuncCfg::read(self)?;
        emb_func_cfg.set_emb_func_disable(val & 0x1);
        emb_func_cfg.write(self)
    }

    /// Get the actual value (enable/disable) for Embedded functions.
    pub fn disable_embedded_function_get(&mut self) -> Result<u8, Error<B::Error>> {
        EmbFuncCfg::read(self).map(|reg| reg.emb_func_disable())
    }

    /// Enable/Disable embedded function sensor conversion.
    ///
    /// If val equals to 1, it enables the embedded functions sensor conversion.
    pub fn emb_func_conv_set(&mut self, val: EmbFuncConv) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut conv_reg = EmbFuncSensorConvEn::read(lock)?;
            conv_reg.set_xl_hg_conv_en(val.xl_hg_conv_en);
            conv_reg.set_gyro_conv_en(val.gyro_conv_en);
            conv_reg.set_temp_conv_en(val.temp_conv_en);
            conv_reg.set_ext_sensor_conv_en(val.ext_sensor_conv_en);
            conv_reg.write(lock)
        })
    }

    /// Enable/Disable embedded function sensor conversion.
    ///
    /// If returns 1 embedded functions are enabled
    pub fn emb_func_conv_get(&mut self) -> Result<EmbFuncConv, Error<B::Error>> {
        let conv_reg = MemBank::operate_over_embed(self, |lock| EmbFuncSensorConvEn::read(lock))?;

        let val = EmbFuncConv {
            xl_hg_conv_en: conv_reg.xl_hg_conv_en(),
            gyro_conv_en: conv_reg.gyro_conv_en(),
            temp_conv_en: conv_reg.temp_conv_en(),
            ext_sensor_conv_en: conv_reg.ext_sensor_conv_en(),
        };

        Ok(val)
    }

    /// Enable/Disable debug mode for embedded functions
    ///
    /// If val is 1 Enable debug mode for embedded functions
    pub fn emb_function_dbg_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl10 = Ctrl10::read(self)?;
        ctrl10.set_emb_func_debug(val);
        ctrl10.write(self)
    }

    /// Get configuration (enable/disable) debug mode for embedded functions
    pub fn emb_function_dbg_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl10::read(self).map(|reg| reg.emb_func_debug())
    }

    /// It changes the polarity of INT2 pin input trigger for data enable (DEN) or embedded functions.
    pub fn den_polarity_set(&mut self, val: DenPolarity) -> Result<(), Error<B::Error>> {
        let mut ctrl4 = Ctrl4::read(self)?;
        ctrl4.set_int2_in_lh((val as u8) & 0x1);
        ctrl4.write(self)
    }

    /// Get the polarity of INT2 pin input trigger for data enable (DEN) or embedded functions.
    pub fn den_polarity_get(&mut self) -> Result<DenPolarity, Error<B::Error>> {
        let ctrl4 = Ctrl4::read(self)?;
        let val = DenPolarity::try_from(ctrl4.int2_in_lh()).unwrap_or_default();
        Ok(val)
    }

    /// Gyroscope full-scale selection for EIS channel.
    ///
    /// WARNING: 4000dps will be available only if also User Interface chain is set to 4000dps.
    pub fn eis_gy_full_scale_set(&mut self, val: EisGyFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl_eis = CtrlEis::read(self)?;
        let mut ctrl2 = Ctrl2::read(self)?;
        let prev_ctrl2 = ctrl2.clone();

        // For the correct operation of the device, the user must set a
        // configuration from 001 to 101 when the gyroscope is in power-down mode.
        if ctrl2.odr_g() != (DataRate::Off as u8) {
            ctrl2.set_odr_g(DataRate::Off as u8);
            ctrl2.write(self)?;
        }

        ctrl_eis.set_fs_g_eis((val as u8) & 0x7);
        ctrl_eis.write(self)?;

        // restore previous odr set
        if prev_ctrl2.odr_g() != (DataRate::Off as u8) {
            prev_ctrl2.write(self)?
        }

        Ok(())
    }

    /// Get the actual Gyroscope full-scale selection for EIS channel.
    ///
    /// WARNING: 4000dps will be available only if also User Interface chain is set to 4000dps
    pub fn eis_gy_full_scale_get(&mut self) -> Result<EisGyFullScale, Error<B::Error>> {
        let ctrl_eis = CtrlEis::read(self)?;

        let val = EisGyFullScale::try_from(ctrl_eis.fs_g_eis()).unwrap_or_default();

        Ok(val)
    }

    /// Enables routing of gyroscope EIS outputs on IF2 (OIS interface).
    ///
    /// The gyroscope data on IF2 (OIS interface) cannot be read from User Interface (UI).
    pub fn eis_gy_on_if2_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl_eis = CtrlEis::read(self)?;
        ctrl_eis.set_g_eis_on_g_ois_out_reg(val);
        ctrl_eis.write(self)
    }

    /// Enables routing of gyroscope EIS outputs on IF2 (OIS interface).
    ///
    /// The gyroscope data on IF2 (OIS interface) cannot be read from User Interface (UI).
    pub fn eis_gy_on_if2_get(&mut self) -> Result<u8, Error<B::Error>> {
        CtrlEis::read(self).map(|reg| reg.g_eis_on_g_ois_out_reg())
    }

    /// Enables and selects the ODR of the gyroscope EIS channel.
    pub fn gy_eis_data_rate_set(&mut self, val: EisGyDataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl_eis = CtrlEis::read(self)?;
        ctrl_eis.set_odr_g_eis((val as u8) & 0x03);
        ctrl_eis.write(self)
    }

    /// Get the actual ODR of the gyroscope EIS channel.
    pub fn gy_eis_data_rate_get(&mut self) -> Result<EisGyDataRate, Error<B::Error>> {
        let ctrl_eis = CtrlEis::read(self)?;

        let val = EisGyDataRate::try_from(ctrl_eis.odr_g_eis()).unwrap_or_default();

        Ok(val)
    }

    /// Set FIFO watermark threshold
    ///
    /// 1 LSb = TAG (1 Byte) + 1 sensor (6 Bytes) written in FIFO.
    pub fn fifo_watermark_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl1 = FifoCtrl1::read(self)?;
        fifo_ctrl1.set_wtm(val);
        fifo_ctrl1.write(self)
    }

    /// Get the actual FIFO watermark threshold
    ///
    /// 1 LSb = TAG (1 Byte) + 1 sensor (6 Bytes) written in FIFO
    pub fn fifo_watermark_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoCtrl1::read(self).map(|reg| reg.wtm())
    }

    /// It configures the compression algorithm to write non-compressed data at each rate.
    pub fn fifo_compress_algo_set(&mut self, val: FifoCompressAlgo) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl2 = FifoCtrl2::read(self)?;
        fifo_ctrl2.set_uncompr_rate(val as u8 & 0x03);
        fifo_ctrl2.write(self)
    }

    /// Get the actual compression algorithm.
    ///
    /// The compression algorithm handles the write of non-compressed data at each rate.
    pub fn fifo_compress_algo_get(&mut self) -> Result<FifoCompressAlgo, Error<B::Error>> {
        let fifo_ctrl2 = FifoCtrl2::read(self)?;

        let val = FifoCompressAlgo::try_from(fifo_ctrl2.uncompr_rate()).unwrap_or_default();

        Ok(val)
    }

    /// Enables ODR CHANGE virtual sensor to be batched in FIFO.
    pub fn fifo_virtual_sens_odr_chg_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl2 = FifoCtrl2::read(self)?;
        fifo_ctrl2.set_odr_chg_en(val);
        fifo_ctrl2.write(self)
    }

    /// Get the configuration (enable/disable) of ODR CHANGE virtual sensor to be batched in FIFO.
    pub fn fifo_virtual_sens_odr_chg_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoCtrl2::read(self).map(|reg| reg.odr_chg_en())
    }

    /// Enables/Disables compression algorithm runtime.
    ///
    /// If val is 1: compression algorithm is active at runtime
    pub fn fifo_compress_algo_real_time_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl2 = FifoCtrl2::read(self)?;
        fifo_ctrl2.set_fifo_compr_rt_en(val);
        fifo_ctrl2.write(self)?;

        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_b = EmbFuncEnB::read(lock)?;
            emb_func_en_b.set_fifo_compr_en(val);
            emb_func_en_b.write(lock)
        })
    }

    /// Get the configuration (enable/disable) compression algorithm runtime.
    pub fn fifo_compress_algo_real_time_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoCtrl2::read(self).map(|reg| reg.fifo_compr_rt_en())
    }

    /// Sensing chain FIFO stop values memorization at threshold level.
    pub fn fifo_stop_on_wtm_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl2 = FifoCtrl2::read(self)?;
        fifo_ctrl2.set_stop_on_wtm(val);
        fifo_ctrl2.write(self)
    }

    /// Get the configuration (enable/disable) for sensing chain FIFO stop values memorization at threshold level.
    pub fn fifo_stop_on_wtm_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoCtrl2::read(self).map(|reg| reg.stop_on_wtm())
    }

    /// Selects Batch Data Rate (write frequency in FIFO) for accelerometer data.
    pub fn fifo_xl_batch_set(&mut self, val: FifoBatch) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl3 = FifoCtrl3::read(self)?;
        fifo_ctrl3.set_bdr_xl(val as u8 & 0xF);
        fifo_ctrl3.write(self)
    }

    /// Get the Batch Data Rate (write frequency in FIFO) for accelerometer data.
    pub fn fifo_xl_batch_get(&mut self) -> Result<FifoBatch, Error<B::Error>> {
        let fifo_ctrl3 = FifoCtrl3::read(self)?;
        let val = FifoBatch::try_from(fifo_ctrl3.bdr_xl()).unwrap_or_default();
        Ok(val)
    }

    /// Selects Batch Data Rate (write frequency in FIFO) for gyroscope data.
    pub fn fifo_gy_batch_set(&mut self, val: FifoBatch) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl3 = FifoCtrl3::read(self)?;
        fifo_ctrl3.set_bdr_gy((val as u8) & 0x0F);
        fifo_ctrl3.write(self)
    }

    /// Get Batch Data Rate (write frequency in FIFO) for gyroscope data.
    pub fn fifo_gy_batch_get(&mut self) -> Result<FifoBatch, Error<B::Error>> {
        let fifo_ctrl3 = FifoCtrl3::read(self)?;
        let val = FifoBatch::try_from(fifo_ctrl3.bdr_gy()).unwrap_or_default();
        Ok(val)
    }

    /// Get FIFO status
    ///
    /// Return a `FifoStatus` object
    pub fn fifo_status_get(&mut self) -> Result<FifoStatus, Error<B::Error>> {
        let status = FifoStatusReg::read(self)?;

        Ok(FifoStatus {
            fifo_bdr: status.counter_bdr_ia(),
            fifo_ovr: status.fifo_ovr_ia(),
            fifo_full: status.fifo_full_ia(),
            fifo_th: status.fifo_wtm_ia(),
            fifo_level: status.diff_fifo(),
        })
    }

    /// Enable FIFO Batch for hg XL data.
    ///
    /// # Arguments
    ///
    /// * `val`: 0 (disable) / 1 (enabled)
    pub fn fifo_hg_xl_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut cbdr_reg = CounterBdrReg1::read(self)?;
        cbdr_reg.set_xl_hg_batch_en(val & 0x01);
        cbdr_reg.write(self)
    }

    /// Get actual configuration of FIFO Batch for hg XL data.
    ///
    /// If returns 1, it's enabled
    pub fn fifo_hg_xl_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        CounterBdrReg1::read(self).map(|reg| reg.xl_hg_batch_en())
    }

    /// Set the FIFO mode.
    pub fn fifo_mode_set(&mut self, val: FifoMode) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl4 = FifoCtrl4::read(self)?;
        fifo_ctrl4.set_fifo_mode((val as u8) & 0x07);
        fifo_ctrl4.write(self)
    }

    /// Get the actual FIFO mode configuration
    pub fn fifo_mode_get(&mut self) -> Result<FifoMode, Error<B::Error>> {
        let fifo_ctrl4 = FifoCtrl4::read(self)?;
        let mode = FifoMode::try_from(fifo_ctrl4.fifo_mode()).unwrap_or_default();
        Ok(mode)
    }

    /// Enables/Disables FIFO batching of EIS gyroscope output values.
    ///
    /// If val is 1 FIFO batching is enabled
    pub fn fifo_gy_eis_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl4 = FifoCtrl4::read(self)?;
        fifo_ctrl4.set_g_eis_fifo_en(val);
        fifo_ctrl4.write(self)
    }

    /// Get the configuration (enabled/disabled) for FIFO batching of EIS gyroscope output values.
    pub fn fifo_gy_eis_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        FifoCtrl4::read(self).map(|reg| reg.g_eis_fifo_en())
    }

    /// Set batch data rate (write frequency in FIFO) for temperature data.
    pub fn fifo_temp_batch_set(&mut self, val: FifoTempBatch) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl4 = FifoCtrl4::read(self)?;
        fifo_ctrl4.set_odr_t_batch((val as u8) & 0x03);
        fifo_ctrl4.write(self)
    }

    /// Get actual batch data rate (write frequency in FIFO) for temperature data.
    pub fn fifo_temp_batch_get(&mut self) -> Result<FifoTempBatch, Error<B::Error>> {
        let fifo_ctrl4 = FifoCtrl4::read(self)?;

        let val = FifoTempBatch::try_from(fifo_ctrl4.odr_t_batch()).unwrap_or_default();
        Ok(val)
    }

    /// Selects decimation for timestamp batching in FIFO.
    ///
    /// Write rate will be the maximum rate between XL and GYRO BDR divided by decimation decoder.
    pub fn fifo_timestamp_batch_set(
        &mut self,
        val: FifoTimestampBatch,
    ) -> Result<(), Error<B::Error>> {
        let mut fifo_ctrl4 = FifoCtrl4::read(self)?;
        fifo_ctrl4.set_dec_ts_batch((val as u8) & 0x03);
        fifo_ctrl4.write(self)
    }

    /// Get the actual decimation for timestamp batching in FIFO.
    ///
    /// Write rate will be the maximum rate between XL and GYRO BDR divided by decimation decoder.
    pub fn fifo_timestamp_batch_get(&mut self) -> Result<FifoTimestampBatch, Error<B::Error>> {
        let fifo_ctrl4 = FifoCtrl4::read(self)?;

        let val = FifoTimestampBatch::try_from(fifo_ctrl4.dec_ts_batch()).unwrap_or_default();
        Ok(val)
    }

    /// Set the threshold for the internal counter of batch events.
    ///
    /// When this counter reaches the threshold, the counter is reset and the interrupt flag is set to 1.
    pub fn fifo_batch_counter_threshold_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        let mut cnt = CounterBdr::read(self)?;
        cnt.set_cnt_bdr_th(val);
        cnt.write(self)
    }

    /// Get the actual threshold for the internal counter of batch events.
    ///
    /// When this counter reaches the threshold, the counter is reset and the interrupt flag is set to 1.
    pub fn fifo_batch_counter_threshold_get(&mut self) -> Result<u16, Error<B::Error>> {
        CounterBdr::read(self).map(|reg| reg.cnt_bdr_th())
    }

    /// Selects the trigger for the internal counter of batch events.
    pub fn fifo_batch_cnt_event_set(
        &mut self,
        val: FifoBatchCntEvent,
    ) -> Result<(), Error<B::Error>> {
        let mut reg = CounterBdrReg1::read(self)?;
        reg.set_trig_counter_bdr((val as u8) & 0x03);
        reg.write(self)
    }

    /// Get the actual trigger for the internal counter of batch events.
    pub fn fifo_batch_cnt_event_get(&mut self) -> Result<FifoBatchCntEvent, Error<B::Error>> {
        let counter_bdr_reg1 = CounterBdrReg1::read(self)?;

        let event =
            FifoBatchCntEvent::try_from(counter_bdr_reg1.trig_counter_bdr()).unwrap_or_default();
        Ok(event)
    }

    /// Get the FIFO data tag and data.
    pub fn fifo_out_raw_get(&mut self) -> Result<FifoOutRaw, Error<B::Error>> {
        let fifo_data_out_tag = FifoDataOutTag::read(self)?;
        let tag = Tag::try_from(fifo_data_out_tag.tag_sensor()).unwrap_or_default();

        let cnt = fifo_data_out_tag.tag_cnt();
        let data = FifoDataOutXYZ::read(self)?.0;

        Ok(FifoOutRaw { tag, cnt, data })
    }

    /// Set the batching in FIFO buffer of step counter value.
    pub fn fifo_stpcnt_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_fifo_en_a = EmbFuncFifoEnA::read(lock)?;
            emb_func_fifo_en_a.set_step_counter_fifo_en(val);
            emb_func_fifo_en_a.write(lock)
        })
    }

    /// Get the acutal batching in FIFO buffer of step counter value.
    pub fn fifo_stpcnt_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        let emb_func_fifo_en_a =
            MemBank::operate_over_embed(self, |lock| EmbFuncFifoEnA::read(lock))?;

        let val: u8 = emb_func_fifo_en_a.step_counter_fifo_en();
        Ok(val)
    }

    /// Set Batching in FIFO buffer of finite state machine results.
    pub fn fifo_fsm_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_fifo_en_b = EmbFuncFifoEnB::read(lock)?;
            emb_func_fifo_en_b.set_fsm_fifo_en(val);
            emb_func_fifo_en_b.write(lock)
        })
    }

    /// Batching in FIFO buffer of finite state machine results.
    pub fn fifo_fsm_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncFifoEnB::read(lock).map(|reg| reg.fsm_fifo_en())
        })
    }

    /// Enables/Disables batching in FIFO buffer of machine learning core results.
    pub fn fifo_mlc_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_fifo_en_a = EmbFuncFifoEnA::read(lock)?;
            emb_func_fifo_en_a.set_mlc_fifo_en(val);
            emb_func_fifo_en_a.write(lock)
        })
    }

    /// Get the configuration (enables/disables) batching in FIFO buffer of machine learning core results.
    pub fn fifo_mlc_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncFifoEnA::read(lock).map(|reg| reg.mlc_fifo_en())
        })
    }

    /// Enables batching in FIFO buffer of machine learning core filters and features.
    pub fn fifo_mlc_filt_batch_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_fifo_en_b = EmbFuncFifoEnB::read(lock)?;
            emb_func_fifo_en_b.set_mlc_filter_feature_fifo_en(val);
            emb_func_fifo_en_b.write(lock)
        })
    }

    /// Get the configuration (enable/disable) of batching in FIFO buffer
    /// of machine learning core filters and features.
    pub fn fifo_mlc_filt_batch_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncFifoEnB::read(lock).map(|reg| reg.mlc_filter_feature_fifo_en())
        })
    }

    /// Enable FIFO data batching of target idx.
    pub fn fifo_sh_batch_target_set(&mut self, idx: u8, val: u8) -> Result<(), Error<B::Error>> {
        assert!(idx <= 3);
        MemBank::operate_over_sensor_hub(self, |state| {
            let mut arr: [u8; 1] = [0];
            state.read_from_register(SensHubReg::Tgt0Config as u8 + idx * 3, &mut arr)?;

            let mut tgt_config = Tgt0Config::from_bits(arr[0]);
            tgt_config.set_batch_ext_sens_0_en(val);
            state.write_to_register(
                SensHubReg::Tgt0Config as u8 + idx * 3,
                &[tgt_config.into_bits()],
            )
        })
    }

    /// Get the actual configuration (enable/disable) FIFO data batching of target idx.
    pub fn fifo_sh_batch_target_get(&mut self, idx: u8) -> Result<u8, Error<B::Error>> {
        assert!(idx <= 3);
        MemBank::operate_over_sensor_hub(self, |state| {
            let mut arr: [u8; 1] = [0];
            state.read_from_register(SensHubReg::Tgt0Config as u8 + idx * 3, &mut arr)?;
            let val: u8 = Tgt0Config::from_bits(arr[0]).batch_ext_sens_0_en();
            Ok(val)
        })
    }

    /// Enable/Disable Batching in FIFO buffer of SFLP.
    pub fn fifo_sflp_batch_set(&mut self, val: FifoSflpRaw) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_fifo_en_a = EmbFuncFifoEnA::read(lock)?;

            emb_func_fifo_en_a.set_sflp_game_fifo_en(val.game_rotation);
            emb_func_fifo_en_a.set_sflp_gravity_fifo_en(val.gravity);
            emb_func_fifo_en_a.set_sflp_gbias_fifo_en(val.gbias);

            emb_func_fifo_en_a.write(lock)
        })
    }

    /// Get the actual configuration (enable/disable) for Batching in FIFO buffer of SFLP.
    pub fn fifo_sflp_batch_get(&mut self) -> Result<FifoSflpRaw, Error<B::Error>> {
        let emb_func_fifo_en_a =
            MemBank::operate_over_embed(self, |lock| EmbFuncFifoEnA::read(lock))?;

        let val = FifoSflpRaw {
            game_rotation: emb_func_fifo_en_a.sflp_game_fifo_en(),
            gravity: emb_func_fifo_en_a.sflp_gravity_fifo_en(),
            gbias: emb_func_fifo_en_a.sflp_gbias_fifo_en(),
        };

        Ok(val)
    }

    /// Set Protocol anti-spike filters.
    pub fn filt_anti_spike_set(&mut self, val: FiltAntiSpike) -> Result<(), Error<B::Error>> {
        let mut if_cfg = IfCfg::read(self)?;
        if_cfg.set_asf_ctrl((val as u8) & 0x01);
        if_cfg.write(self)
    }

    /// Get the actual Protocol anti-spike filters.
    pub fn filt_anti_spike_get(&mut self) -> Result<FiltAntiSpike, Error<B::Error>> {
        let if_cfg = IfCfg::read(self)?;
        let val = FiltAntiSpike::try_from(if_cfg.asf_ctrl()).unwrap_or_default();

        Ok(val)
    }

    /// It masks DRDY and Interrupts RQ until filter settling ends.
    pub fn filt_settling_mask_set(&mut self, val: FiltSettlingMask) -> Result<(), Error<B::Error>> {
        let mut ctrl4 = Ctrl4::read(self)?;
        ctrl4.set_drdy_mask(val.drdy);
        ctrl4.write(self)?;

        let mut emb_func_cfg = EmbFuncCfg::read(self)?;
        emb_func_cfg.set_emb_func_irq_mask_xl_settl(val.irq_xl);
        emb_func_cfg.set_emb_func_irq_mask_xl_hg_settl(val.irq_xl_hg);
        emb_func_cfg.set_emb_func_irq_mask_g_settl(val.irq_g);
        emb_func_cfg.write(self)?;

        let mut ui_int_ois = UiIntOis::read(self)?;
        ui_int_ois.set_drdy_mask_ois(val.ois_drdy);
        ui_int_ois.write(self)?;
        Ok(())
    }

    /// Get the configuration for masks DRDY and Interrupts RQ.
    pub fn filt_settling_mask_get(&mut self) -> Result<FiltSettlingMask, Error<B::Error>> {
        let ctrl4 = Ctrl4::read(self)?;

        let emb_func_cfg = EmbFuncCfg::read(self)?;

        let ui_int_ois = UiIntOis::read(self)?;

        let val = FiltSettlingMask {
            drdy: ctrl4.drdy_mask(),
            ois_drdy: ui_int_ois.drdy_mask_ois(),
            irq_xl: emb_func_cfg.emb_func_irq_mask_xl_settl(),
            irq_xl_hg: emb_func_cfg.emb_func_irq_mask_xl_hg_settl(),
            irq_g: emb_func_cfg.emb_func_irq_mask_g_settl(),
        };

        Ok(val)
    }

    /// Set the masks for DRDY and Interrupts RQ until filter settling ends.
    pub fn filt_ois_settling_mask_set(
        &mut self,
        val: FiltOisSettlingMask,
    ) -> Result<(), Error<B::Error>> {
        let mut reg = If2IntOis::read(self)?;
        reg.set_drdy_mask_ois(val.ois_drdy);
        reg.write(self)
    }

    /// Get the masks DRDY and Interrupts RQ until filter settling ends.
    pub fn filt_ois_settling_mask_get(&mut self) -> Result<FiltOisSettlingMask, Error<B::Error>> {
        let ois_drdy: u8 = If2IntOis::read(self)?.drdy_mask_ois();
        Ok(FiltOisSettlingMask { ois_drdy })
    }

    /// Set the Gyroscope low-pass filter (LPF1) bandwidth
    pub fn filt_gy_lp1_bandwidth_set(
        &mut self,
        val: FiltLpBandwidth,
    ) -> Result<(), Error<B::Error>> {
        let mut ctrl6 = Ctrl6::read(self)?;
        ctrl6.set_lpf1_g_bw((val as u8) & 0x0F);
        ctrl6.write(self)
    }

    /// Get the gyroscope low-pass filter (LPF1) bandwidth.
    pub fn filt_gy_lp1_bandwidth_get(&mut self) -> Result<FiltLpBandwidth, Error<B::Error>> {
        let ctrl6 = Ctrl6::read(self)?;
        let val = FiltLpBandwidth::try_from(ctrl6.lpf1_g_bw()).unwrap_or_default();

        Ok(val)
    }

    /// Enables/Disable gyroscope digital LPF1 filter.
    pub fn filt_gy_lp1_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_lpf1_g_en(val);
        ctrl7.write(self)
    }

    /// Get the configuration (enables/disables) gyroscope digital LPF1 filter.
    pub fn filt_gy_lp1_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl7::read(self).map(|ctrl7| ctrl7.lpf1_g_en())
    }

    /// Setup xl filter pipeline for lpf1 filter to UI
    pub fn filt_xl_setup(
        &mut self,
        filter: XlFilter,
        bw: FiltLpBandwidth,
        hp_ref_mode_xl: u8,
    ) -> Result<(), Error<B::Error>> {
        if (filter == XlFilter::Hp && bw == FiltLpBandwidth::UltraLight)
            || (hp_ref_mode_xl == 1 && filter != XlFilter::Hp)
            || (filter == XlFilter::HpSlope && (bw as u8) != 0x0)
        {
            return Err(Error::InvalidConfiguration);
        }

        let mut ctrl8 = Ctrl8::read(self)?;
        let mut ctrl9 = Ctrl9::read(self)?;

        match filter {
            XlFilter::Lpf2 => {
                ctrl9.set_hp_slope_xl_en(0);
                ctrl9.set_lpf2_xl_en(1);
            }
            XlFilter::Lpf1 => {
                ctrl9.set_hp_slope_xl_en(0);
                ctrl9.set_lpf2_xl_en(0);
            }
            XlFilter::Hp | XlFilter::HpSlope => {
                ctrl9.set_hp_slope_xl_en(1);
                ctrl9.set_lpf2_xl_en(0);
            }
        }

        ctrl8.set_hp_lpf2_xl_bw(bw as u8);
        ctrl9.set_hp_ref_mode_xl(hp_ref_mode_xl);

        ctrl8.write(self)?;
        ctrl9.write(self)
    }

    /// Set the Accelerometer LPF2 and high pass filter configuration and cutoff setting.
    pub fn filt_xl_lp2_bandwidth_set(
        &mut self,
        val: FiltLpBandwidth,
    ) -> Result<(), Error<B::Error>> {
        let mut ctrl8 = Ctrl8::read(self)?;
        ctrl8.set_hp_lpf2_xl_bw((val as u8) & 0x07);
        ctrl8.write(self)
    }

    /// Get the current Accelerometer LPF2 and high pass filter configuration and cutoff setting.
    pub fn filt_xl_lp2_bandwidth_get(&mut self) -> Result<FiltLpBandwidth, Error<B::Error>> {
        let ctrl8 = Ctrl8::read(self)?;
        let val = FiltLpBandwidth::try_from(ctrl8.hp_lpf2_xl_bw()).unwrap_or_default();

        Ok(val)
    }

    /// Enable/Disable accelerometer LPS2 (Low Pass Filter 2) filtering stage.
    pub fn filt_xl_lp2_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl9 = Ctrl9::read(self)?;
        ctrl9.set_lpf2_xl_en(val);
        ctrl9.write(self)
    }

    /// Get the accelerometer LPS2 (Low Pass Filter 2) filtering stage.
    pub fn filt_xl_lp2_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl9::read(self).map(|ctrl9| ctrl9.lpf2_xl_en())
    }

    /// Accelerometer slope filter / high-pass filter selection.
    pub fn filt_xl_hp_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl9 = Ctrl9::read(self)?;
        ctrl9.set_hp_slope_xl_en(val);
        ctrl9.write(self)
    }

    /// Get the Accelerometer slope filter / high-pass filter selection.
    pub fn filt_xl_hp_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl9::read(self).map(|ctrl9| ctrl9.hp_slope_xl_en())
    }

    /// Enables accelerometer LPF2 and HPF fast-settling mode. The filter sets the first sample.
    pub fn filt_xl_fast_settling_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl9 = Ctrl9::read(self)?;
        ctrl9.set_xl_fastsettl_mode(val);
        ctrl9.write(self)
    }

    /// Get accelerometer LPF2 and HPF fast-settling mode. The filter sets the first sample.
    pub fn filt_xl_fast_settling_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ctrl9::read(self).map(|ctrl9| ctrl9.xl_fastsettl_mode())
    }

    /// Set Accelerometer high-pass filter mode.
    pub fn filt_xl_hp_mode_set(&mut self, val: FiltXlHpMode) -> Result<(), Error<B::Error>> {
        let mut ctrl9 = Ctrl9::read(self)?;
        ctrl9.set_hp_ref_mode_xl((val as u8) & 0x01);
        ctrl9.set_hp_slope_xl_en(((val as u8) & 0x02) >> 1);
        ctrl9.write(self)
    }

    /// Get Accelerometer high-pass filter mode.
    pub fn filt_xl_hp_mode_get(&mut self) -> Result<FiltXlHpMode, Error<B::Error>> {
        let ctrl9 = Ctrl9::read(self)?;
        let value = ctrl9.hp_ref_mode_xl() | (ctrl9.hp_slope_xl_en() << 1);

        let val = FiltXlHpMode::try_from(value).unwrap_or_default();
        Ok(val)
    }

    /// Filter wakeup activity feed set.
    ///
    /// Set HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions.
    pub fn filt_wkup_act_feed_set(&mut self, val: FiltWkupActFeed) -> Result<(), Error<B::Error>> {
        let mut wake_up_ths = WakeUpThs::read(self)?;
        let mut tap_cfg0 = TapCfg0::read(self)?;

        tap_cfg0.set_slope_fds((val as u8) & 0x01);
        tap_cfg0.write(self)?;

        wake_up_ths.set_usr_off_on_wu(((val as u8) & 0x02) >> 1);
        wake_up_ths.write(self)?;

        Ok(())
    }

    /// Filter wakeup activity feed get.
    ///
    /// Get the actual HPF or SLOPE filter on wake-up and Activity/Inactivity functions.
    pub fn filt_wkup_act_feed_get(&mut self) -> Result<FiltWkupActFeed, Error<B::Error>> {
        let wake_up_ths = WakeUpThs::read(self)?;
        let tap_cfg0 = TapCfg0::read(self)?;

        let value = (wake_up_ths.usr_off_on_wu() << 1) + tap_cfg0.slope_fds();
        let result = FiltWkupActFeed::try_from(value).unwrap_or_default();
        Ok(result)
    }

    /// Mask hw function triggers when xl is settling.
    ///
    /// If val is 1 it enables the masking
    pub fn mask_trigger_xl_settl_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut tap_cfg0 = TapCfg0::read(self)?;
        tap_cfg0.set_hw_func_mask_xl_settl(val & 0x01);
        tap_cfg0.write(self)
    }

    /// Get current configuration (enable/disable) of mask hw function
    ///
    /// triggers when xl is settling.
    pub fn mask_trigger_xl_settl_get(&mut self) -> Result<u8, Error<B::Error>> {
        TapCfg0::read(self).map(|tap_cfg0| tap_cfg0.hw_func_mask_xl_settl())
    }

    /// Configure the LPF2 filter on 6D (sixd) function.
    pub fn filt_sixd_feed_set(&mut self, val: FiltSixdFeed) -> Result<(), Error<B::Error>> {
        let mut tap_cfg0 = TapCfg0::read(self)?;
        tap_cfg0.set_low_pass_on_6d((val as u8) & 0x01);
        tap_cfg0.write(self)
    }

    /// Get the actual LPF2 filter on 6D (sixd) function selection.
    pub fn filt_sixd_feed_get(&mut self) -> Result<FiltSixdFeed, Error<B::Error>> {
        let tap_cfg0 = TapCfg0::read(self)?;
        let val = FiltSixdFeed::try_from(tap_cfg0.low_pass_on_6d()).unwrap_or_default();

        Ok(val)
    }

    /// Set the gyroscope digital LPF_EIS filter bandwidth.
    pub fn filt_gy_eis_lp_bandwidth_set(
        &mut self,
        val: FiltGyEisLpBandwidth,
    ) -> Result<(), Error<B::Error>> {
        let mut ctrl_eis = CtrlEis::read(self)?;
        ctrl_eis.set_lpf_g_eis_bw((val as u8) & 0x01);
        ctrl_eis.write(self)
    }

    /// Get the current gyroscope digital LPF_EIS filter bandwidth selection.
    pub fn filt_gy_eis_lp_bandwidth_get(
        &mut self,
    ) -> Result<FiltGyEisLpBandwidth, Error<B::Error>> {
        let ctrl_eis = CtrlEis::read(self)?;
        let val = FiltGyEisLpBandwidth::try_from(ctrl_eis.lpf_g_eis_bw()).unwrap_or_default();

        Ok(val)
    }

    /// Set the Gyroscope OIS digital LPF1 filter bandwidth.
    ///
    /// This function works also on OIS interface (IF2_CTRL2_OIS = UI_CTRL2_OIS).
    pub fn filt_gy_ois_lp_bandwidth_set(
        &mut self,
        val: FiltGyOisLpBandwidth,
    ) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl2_ois = UiCtrl2Ois::read(self)?;
        ui_ctrl2_ois.set_lpf1_g_ois_bw(val as u8 & 0x03);
        ui_ctrl2_ois.write(self)
    }

    /// Get the current gyroscope OIS digital LPF1 filter bandwidth.
    ///
    /// This function works also on OIS interface (IF2_CTRL2_OIS = UI_CTRL2_OIS).
    pub fn filt_gy_ois_lp_bandwidth_get(
        &mut self,
    ) -> Result<FiltGyOisLpBandwidth, Error<B::Error>> {
        let ui_ctrl2_ois = UiCtrl2Ois::read(self)?;

        let val = FiltGyOisLpBandwidth::try_from(ui_ctrl2_ois.lpf1_g_ois_bw()).unwrap_or_default();
        Ok(val)
    }

    /// Set accelerometer OIS channel bandwidth.
    ///
    /// This function works also on OIS interface (IF2_CTRL3_OIS = UI_CTRL3_OIS).
    pub fn filt_xl_ois_lp_bandwidth_set(
        &mut self,
        val: FiltXlOisLpBandwidth,
    ) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl3_ois = UiCtrl3Ois::read(self)?;
        ui_ctrl3_ois.set_lpf_xl_ois_bw((val as u8) & 0x07);
        ui_ctrl3_ois.write(self)
    }

    /// Get the accelerometer OIS channel bandwidth.
    ///
    /// This function works also on OIS interface (IF2_CTRL3_OIS = UI_CTRL3_OIS).
    pub fn filt_xl_ois_lp_bandwidth_get(
        &mut self,
    ) -> Result<FiltXlOisLpBandwidth, Error<B::Error>> {
        let ui_ctrl3_ois = UiCtrl3Ois::read(self)?;

        let val = FiltXlOisLpBandwidth::try_from(ui_ctrl3_ois.lpf_xl_ois_bw()).unwrap_or_default();
        Ok(val)
    }

    /// Enables the control of the CTRL registers to FSM.
    ///
    /// Warning: FSM can change some configurations of the device autonomously.
    pub fn fsm_permission_set(&mut self, val: FsmPermission) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::read(self)?;
        func_cfg_access.set_fsm_wr_ctrl_en((val as u8) & 0x01);
        func_cfg_access.write(self)
    }

    /// Get the FSM permission to change the CTRL registers.
    pub fn fsm_permission_get(&mut self) -> Result<FsmPermission, Error<B::Error>> {
        let func_cfg_access = FuncCfgAccess::read(self)?;

        let val = FsmPermission::try_from(func_cfg_access.fsm_wr_ctrl_en()).unwrap_or_default();
        Ok(val)
    }

    /// Get the FSM permission status
    ///
    /// Returns 0 if all registers writable from standard interface;
    /// 1 if some registers are under FSM control.
    pub fn fsm_permission_status(&mut self) -> Result<u8, Error<B::Error>> {
        CtrlStatus::read(self).map(|reg| reg.fsm_wr_ctrl_status())
    }

    /// Enable Finite State Machine (FSM) feature.
    pub fn fsm_mode_set(&mut self, val: FsmMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_b = EmbFuncEnB::read(lock)?;
            let mut fsm_enable = FsmEnable::read(lock)?;

            if (val.fsm1_en
                | val.fsm2_en
                | val.fsm3_en
                | val.fsm4_en
                | val.fsm5_en
                | val.fsm6_en
                | val.fsm7_en
                | val.fsm8_en)
                == 1
            {
                emb_func_en_b.set_fsm_en(1);
            } else {
                emb_func_en_b.set_fsm_en(0);
            }

            fsm_enable.set_fsm1_en(val.fsm1_en);
            fsm_enable.set_fsm2_en(val.fsm2_en);
            fsm_enable.set_fsm3_en(val.fsm3_en);
            fsm_enable.set_fsm4_en(val.fsm4_en);
            fsm_enable.set_fsm5_en(val.fsm5_en);
            fsm_enable.set_fsm6_en(val.fsm6_en);
            fsm_enable.set_fsm7_en(val.fsm7_en);
            fsm_enable.set_fsm8_en(val.fsm8_en);

            fsm_enable.write(lock)?;
            emb_func_en_b.write(lock)?;

            Ok(())
        })
    }

    /// Get the enabled Finite State Machine (FSM) feature.
    pub fn fsm_mode_get(&mut self) -> Result<FsmMode, Error<B::Error>> {
        let fsm_enable = MemBank::operate_over_embed(self, |lock| FsmEnable::read(lock))?;

        let val = FsmMode {
            fsm1_en: fsm_enable.fsm1_en(),
            fsm2_en: fsm_enable.fsm2_en(),
            fsm3_en: fsm_enable.fsm3_en(),
            fsm4_en: fsm_enable.fsm4_en(),
            fsm5_en: fsm_enable.fsm5_en(),
            fsm6_en: fsm_enable.fsm6_en(),
            fsm7_en: fsm_enable.fsm7_en(),
            fsm8_en: fsm_enable.fsm8_en(),
        };
        Ok(val)
    }

    /// Set the FSM long counter status register.
    ///
    /// Long counter value is an unsigned integer value (16-bit format).
    pub fn fsm_long_cnt_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| FsmLongCounter(val).write(lock))
    }

    /// Get the FSM long counter status register.
    ///
    /// Long counter value is an unsigned integer value (16-bit format).
    pub fn fsm_long_cnt_get(&mut self) -> Result<u16, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| FsmLongCounter::read(lock).map(|cnt| cnt.0))
    }

    /// Get the FSM output results.
    pub fn fsm_out_get(&mut self) -> Result<FsmOut, Error<B::Error>> {
        let mut buf: [u8; 8] = [0; 8];
        MemBank::operate_over_embed(self, |lock| FsmOuts1::read_more(lock, &mut buf))?;

        Ok(FsmOut::from_le_bytes(buf))
    }

    /// Set the Finite State Machine Output Data Rate (ODR).
    pub fn fsm_data_rate_set(&mut self, val: FsmDataRate) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut fsm_odr = FsmOdr::read(lock)?;
            fsm_odr.set_fsm_odr((val as u8) & 0x07);
            fsm_odr.write(lock)
        })
    }

    /// Get the finite State Machine Output Data Rate (ODR) configuration.
    pub fn fsm_data_rate_get(&mut self) -> Result<FsmDataRate, Error<B::Error>> {
        let fsm_odr =
            MemBank::operate_over_embed(self, |lock| FsmOdr::read(lock).map(|reg| reg.fsm_odr()))?;

        let val = FsmDataRate::try_from(fsm_odr).unwrap_or_default();
        Ok(val)
    }

    /// Set SFLP GBIAS value for x/y/z axis.
    ///
    /// The register value is expressed as half-precision
    /// floating-point format: SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent
    /// bits; F: 10 fraction bits).
    pub fn sflp_game_gbias_set(&mut self, val: &SflpGbias) -> Result<(), Error<B::Error>> {
        let sflp_odr = self.sflp_data_rate_get()?;

        let k = match sflp_odr {
            SflpDataRate::_30hz => 0.02,
            SflpDataRate::_60hz => 0.01,
            SflpDataRate::_120hz => 0.005,
            SflpDataRate::_240hz => 0.0025,
            SflpDataRate::_480hz => 0.00125,
            _ => 0.04, // SflpDataRate::_15hz => 0.04,
        };

        MemBank::operate_over_embed(self, |lock| {
            // compute gbias as half precision float in order to be put in embedded advanced feature register
            let gbias = SflpGbiasXYZInit {
                x: crate::npy_float_to_half(val.gbias_x * (core::f32::consts::PI / 180.0) / k),
                y: crate::npy_float_to_half(val.gbias_y * (core::f32::consts::PI / 180.0) / k),
                z: crate::npy_float_to_half(val.gbias_z * (core::f32::consts::PI / 180.0) / k),
            };
            gbias.write(lock)
        })
    }

    /// Set the External sensor sensitivity value register for the Finite State Machine.
    ///
    /// This register corresponds to the conversion value of the external sensor.
    /// The register value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    /// Default value is 0x1624 (when using an external magnetometer this value
    /// corresponds to 0.0015 gauss/LSB).
    pub fn fsm_ext_sens_sensitivity_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        FsmExtSensitivity(val).write(self)
    }

    /// Get the External sensor sensitivity value register for the Finite State Machine.
    ///
    /// This register corresponds to the conversion value of the external sensor.
    /// The register value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    /// Default value is 0x1624 (when using an external magnetometer this value
    /// corresponds to 0.0015 gauss/LSB).
    pub fn fsm_ext_sens_sensitivity_get(&mut self) -> Result<u16, Error<B::Error>> {
        FsmExtSensitivity::read(self).map(|reg| reg.0)
    }

    /// Set External sensor offsets (X,Y,Z).
    ///
    /// The values are expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    pub fn fsm_ext_sens_offset_set(
        &mut self,
        val: XlFsmExtSensOffset,
    ) -> Result<(), Error<B::Error>> {
        let mut buff: [u8; 6] = [0; 6];

        buff[1] = (val.x >> 8) as u8;
        buff[0] = (val.x & 0xFF) as u8;
        buff[3] = (val.y >> 8) as u8;
        buff[2] = (val.y & 0xFF) as u8;
        buff[5] = (val.z >> 8) as u8;
        buff[4] = (val.z & 0xFF) as u8;

        self.ln_pg_write(EmbAdv0Reg::FsmExtOffxL as u16, &buff, 6)?;

        Ok(())
    }

    /// Get External sensor offsets (X,Y,Z).
    ///
    /// The values are expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    pub fn fsm_ext_sens_offset_get(&mut self) -> Result<XlFsmExtSensOffset, Error<B::Error>> {
        let mut buff: [u8; 6] = [0; 6];
        FsmExtOffxL::read_more(self, &mut buff)?;

        let x = ((buff[1] as u16) << 8) | (buff[0] as u16);
        let y = ((buff[3] as u16) << 8) | (buff[2] as u16);
        let z = ((buff[5] as u16) << 8) | (buff[4] as u16);

        Ok(XlFsmExtSensOffset { x, y, z })
    }

    /// Set External sensor transformation matrix.
    ///
    /// The value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    pub fn fsm_ext_sens_matrix_set(
        &mut self,
        val: XlFsmExtSensMatrix,
    ) -> Result<(), Error<B::Error>> {
        let mut buff: [u8; 12] = [0; 12];

        buff[0] = (val.xx & 0xFF) as u8;
        buff[1] = (val.xx >> 8) as u8;
        buff[2] = (val.xy & 0xFF) as u8;
        buff[3] = (val.xy >> 8) as u8;
        buff[4] = (val.xz & 0xFF) as u8;
        buff[5] = (val.xz >> 8) as u8;
        buff[6] = (val.yy & 0xFF) as u8;
        buff[7] = (val.yy >> 8) as u8;
        buff[8] = (val.yz & 0xFF) as u8;
        buff[9] = (val.yz >> 8) as u8;
        buff[10] = (val.zz & 0xFF) as u8;
        buff[11] = (val.zz >> 8) as u8;

        self.ln_pg_write(EmbAdv0Reg::FsmExtMatrixXxL as u16, &buff, 12)?;
        Ok(())
    }

    /// Get the External sensor transformation matrix.
    ///
    /// The value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    pub fn fsm_ext_sens_matrix_get(&mut self) -> Result<XlFsmExtSensMatrix, Error<B::Error>> {
        let mut buff: [u8; 12] = [0; 12];
        FsmExtMatrixXxL::read_more(self, &mut buff)?;

        let xx = (buff[1] as u16) << 8 | buff[0] as u16;
        let xy = (buff[3] as u16) << 8 | buff[2] as u16;
        let xz = (buff[5] as u16) << 8 | buff[4] as u16;
        let yy = (buff[7] as u16) << 8 | buff[6] as u16;
        let yz = (buff[9] as u16) << 8 | buff[8] as u16;
        let zz = (buff[11] as u16) << 8 | buff[10] as u16;

        Ok(XlFsmExtSensMatrix {
            xx,
            xy,
            xz,
            yy,
            yz,
            zz,
        })
    }

    /// Set External sensor z-axis coordinates rotation.
    pub fn fsm_ext_sens_z_orient_set(
        &mut self,
        val: FsmExtSensZOrient,
    ) -> Result<(), Error<B::Error>> {
        let mut ext_cfg_a = ExtCfgA::read(self)?;
        ext_cfg_a.set_ext_z_axis((val as u8) & 0x07);
        ext_cfg_a.write(self)
    }

    /// Get External sensor z-axis coordinates rotation.
    pub fn fsm_ext_sens_z_orient_get(&mut self) -> Result<FsmExtSensZOrient, Error<B::Error>> {
        let ext_cfg_a = ExtCfgA::read(self)?;

        let val = FsmExtSensZOrient::try_from(ext_cfg_a.ext_z_axis()).unwrap_or_default();
        Ok(val)
    }

    /// Set External sensor Y-axis coordinates rotation.
    pub fn fsm_ext_sens_y_orient_set(
        &mut self,
        val: FsmExtSensYOrient,
    ) -> Result<(), Error<B::Error>> {
        let mut ext_cfg_a = ExtCfgA::read(self)?;
        ext_cfg_a.set_ext_y_axis((val as u8) & 0x7);
        ext_cfg_a.write(self)
    }

    /// Get External sensor Y-axis coordinates rotation.
    pub fn fsm_ext_sens_y_orient_get(&mut self) -> Result<FsmExtSensYOrient, Error<B::Error>> {
        let ext_cfg_a = ExtCfgA::read(self)?;

        let val = FsmExtSensYOrient::try_from(ext_cfg_a.ext_y_axis()).unwrap_or_default();
        Ok(val)
    }

    /// Set External sensor X-axis coordinates rotation.
    pub fn fsm_ext_sens_x_orient_set(
        &mut self,
        val: FsmExtSensXOrient,
    ) -> Result<(), Error<B::Error>> {
        let mut ext_cfg_b = ExtCfgB::read(self)?;
        ext_cfg_b.set_ext_x_axis((val as u8) & 0x7);
        ext_cfg_b.write(self)
    }

    /// Get External sensor X-axis coordinates rotation.
    pub fn fsm_ext_sens_x_orient_get(&mut self) -> Result<FsmExtSensXOrient, Error<B::Error>> {
        let ext_cfg_b = ExtCfgB::read(self)?;

        let val = FsmExtSensXOrient::try_from(ext_cfg_b.ext_x_axis()).unwrap_or_default();
        Ok(val)
    }

    /// Enable/disable High-g accelerometer peak tracking.
    ///
    /// If val equals to 1 enables the feature
    pub fn xl_hg_peak_tracking_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_init_b = EmbFuncInitB::read(lock)?;
            emb_func_init_b.set_pt_init(val);
            emb_func_init_b.write(lock)
        })
    }

    /// Get the configuration (enable/disable) for High-g accelerometer peak tracking enable.
    pub fn xl_hg_peak_tracking_get(&mut self) -> Result<u8, Error<B::Error>> {
        let emb_func_init_b = MemBank::operate_over_embed(self, |lock| EmbFuncInitB::read(lock))?;
        Ok(emb_func_init_b.pt_init())
    }

    /// Set the High-g accelerometer sensitivity value register for FSM and MLC.
    pub fn xl_hg_sensitivity_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        XlHgSensitivity::from_bits(val).write(self)
    }

    /// Get the High-g accelerometer sensitivity value register for FSM and MLC.
    pub fn xl_hg_sensitivity_get(&mut self) -> Result<u16, Error<B::Error>> {
        XlHgSensitivity::read(self).map(|reg| reg.xl_hg())
    }

    /// Set FSM long counter timeout.
    ///
    /// The long counter timeout value is an unsigned integer value (16-bit format).
    /// When the long counter value reached this value, the FSM generates an interrupt.
    pub fn fsm_long_cnt_timeout_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        FsmLcTimeout(val).write(self)
    }

    /// Get the current FSM long counter timeout.
    ///
    /// The long counter timeout value is an unsigned integer value (16-bit format).
    /// When the long counter value reached this value, the FSM generates an interrupt.
    pub fn fsm_long_cnt_timeout_get(&mut self) -> Result<u16, Error<B::Error>> {
        FsmLcTimeout::read(self).map(|reg| reg.0)
    }

    /// Set the FSM number of programs.
    ///
    /// Must be less than or equal to 8. Default 0.
    pub fn fsm_number_of_programs_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut fsm_programs = FsmPrograms::read(self)?;
        fsm_programs.set_fsm_n_prog(val);
        fsm_programs.write(self)
    }

    /// Get the actual FSM number of programs.
    pub fn fsm_number_of_programs_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val = FsmPrograms::read(self)?;
        Ok(val.fsm_n_prog())
    }

    /// Set the FSM start address.
    ///
    /// First available address is 0x35C.
    pub fn fsm_start_address_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        FsmStartAdd(val).write(self)
    }

    /// Get the actual FSM start address.
    ///
    /// First available address is 0x35C.
    pub fn fsm_start_address_get(&mut self) -> Result<u16, Error<B::Error>> {
        FsmStartAdd::read(self).map(|reg| reg.0)
    }

    /// Set the time windows configuration for Free Fall detection.
    ///
    /// 1 LSB = 1/ODR_XL time
    pub fn ff_time_windows_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut wake_up_dur = WakeUpDur::read(self)?;
        wake_up_dur.set_ff_dur((val & 0x20) >> 5);
        wake_up_dur.write(self)?;

        let mut free_fall = FreeFall::read(self)?;
        free_fall.set_ff_dur(val & 0x1F);
        free_fall.write(self)
    }

    /// Get the time windows configuration for Free Fall detection.
    ///
    /// 1 LSB = 1/ODR_XL time
    pub fn ff_time_windows_get(&mut self) -> Result<u8, Error<B::Error>> {
        let wake_up_dur = WakeUpDur::read(self)?;
        let free_fall = FreeFall::read(self)?;

        let val = (wake_up_dur.ff_dur() << 5) + free_fall.ff_dur();
        Ok(val)
    }

    /// Set the Free fall threshold.
    pub fn ff_thresholds_set(&mut self, val: FfThreshold) -> Result<(), Error<B::Error>> {
        let mut free_fall = FreeFall::read(self)?;
        free_fall.set_ff_ths((val as u8) & 0x7);
        free_fall.write(self)
    }

    /// Get the current Free fall threshold setting.
    pub fn ff_thresholds_get(&mut self) -> Result<FfThreshold, Error<B::Error>> {
        let free_fall = FreeFall::read(self)?;
        let val = FfThreshold::try_from(free_fall.ff_ths()).unwrap_or_default();

        Ok(val)
    }

    /// Set Machine Learning Core mode (MLC).
    ///
    /// When the Machine Learning Core is enabled the Finite State Machine (FSM)
    /// programs are executed before executing the MLC algorithms.
    pub fn mlc_set(&mut self, val: MlcMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_en_a = EmbFuncEnA::read(lock)?;
            let mut emb_en_b = EmbFuncEnB::read(lock)?;

            match val {
                MlcMode::Off => {
                    emb_en_a.set_mlc_before_fsm_en(0);
                    emb_en_b.set_mlc_en(0);
                }
                MlcMode::On => {
                    emb_en_a.set_mlc_before_fsm_en(0);
                    emb_en_b.set_mlc_en(1);
                }
                MlcMode::OnBeforeFsm => {
                    emb_en_a.set_mlc_before_fsm_en(1);
                    emb_en_b.set_mlc_en(0);
                }
            }

            emb_en_a.write(lock)?;
            emb_en_b.write(lock)
        })
    }

    /// Get the configuration ofMachine Learning Core (MLC).
    ///
    /// When the Machine Learning Core is enabled the Finite State Machine (FSM)
    /// programs are executed before executing the MLC algorithms.
    pub fn mlc_get(&mut self) -> Result<MlcMode, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let emb_en_a = EmbFuncEnA::read(lock)?;
            let emb_en_b = EmbFuncEnB::read(lock)?;

            let val = if emb_en_a.mlc_before_fsm_en() == 0 && emb_en_b.mlc_en() == 0 {
                MlcMode::Off
            } else if emb_en_a.mlc_before_fsm_en() == 0 && emb_en_b.mlc_en() == 1 {
                MlcMode::On
            } else {
                MlcMode::OnBeforeFsm
            };

            Ok(val)
        })
    }

    /// Set Machine Learning Core Output Data Rate (ODR).
    pub fn mlc_data_rate_set(&mut self, val: MlcDataRate) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut mlc_odr = MlcOdr::read(lock)?;
            mlc_odr.set_mlc_odr((val as u8) & 0x07);
            mlc_odr.write(lock)
        })
    }

    /// Get the Machine Learning Core Output Data Rate (ODR).
    pub fn mlc_data_rate_get(&mut self) -> Result<MlcDataRate, Error<B::Error>> {
        let mlc_odr = MemBank::operate_over_embed(self, |lock| MlcOdr::read(lock))?;

        let val = MlcDataRate::try_from(mlc_odr.mlc_odr()).unwrap_or_default();

        Ok(val)
    }

    /// Get the output value of all MLC decision trees.
    pub fn mlc_out_get(&mut self) -> Result<MlcOut, Error<B::Error>> {
        let mut arr: [u8; 8] = [0; 8];

        MemBank::operate_over_embed(self, |lock| Mlc1Src::read_more(lock, &mut arr))?;

        Ok(MlcOut::from_le_bytes(arr))
    }

    /// Set the External sensor sensitivity value register for the Machine Learning Core.
    ///
    /// This register corresponds to the conversion value of the external sensor.
    /// The register value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    /// Default value is 0x3C00 (when using an external magnetometer this value
    /// corresponds to 1 gauss/LSB).
    pub fn mlc_ext_sens_sensitivity_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        MlcExtSensitivity::new().with_mlc_ext(val).write(self)
    }

    /// Get the External sensor sensitivity value register for the Machine Learning Core.
    ///
    /// This register corresponds to the conversion value of the external sensor.
    /// The register value is expressed as half-precision floating-point format:
    /// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits).
    /// Default value is 0x3C00 (when using an external magnetometer this value
    /// corresponds to 1 gauss/LSB).
    pub fn mlc_ext_sens_sensitivity_get(&mut self) -> Result<u16, Error<B::Error>> {
        MlcExtSensitivity::read(self).map(|reg| reg.mlc_ext())
    }

    /// Enable/Disable the full control of OIS configurations from the UI (User Interface).
    pub fn ois_ctrl_mode_set(&mut self, val: OisCtrlMode) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::read(self)?;
        func_cfg_access.set_ois_ctrl_from_ui((val as u8) & 0x1);
        func_cfg_access.write(self)
    }

    /// Get the configuration for the full control of OIS configurations from the UI (User Interface).
    pub fn ois_ctrl_mode_get(&mut self) -> Result<OisCtrlMode, Error<B::Error>> {
        let func_cfg_access = FuncCfgAccess::read(self)?;

        let val = func_cfg_access.ois_ctrl_from_ui();
        let val = OisCtrlMode::try_from(val).unwrap_or_default();

        Ok(val)
    }

    /// Resets the control registers of OIS from the UI (User Interface).
    ///
    /// The bits set are not auto-cleared.
    pub fn ois_reset_set(&mut self, val: i8) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::read(self)?;
        func_cfg_access.set_if2_reset(val as u8);
        func_cfg_access.write(self)
    }

    /// Get Resets the control registers of OIS from the UI (User Interface)
    pub fn ois_reset_get(&mut self) -> Result<i8, Error<B::Error>> {
        FuncCfgAccess::read(self).map(|reg| reg.if2_reset() as i8)
    }

    /// Enable/disable pull up on OIS interface.
    pub fn ois_interface_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_ois_pu_dis(val);
        pin_ctrl.write(self)
    }

    /// Get the configuration (enable/disable) pull up on OIS interface.
    pub fn ois_interface_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        PinCtrl::read(self).map(|reg| reg.ois_pu_dis())
    }

    /// Set Handshake for (User Interface) UI / (OIS interface) IF2 shared registers.
    ///
    /// ACK: This bit acknowledges the handshake. If the secondary interface is not accessing the shared registers, this bit is set to 1 by the device and the R/W operation on the UI_IF2_SHARED registers is allowed on the primary interface.
    /// REQ: This bit is used by the primary interface controller to request access to the UI_IF2_SHARED registers. When the R/W operation is finished, the controller must reset this bit.
    pub fn ois_handshake_from_ui_set(&mut self, val: OisHandshake) -> Result<(), Error<B::Error>> {
        let mut ui_handshake_ctrl = UiHandshakeCtrl::read(self)?;
        ui_handshake_ctrl.set_ui_shared_ack(val.ack);
        ui_handshake_ctrl.set_ui_shared_req(val.req);
        ui_handshake_ctrl.write(self)
    }

    /// Get Handshake for (User Interface) UI / (OIS interface) IF2 shared registers.
    ///
    /// ACK: This bit acknowledges the handshake. If the secondary interface is not accessing the shared registers,
    /// this bit is set to 1 by the device and the R/W operation on the UI_IF2_SHARED registers is allowed on the primary interface.
    /// REQ: This bit is used by the primary interface controller to request access to the UI_IF2_SHARED registers.
    /// When the R/W operation is finished, the controller must reset this bit.
    pub fn ois_handshake_from_ui_get(&mut self) -> Result<OisHandshake, Error<B::Error>> {
        let reg = UiHandshakeCtrl::read(self)?;
        let val = OisHandshake {
            ack: reg.ui_shared_ack(),
            req: reg.ui_shared_req(),
        };
        Ok(val)
    }

    /// Set Handshake for (User Interface) UI / (OIS interface) IF2 shared registers.
    ///
    /// ACK: This bit acknowledges the handshake. If the secondary interface is not
    /// accessing the shared registers, this bit is set to 1 by the device and the R/W
    /// operation on the UI_IF2_SHARED registers is allowed on the primary interface.
    /// REQ: This bit is used by the primary interface controller to request access to
    /// the UI_IF2_SHARED registers. When the R/W operation is finished, the controller
    /// must reset this bit.
    pub fn ois_handshake_from_ois_set(&mut self, val: OisHandshake) -> Result<(), Error<B::Error>> {
        let mut if2_handshake_ctrl = If2HandshakeCtrl::read(self)?;
        if2_handshake_ctrl.set_if2_shared_ack(val.ack);
        if2_handshake_ctrl.set_if2_shared_req(val.req);
        if2_handshake_ctrl.write(self)
    }

    /// Get Handshake for (User Interface) UI / (OIS interface) IF2 shared registers.
    ///
    /// ACK: This bit acknowledges the handshake. If the secondary interface is not
    /// accessing the shared registers, this bit is set to 1 by the device and the R/W
    /// operation on the UI_IF2_SHARED registers is allowed on the primary interface.
    /// REQ: This bit is used by the primary interface controller to request access to
    /// the UI_IF2_SHARED registers. When the R/W operation is finished, the controller
    /// must reset this bit.
    pub fn ois_handshake_from_ois_get(&mut self) -> Result<OisHandshake, Error<B::Error>> {
        let if2_handshake_ctrl = If2HandshakeCtrl::read(self)?;
        let val = OisHandshake {
            ack: if2_handshake_ctrl.if2_shared_ack(),
            req: if2_handshake_ctrl.if2_shared_req(),
        };
        Ok(val)
    }

    /// Set User interface (UI) / IF2 (OIS) shared registers
    pub fn ois_shared_set(&mut self, val: [u8; 6]) -> Result<(), Error<B::Error>> {
        self.write_to_register(Reg::UiIf2Shared0 as u8, &val)?;
        Ok(())
    }

    /// Get the content of User interface (UI) / IF2 (OIS) shared registers
    pub fn ois_shared_get(&mut self) -> Result<[u8; 6], Error<B::Error>> {
        let mut val: [u8; 6] = [0; 6];
        self.read_from_register(Reg::UiIf2Shared0 as u8, &mut val)?;
        Ok(val)
    }

    /// In User Interface (UI) full control mode, enables IF2 (OIS Interface) for reading OIS data.
    ///
    /// This function works also on OIS (UI_CTRL1_OIS = IF2_CTRL1_OIS).
    pub fn ois_on_if2_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl1_ois = UiCtrl1Ois::read(self)?;
        ui_ctrl1_ois.set_if2_spi_read_en(val);
        ui_ctrl1_ois.write(self)
    }

    /// Get User Interface (UI) full control mode, enables IF2 (OIS Interface) for reading OIS data.
    ///
    /// This function works also on OIS (UI_CTRL1_OIS = IF2_CTRL1_OIS).
    pub fn ois_on_if2_get(&mut self) -> Result<u8, Error<B::Error>> {
        UiCtrl1Ois::read(self).map(|reg| reg.if2_spi_read_en())
    }

    /// Enables gyroscope/accelerometer OIS chain.
    ///
    /// This function works also on OIS (UI_CTRL1_OIS = IF2_CTRL1_OIS).
    pub fn ois_chain_set(&mut self, val: OisChain) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl1_ois = UiCtrl1Ois::read(self)?;
        ui_ctrl1_ois.set_ois_g_en(val.gy);
        ui_ctrl1_ois.set_ois_xl_en(val.xl);
        ui_ctrl1_ois.write(self)
    }

    /// Get the configuration (enable/disable) gyroscope/accelerometer OIS chain.
    pub fn ois_chain_get(&mut self) -> Result<OisChain, Error<B::Error>> {
        let reg = UiCtrl1Ois::read(self)?;
        let val = OisChain {
            gy: reg.ois_g_en(),
            xl: reg.ois_xl_en(),
        };
        Ok(val)
    }

    /// Set gyroscope OIS full-scale
    pub fn ois_gy_full_scale_set(&mut self, val: OisGyFullScale) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl2_ois = UiCtrl2Ois::read(self)?;
        ui_ctrl2_ois.set_fs_g_ois(val as u8 & 0x07);
        ui_ctrl2_ois.write(self)
    }

    /// Get the gyroscope OIS full-scale selection
    pub fn ois_gy_full_scale_get(&mut self) -> Result<OisGyFullScale, Error<B::Error>> {
        let ui_ctrl2_ois = UiCtrl2Ois::read(self)?;

        let val = OisGyFullScale::try_from(ui_ctrl2_ois.fs_g_ois()).unwrap_or_default();
        Ok(val)
    }

    /// Set accelerometer OIS channel full-scale.
    pub fn ois_xl_full_scale_set(&mut self, val: OisXlFullScale) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl3_ois = UiCtrl3Ois::read(self)?;
        ui_ctrl3_ois.set_fs_xl_ois(val as u8 & 0x3);
        ui_ctrl3_ois.write(self)
    }

    /// Get accelerometer OIS channel full-scale.
    pub fn ois_xl_full_scale_get(&mut self) -> Result<OisXlFullScale, Error<B::Error>> {
        let ui_ctrl3_ois = UiCtrl3Ois::read(self)?;

        let val = OisXlFullScale::try_from(ui_ctrl3_ois.fs_xl_ois()).unwrap_or_default();

        Ok(val)
    }

    /// Set Threshold for 4D/6D function.
    pub fn six_d_threshold_set(&mut self, val: SixdThreshold) -> Result<(), Error<B::Error>> {
        let mut tap_ths_6d = TapThs6d::read(self)?;
        tap_ths_6d.set_sixd_ths((val as u8) & 0x03);
        tap_ths_6d.write(self)
    }

    /// Get Threshold for 4D/6D function.
    pub fn six_d_threshold_get(&mut self) -> Result<SixdThreshold, Error<B::Error>> {
        let tap_ths_6d = TapThs6d::read(self)?;

        let val = SixdThreshold::try_from(tap_ths_6d.sixd_ths()).unwrap_or_default();
        Ok(val)
    }

    /// Enables/Disables 4D orientation detection.
    ///
    /// Z-axis position detection is disabled.
    pub fn four_d_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut tap_ths_6d = TapThs6d::read(self)?;
        tap_ths_6d.set_d4d_en(val);
        tap_ths_6d.write(self)
    }

    /// Get the configuration for 4D orientation detection enable.
    ///
    /// Z-axis position detection is disabled.
    pub fn four_d_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        TapThs6d::read(self).map(|reg| reg.d4d_en())
    }

    /// Set I3C configuration.
    pub fn i3c_config_set(&mut self, val: I3cConfig) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        let mut ctrl5 = Ctrl5::read(self)?;

        pin_ctrl.set_ibhr_por_en((val.rst_mode as u8) & 0x01);
        ctrl5.set_bus_act_sel((val.ibi_time as u8) & 0x03);
        ctrl5.set_if2_ta0_pid(val.if2_ta0_pid & 0x01);

        pin_ctrl.write(self)?;
        ctrl5.write(self)
    }

    /// Get the I3C configuration.
    pub fn i3c_config_get(&mut self) -> Result<I3cConfig, Error<B::Error>> {
        let pin_ctrl = PinCtrl::read(self)?;
        let rst_mode = RstMode::try_from(pin_ctrl.ibhr_por_en()).unwrap_or_default();

        let ctrl5 = Ctrl5::read(self)?;
        let ibi_time = IbiTime::try_from(ctrl5.bus_act_sel()).unwrap_or_default();

        let config = I3cConfig {
            if2_ta0_pid: ctrl5.if2_ta0_pid(),
            rst_mode,
            ibi_time,
        };

        Ok(config)
    }

    /// Enables/Disables Sensor Hub controller I2C pull-up.
    pub fn sh_controller_interface_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut if_cfg = IfCfg::read(self)?;
        if_cfg.set_shub_pu_en(val);
        if_cfg.write(self)
    }

    /// Get the configuration (enable/disable) for Sensor Hub controller I2C pull-up.
    pub fn sh_controller_interface_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        IfCfg::read(self).map(|reg| reg.shub_pu_en())
    }

    /// Get the Sensor hub output registers.
    pub fn sh_read_data_raw_get(&mut self, buf: &mut [u8]) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| SensorHub1::read_more(lock, buf))
    }

    /// Set the number of external sensors to be read by the sensor hub.
    pub fn sh_target_connected_set(
        &mut self,
        val: ShTargetConnected,
    ) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_aux_sens_on((val as u8) & 0x3);
            controller_config.write(lock)
        })
    }

    /// Get the number of external sensors to be read by the sensor hub.
    pub fn sh_target_connected_get(&mut self) -> Result<ShTargetConnected, Error<B::Error>> {
        let controller_config =
            MemBank::operate_over_sensor_hub(self, |lock| ControllerConfig::read(lock))?;

        let aux_sens_on =
            ShTargetConnected::try_from(controller_config.aux_sens_on()).unwrap_or_default();

        Ok(aux_sens_on)
    }

    /// Enables/disables sensor hub I2C controller.
    pub fn sh_controller_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_controller_on(val);
            controller_config.write(lock)
        })
    }

    /// Get the Sensor hub I2C controller configuration.
    pub fn sh_controller_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let reg = ControllerConfig::read(lock)?;
            Ok(reg.controller_on())
        })
    }

    /// Enable/disable I2C interface pass-through mode.
    pub fn sh_pass_through_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_pass_through_mode(val);
            controller_config.write(lock)
        })
    }

    /// Get I2C interface pass-through configuration.
    pub fn sh_pass_through_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let cfg = ControllerConfig::read(lock)?;
            Ok(cfg.pass_through_mode())
        })
    }

    /// Set Sensor hub trigger signal.
    pub fn sh_syncro_mode_set(&mut self, val: ShSyncroMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_start_config((val as u8) & 0x01);
            controller_config.write(lock)
        })
    }

    /// Get the Sensor hub trigger signal configuration.
    pub fn sh_syncro_mode_get(&mut self) -> Result<ShSyncroMode, Error<B::Error>> {
        let controller_config =
            MemBank::operate_over_sensor_hub(self, |lock| ControllerConfig::read(lock))?;

        let val = ShSyncroMode::try_from(controller_config.start_config()).unwrap_or_default();
        Ok(val)
    }

    /// Enable/Disable Target 0 write operation write_once.
    ///
    /// If enabled the write is performed only at the first sensor hub cycle.
    pub fn sh_write_mode_set(&mut self, val: ShWriteMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_write_once((val as u8) & 0x01);
            controller_config.write(lock)
        })
    }

    /// Get the current Target 0 write operation write_once.
    ///
    /// If enabled the write is performed only at the first sensor hub cycle.
    pub fn sh_write_mode_get(&mut self) -> Result<ShWriteMode, Error<B::Error>> {
        let controller_config =
            MemBank::operate_over_sensor_hub(self, |lock| ControllerConfig::read(lock))?;

        let mode = ShWriteMode::try_from(controller_config.write_once()).unwrap_or_default();
        Ok(mode)
    }

    /// Reset Controller logic and output registers.
    ///
    /// Must be set to '1' and then set it to '0'.
    pub fn sh_reset_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut controller_config = ControllerConfig::read(lock)?;
            controller_config.set_rst_controller_regs(val);
            controller_config.write(lock)
        })
    }

    /// Get the reset Controller logic and output registers.
    ///
    /// Must be set to '1' and then set it to '0'.
    pub fn sh_reset_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            ControllerConfig::read(lock).map(|reg| reg.rst_controller_regs())
        })
    }

    /// Configure target 0 for perform a write.
    ///
    /// # Arguments
    ///
    /// * `val`: A structure that contains:
    ///     - `tgt0_add`: 8-bit I2C device address
    ///     - `tgt0_subadd`: 8-bit register device address
    ///     - `tgt0_data`: 8-bit data to write
    pub fn sh_cfg_write(&mut self, val: ShCfgWrite) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |state| {
            let mut reg = Tgt0Add::new();
            reg.set_target0_add(val.tgt0_add);
            reg.set_rw_0(0);
            reg.write(state)?;

            Tgt0Subadd::from_bits(val.tgt0_subadd).write(state)?;
            DatawriteTgt0::from_bits(val.tgt0_data).write(state)
        })
    }

    /// Set the rate at which the controller communicates.
    pub fn sh_data_rate_set(&mut self, val: ShDataRate) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut tgt0_config = Tgt0Config::read(lock)?;
            tgt0_config.set_shub_odr(val as u8 & 0x07);
            tgt0_config.write(lock)
        })
    }

    /// Get the rate at which the controller communicates.
    pub fn sh_data_rate_get(&mut self) -> Result<ShDataRate, Error<B::Error>> {
        let shub_odr = MemBank::operate_over_sensor_hub(self, |lock| {
            let shub_odr = Tgt0Config::read(lock)?;
            Ok(shub_odr.shub_odr())
        })?;

        let val = ShDataRate::try_from(shub_odr).unwrap_or_default();
        Ok(val)
    }

    /// Configure target idx for perform a read.
    ///
    /// # Arguments
    ///
    /// * `idx`: Index to configure.
    /// * `val`: Structure that contains:
    ///     - `tgt_add`: 8-bit I2C device address.
    ///     - `tgt_subadd`: 8-bit register device address.
    ///     - `tgt_len`: Number of bits to read.
    pub fn sh_tgt_cfg_read(&mut self, idx: u8, val: ShCfgRead) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |state| {
            let mut tgt_add = Tgt0Add::new();
            tgt_add.set_target0_add(val.tgt_add);
            tgt_add.set_rw_0(1);
            state.write_to_register(SensHubReg::Tgt0Add as u8 + idx * 3, &[tgt_add.into_bits()])?;

            state.write_to_register(SensHubReg::Tgt0Subadd as u8 + idx * 3, &[val.tgt_subadd])?;

            let mut arr: [u8; 1] = [0];
            state.read_from_register(SensHubReg::Tgt0Config as u8 + idx * 3, &mut arr)?;
            let mut tgt_config = Tgt0Config::from_bits(arr[0]);
            tgt_config.set_target0_numop(val.tgt_len);

            state.write_to_register(
                SensHubReg::Tgt0Config as u8 + idx * 3,
                &[tgt_config.into_bits()],
            )
        })
    }

    /// Get Sensor hub status register.
    pub fn sh_status_get(&mut self) -> Result<StatusControllerMainpage, Error<B::Error>> {
        StatusControllerMainpage::read(self)
    }

    /// Enables/Disables pull-up on SDO pin of UI (User Interface).
    pub fn ui_sdo_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_sdo_pu_en(val);
        pin_ctrl.write(self)
    }

    /// Get the pull-up on SDO pin of UI (User Interface) configuration.
    pub fn ui_sdo_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        PinCtrl::read(self).map(|reg| reg.sdo_pu_en())
    }

    /// Set Pad strength.
    pub fn pad_strength_set(&mut self, val: PadStrength) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_io_pad_strength(val as u8);
        pin_ctrl.write(self)
    }

    /// Get the Pad strength.
    pub fn pad_strength_get(&mut self) -> Result<PadStrength, Error<B::Error>> {
        let pin_ctrl = PinCtrl::read(self)?;

        let val = PadStrength::try_from(pin_ctrl.io_pad_strength()).unwrap_or_default();
        Ok(val)
    }

    /// Enables/Disables I2C and I3C on UI (User Interface).
    pub fn ui_i2c_i3c_mode_set(&mut self, val: UiI2cI3cMode) -> Result<(), Error<B::Error>> {
        let mut if_cfg = IfCfg::read(self)?;
        if_cfg.set_i2c_i3c_disable(val as u8 & 0x1);
        if_cfg.write(self)
    }

    /// Get I2C and I3C on UI (User Interface) mode configuration.
    pub fn ui_i2c_i3c_mode_get(&mut self) -> Result<UiI2cI3cMode, Error<B::Error>> {
        let if_cfg = IfCfg::read(self)?;

        let val = UiI2cI3cMode::try_from(if_cfg.i2c_i3c_disable()).unwrap_or_default();
        Ok(val)
    }

    /// SPI Serial Interface Mode selection.
    pub fn spi_mode_set(&mut self, val: SpiMode) -> Result<(), Error<B::Error>> {
        let mut if_cfg = IfCfg::read(self)?;
        if_cfg.set_sim((val as u8) & 0x01);
        if_cfg.write(self)
    }

    /// Get the SPI Serial Interface Mode.
    pub fn spi_mode_get(&mut self) -> Result<SpiMode, Error<B::Error>> {
        let if_cfg = IfCfg::read(self)?;

        let val = SpiMode::try_from(if_cfg.sim()).unwrap_or_default();
        Ok(val)
    }

    /// Enables/Disables pull-up on SDA pin.
    pub fn ui_sda_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut if_cfg = IfCfg::read(self)?;
        if_cfg.set_sda_pu_en(val);
        if_cfg.write(self)
    }

    /// Get pull-up configuration on SDA pin.
    pub fn ui_sda_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        IfCfg::read(self).map(|reg| reg.sda_pu_en())
    }

    /// Set IF2 (OIS Interface) Serial Interface Mode.
    ///
    /// This function works also on OIS (UI_CTRL1_OIS = IF2_CTRL1_OIS).
    pub fn if2_spi_mode_set(&mut self, val: SpiMode) -> Result<(), Error<B::Error>> {
        let mut ui_ctrl1_ois = UiCtrl1Ois::read(self)?;
        ui_ctrl1_ois.set_sim_ois((val as u8) & 0x01);
        ui_ctrl1_ois.write(self)
    }

    /// Get IF2 (OIS Interface) Serial Interface Mode.
    ///
    /// This function works also on OIS (UI_CTRL1_OIS = IF2_CTRL1_OIS).
    pub fn if2_spi_mode_get(&mut self) -> Result<SpiMode, Error<B::Error>> {
        let ui_ctrl1_ois = UiCtrl1Ois::read(self)?;

        let val = SpiMode::try_from(ui_ctrl1_ois.sim_ois()).unwrap_or_default();
        Ok(val)
    }

    /// Enables/Disables significant motion detection function.
    pub fn sigmot_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_a = EmbFuncEnA::read(lock)?;
            emb_func_en_a.set_sign_motion_en(val);
            emb_func_en_a.write(lock)
        })
    }

    /// Get the significant motion detection configuration.
    pub fn sigmot_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncEnA::read(lock).map(|reg| reg.sign_motion_en())
        })
    }

    /// Set step counter mode
    pub fn stpcnt_mode_set(&mut self, val: StpcntMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_a = EmbFuncEnA::read(lock)?;
            let emb_func_en_b = EmbFuncEnB::read(lock)?;

            if val.false_step_rej == 1
                && (emb_func_en_a.mlc_before_fsm_en() & emb_func_en_b.mlc_en()) == 0
            {
                emb_func_en_a.set_mlc_before_fsm_en(1);
            }

            emb_func_en_a.set_pedo_en(val.step_counter_enable);
            emb_func_en_a.write(lock)
        })?;

        let mut pedo_cmd = PedoCmdReg::read(self)?;
        pedo_cmd.set_fp_rejection_en(val.false_step_rej);
        pedo_cmd.write(self)
    }

    /// Get step counter mode
    pub fn stpcnt_mode_get(&mut self) -> Result<StpcntMode, Error<B::Error>> {
        let emb_func_en_a = MemBank::operate_over_embed(self, |lock| EmbFuncEnA::read(lock))?;

        let pedo_cmd_reg = PedoCmdReg::read(self)?;

        let mode = StpcntMode {
            false_step_rej: pedo_cmd_reg.fp_rejection_en(),
            step_counter_enable: emb_func_en_a.pedo_en(),
        };

        Ok(mode)
    }

    /// Get step counter output: number of detected steps.
    pub fn stpcnt_steps_get(&mut self) -> Result<u16, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| StepCounter::read(lock).map(|step| step.0))
    }

    /// Reset step counter.
    ///
    /// If val 1 step is resetted
    pub fn stpcnt_rst_step_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_src = EmbFuncSrc::read(lock)?;
            emb_func_src.set_pedo_rst_step(val);
            emb_func_src.write(lock)
        })
    }

    /// Get reset step counter configuration.
    pub fn stpcnt_rst_step_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncSrc::read(lock).map(|reg| reg.pedo_rst_step())
        })
    }

    /// Set Pedometer debounce number.
    pub fn stpcnt_debounce_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut pedo_deb_steps_conf = PedoDebStepsConf::read(self)?;
        pedo_deb_steps_conf.set_deb_step(val);
        pedo_deb_steps_conf.write(self)
    }

    /// Get Pedometer debounce number.
    pub fn stpcnt_debounce_get(&mut self) -> Result<u8, Error<B::Error>> {
        let pedo_deb_steps_conf = PedoDebStepsConf::read(self)?;
        Ok(pedo_deb_steps_conf.deb_step())
    }

    /// Set time period register for step detection on delta time.
    pub fn stpcnt_period_set(&mut self, val: u16) -> Result<(), Error<B::Error>> {
        PedoScDeltat(val).write(self)
    }

    /// Get time period register for step detection on delta time.
    pub fn stpcnt_period_get(&mut self) -> Result<u16, Error<B::Error>> {
        PedoScDeltat::read(self).map(|arr| arr.0)
    }

    /// Enables/Disables SFLP Game Rotation Vector (6x).
    pub fn sflp_game_rotation_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_a = EmbFuncEnA::read(lock)?;
            emb_func_en_a.set_sflp_game_en(val);
            emb_func_en_a.write(lock)
        })
    }

    /// Get the configuration (enable/disable) for SFLP Game Rotation Vector (6x).
    pub fn sflp_game_rotation_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            EmbFuncEnA::read(lock).map(|reg| reg.sflp_game_en())
        })
    }

    /// Set SFLP Data Rate (ODR).
    pub fn sflp_data_rate_set(&mut self, val: SflpDataRate) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut sflp_odr = SflpOdr::read(lock)?;
            sflp_odr.set_sflp_game_odr((val as u8) & 0x07);
            sflp_odr.write(lock)
        })
    }

    /// Get SFLP Data Rate (ODR).
    pub fn sflp_data_rate_get(&mut self) -> Result<SflpDataRate, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let reg = SflpOdr::read(lock)?;

            Ok(SflpDataRate::try_from(reg.sflp_game_odr()).unwrap_or_default())
        })
    }

    /// Enable axis for Tap - Double Tap detection.
    pub fn tap_detection_set(&mut self, val: TapDetection) -> Result<(), Error<B::Error>> {
        let mut tap_cfg0 = TapCfg0::read(self)?;
        tap_cfg0.set_tap_x_en(val.tap_x_en);
        tap_cfg0.set_tap_y_en(val.tap_y_en);
        tap_cfg0.set_tap_z_en(val.tap_z_en);
        tap_cfg0.write(self)
    }

    /// Get configuration for Tap on each axis - Double Tap detection.
    pub fn tap_detection_get(&mut self) -> Result<TapDetection, Error<B::Error>> {
        let tap_cfg0 = TapCfg0::read(self)?;

        let val = TapDetection {
            tap_x_en: tap_cfg0.tap_x_en(),
            tap_y_en: tap_cfg0.tap_y_en(),
            tap_z_en: tap_cfg0.tap_z_en(),
        };

        Ok(val)
    }

    /// Set Double Tap recognition thresholds - axis Tap
    pub fn tap_thresholds_set(&mut self, val: TapThresholds) -> Result<(), Error<B::Error>> {
        let mut tap_cfg1 = TapCfg1::read(self)?;
        let mut tap_cfg2 = TapCfg2::read(self)?;
        let mut tap_ths_6d = TapThs6d::read(self)?;

        tap_cfg1.set_tap_ths_x(val.x);
        tap_cfg2.set_tap_ths_y(val.y);
        tap_ths_6d.set_tap_ths_z(val.z);

        tap_ths_6d.write(self)?;
        tap_cfg2.write(self)?;
        tap_cfg1.write(self)?;

        Ok(())
    }

    /// Get Double Tap recognition thresholds - axis Tap
    pub fn tap_thresholds_get(&mut self) -> Result<TapThresholds, Error<B::Error>> {
        let tap_cfg1 = TapCfg1::read(self)?;
        let tap_cfg2 = TapCfg2::read(self)?;
        let tap_ths_6d = TapThs6d::read(self)?;

        let val = TapThresholds {
            x: tap_cfg1.tap_ths_x(),
            y: tap_cfg2.tap_ths_y(),
            z: tap_ths_6d.tap_ths_z(),
        };

        Ok(val)
    }

    /// Set axis priority for TAP detection.
    pub fn tap_axis_priority_set(&mut self, val: TapAxisPriority) -> Result<(), Error<B::Error>> {
        let mut tap_cfg1 = TapCfg1::read(self)?;
        tap_cfg1.set_tap_priority((val as u8) & 0x7);
        tap_cfg1.write(self)
    }

    /// Get axis priority for TAP detection.
    pub fn tap_axis_priority_get(&mut self) -> Result<TapAxisPriority, Error<B::Error>> {
        let tap_cfg1 = TapCfg1::read(self)?;
        let val = TapAxisPriority::try_from(tap_cfg1.tap_priority()).unwrap_or_default();

        Ok(val)
    }

    /// Set Time windows configuration for Tap - Double Tap SHOCK, QUIET, DUR.
    ///
    /// SHOCK Maximum duration is the maximum time of an overthreshold signal detection
    /// to be recognized as a tap event. The default value of these bits is 00b which
    /// corresponds to 4/ODR_XL time. If the SHOCK bits are set to a different value,
    /// 1LSB corresponds to 8/ODR_XL time. QUIET Expected quiet time after a tap detection.
    /// Quiet time is the time after the first detected tap in which there must not be any
    /// overthreshold event. The default value of these bits is 00b which corresponds to
    /// 2/ODR_XL time. If the QUIET bits are set to a different value, 1LSB corresponds to
    /// 4/ODR_XL time. DUR Duration of maximum time gap for double tap recognition.
    /// When double tap recognition is enabled, this register expresses the maximum time
    /// between two consecutive detected taps to determine a double tap event. The default
    /// value of these bits is 0000b which corresponds to 16/ODR_XL time. If the DUR_\[3:0\]
    /// bits are set to a different value, 1LSB corresponds to 32/ODR_XL time.
    pub fn tap_time_windows_set(&mut self, val: TapTimeWindows) -> Result<(), Error<B::Error>> {
        let mut tap_dur = TapDur::read(self)?;
        tap_dur.set_shock(val.shock);
        tap_dur.set_quiet(val.quiet);
        tap_dur.set_dur(val.tap_gap);
        tap_dur.write(self)
    }

    /// Get Time windows configuration for Tap - Double Tap SHOCK, QUIET, DUR.
    ///
    /// SHOCK Maximum duration is the maximum time of an overthreshold signal detection to be
    /// recognized as a tap event.
    /// QUIET Expected quiet time after a tap detection.
    /// DUR Duration of maximum time gap for double tap recognition.
    pub fn tap_time_windows_get(&mut self) -> Result<TapTimeWindows, Error<B::Error>> {
        let tap_dur = TapDur::read(self)?;

        Ok(TapTimeWindows {
            shock: tap_dur.shock(),
            quiet: tap_dur.quiet(),
            tap_gap: tap_dur.dur(),
        })
    }

    /// Enable/Disable single/double-tap event.
    pub fn tap_mode_set(&mut self, val: TapMode) -> Result<(), Error<B::Error>> {
        let mut wake_up_ths = WakeUpThs::read(self)?;
        wake_up_ths.set_single_double_tap((val as u8) & 0x01);
        wake_up_ths.write(self)
    }

    /// Get configuration (enable/disable) for Single/double-tap event
    pub fn tap_mode_get(&mut self) -> Result<TapMode, Error<B::Error>> {
        let wake_up_ths = WakeUpThs::read(self)?;

        let val = TapMode::try_from(wake_up_ths.single_double_tap()).unwrap_or_default();
        Ok(val)
    }

    /// Set Tilt mode.
    pub fn tilt_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let mut emb_func_en_a = EmbFuncEnA::read(lock)?;
            emb_func_en_a.set_tilt_en(val);
            emb_func_en_a.write(lock)
        })
    }

    /// Get Tilt mode.
    pub fn tilt_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_embed(self, |lock| {
            let reg = EmbFuncEnA::read(lock)?;
            Ok(reg.tilt_en())
        })
    }

    /// Get Timestamp raw data
    pub fn timestamp_raw_get(&mut self) -> Result<u32, Error<B::Error>> {
        Timestamp::read(self).map(|time| time.0)
    }

    /// Enables timestamp counter.
    pub fn timestamp_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut functions_enable = FunctionsEnable::read(self)?;
        functions_enable.set_timestamp_en(val);
        functions_enable.write(self)
    }

    /// Get the actual timestamp counter configuration.
    ///
    /// If return 1 timestamp counter is active
    pub fn timestamp_get(&mut self) -> Result<u8, Error<B::Error>> {
        FunctionsEnable::read(self).map(|reg| reg.timestamp_en())
    }

    /// Configure activity/inactivity (sleep)
    ///
    /// `ActMode` could handle different setting for accelerometer and gyroscope
    pub fn act_mode_set(&mut self, val: ActMode) -> Result<(), Error<B::Error>> {
        let mut functions_enable = FunctionsEnable::read(self)?;
        functions_enable.set_inact_en((val as u8) & 0x3);
        functions_enable.write(self)
    }

    /// Get activity/inactivity (sleep)
    pub fn act_mode_get(&mut self) -> Result<ActMode, Error<B::Error>> {
        let functions_enable = FunctionsEnable::read(self)?;
        let val = ActMode::try_from(functions_enable.inact_en()).unwrap_or_default();

        Ok(val)
    }

    /// Set duration in the transition from Stationary to Motion (from Inactivity to Activity).
    pub fn act_from_sleep_to_act_dur_set(
        &mut self,
        val: ActFromSleepToActDur,
    ) -> Result<(), Error<B::Error>> {
        let mut inactivity_dur = InactivityDur::read(self)?;
        inactivity_dur.set_inact_dur((val as u8) & 0x3);
        inactivity_dur.write(self)
    }

    /// Get duration in the transition from Stationary to Motion (from Inactivity to Activity).
    pub fn act_from_sleep_to_act_dur_get(
        &mut self,
    ) -> Result<ActFromSleepToActDur, Error<B::Error>> {
        let inactivity_dur = InactivityDur::read(self)?;
        let val = ActFromSleepToActDur::try_from(inactivity_dur.inact_dur()).unwrap_or_default();

        Ok(val)
    }

    /// Set the accelerometer data rate during Inactivity.
    pub fn act_sleep_xl_odr_set(&mut self, val: ActSleepXlOdr) -> Result<(), Error<B::Error>> {
        let mut inactivity_dur = InactivityDur::read(self)?;
        inactivity_dur.set_xl_inact_odr((val as u8) & 0x03);
        inactivity_dur.write(self)
    }

    /// Get the accelerometer data rate during Inactivity.
    pub fn act_sleep_xl_odr_get(&mut self) -> Result<ActSleepXlOdr, Error<B::Error>> {
        let inactivity_dur = InactivityDur::read(self)?;
        let val = ActSleepXlOdr::try_from(inactivity_dur.xl_inact_odr()).unwrap_or_default();

        Ok(val)
    }

    /// Set Wakeup and activity/inactivity threshold.
    pub fn act_thresholds_set(&mut self, val: &ActThresholds) -> Result<(), Error<B::Error>> {
        let mut inactivity_dur = InactivityDur::read(self)?;
        let mut inactivity_ths = InactivityThs::read(self)?;
        let mut wake_up_ths = WakeUpThs::read(self)?;
        let mut wake_up_dur = WakeUpDur::read(self)?;

        inactivity_dur.set_wu_inact_ths_w(val.inactivity_cfg.wu_inact_ths_w());
        inactivity_dur.set_xl_inact_odr(val.inactivity_cfg.xl_inact_odr());
        inactivity_dur.set_inact_dur(val.inactivity_cfg.inact_dur());

        inactivity_ths.set_inact_ths(val.inactivity_ths);
        wake_up_ths.set_wk_ths(val.threshold);
        wake_up_dur.set_wake_dur(val.duration);

        inactivity_dur.write(self)?;
        inactivity_ths.write(self)?;
        wake_up_ths.write(self)?;
        wake_up_dur.write(self)?;

        Ok(())
    }

    /// Get Wakeup and activity/inactivity threshold.
    pub fn act_thresholds_get(&mut self) -> Result<ActThresholds, Error<B::Error>> {
        let inactivity_cfg = InactivityDur::read(self)?;
        let inactivity_ths = InactivityThs::read(self)?.inact_ths();
        let threshold = WakeUpThs::read(self)?.wk_ths();
        let duration = WakeUpDur::read(self)?.wake_dur();

        Ok(ActThresholds {
            inactivity_cfg,
            inactivity_ths,
            threshold,
            duration,
        })
    }

    /// Set Time windows for Wake Up - Activity - Inactivity (SLEEP, WAKE).
    ///
    /// Duration to go in sleep mode. Default value: 0000 (this corresponds to 16 ODR)
    /// 1 LSB = 512/ODR_XL time. Wake up duration event. 1 LSB = 1/ODR_XL time.
    pub fn act_wkup_time_windows_set(
        &mut self,
        val: ActWkupTimeWindows,
    ) -> Result<(), Error<B::Error>> {
        let mut wake_up_dur = WakeUpDur::read(self)?;
        wake_up_dur.set_wake_dur(val.shock);
        wake_up_dur.set_sleep_dur(val.quiet);
        wake_up_dur.write(self)
    }

    /// Get Time windows configuration for Wake Up - Activity - Inactivity (SLEEP, WAKE).
    ///
    /// Duration to go in sleep mode. Default value: 0000 (this corresponds to 16 ODR)
    /// 1 LSB = 512/ODR_XL time. Wake up duration event. 1 LSB = 1/ODR_XL time.
    pub fn act_wkup_time_windows_get(&mut self) -> Result<ActWkupTimeWindows, Error<B::Error>> {
        let wake_up_dur = WakeUpDur::read(self)?;

        Ok(ActWkupTimeWindows {
            shock: wake_up_dur.wake_dur(),
            quiet: wake_up_dur.sleep_dur(),
        })
    }
}

pub fn npy_half_to_float(bytes: u16) -> f32 {
    let half_float = f16::from_bits(bytes);
    f32::from(half_float)
}

pub fn npy_float_to_half(f: f32) -> u16 {
    //let fbits: u32 = f.to_bits();
    //self.npy_floatbits_to_halfbits(bits)
    let half_float = f16::from_f32(f);
    half_float.to_bits()
}

/// Converts from SFLP to mg.
pub fn from_sflp_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.061
}

/// Convert the given LSB value to milligrams.
pub fn from_fs2_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.061
}

/// Convert LSB to mg.
pub fn from_fs4_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.122
}

/// Converts LSB value to milligravity (mg).
pub fn from_fs8_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.244
}

/// Converts a 16-bit integer to milligrams.
pub fn from_fs16_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.488
}

/// Convert from full-scale 32 to milligrams.
pub fn from_fs32_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.976
}

/// Converts a 16-bit LSB value to milligravity (mg).
pub fn from_fs64_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 1.952
}

/// Converts the given lsb to milligrams using the FS128 scale.
pub fn from_fs128_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 3.904
}

/// Converts LSB to milligrams using full scale 256.
pub fn from_fs256_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 7.808f32
}

/// Converts the LSB value to milligrams.
pub fn from_fs320_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 10.417
}

/// Converts the given LSB value to millidegrees per second.
pub fn from_fs250_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 8.750
}

pub fn from_fs500_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 17.50
}

/// Converts the given LSB to milli-degrees per second (mdps).
pub fn from_fs1000_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 35.0
}

/// Converts lsb to mdps for fs2000.
pub fn from_fs2000_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 70.0
}

/// Converts a 16-bit signed integer from full-scale 4000 to millidegrees per second.
pub fn from_fs4000_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 140.0
}

/// Convert LSB to Celsius temperature.
pub fn from_lsb_to_celsius(lsb: i16) -> f32 {
    ((lsb as f32) / 256.0) + 25.0
}

/// Converts a value from LSB to nanoseconds.
pub fn from_lsb_to_nsec(lsb: u32) -> u64 {
    (lsb as u64) * 21700
}

pub fn from_lsb_to_mv(lsb: i16) -> f32 {
    (lsb as f32) / 78.0
}

/// Converts gyroscope bias from LSB to mdps.
pub fn from_gbias_lsb_to_mdps(lsb: i16) -> f32 {
    lsb as f32 * 4.375
}

/// Convert gravity from LSB to mg.
pub fn from_gravity_lsb_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.061
}

/// Converts quaternion LSB to float.
pub fn from_quaternion_lsb_to_float(lsb: u16) -> f32 {
    crate::npy_half_to_float(lsb)
}

#[derive(Default)]
pub struct AllSources {
    pub drdy_xl: u8,
    pub drdy_gy: u8,
    pub drdy_temp: u8,
    pub drdy_xlhgda: u8,
    pub drdy_eis: u8,
    pub drdy_ois: u8,
    pub gy_settling: u8,
    pub timestamp: u8,
    pub hg: u8,
    pub free_fall: u8,
    pub wake_up: u8,
    pub wake_up_z: u8,
    pub wake_up_y: u8,
    pub wake_up_x: u8,
    pub single_tap: u8,
    pub double_tap: u8,
    pub tap_z: u8,
    pub tap_y: u8,
    pub tap_x: u8,
    pub tap_sign: u8,
    pub six_d: u8,
    pub six_d_xl: u8,
    pub six_d_xh: u8,
    pub six_d_yl: u8,
    pub six_d_yh: u8,
    pub six_d_zl: u8,
    pub six_d_zh: u8,
    pub sleep_change: u8,
    pub sleep_state: u8,
    pub step_detector: u8,
    pub step_count_inc: u8,
    pub step_count_overflow: u8,
    pub step_on_delta_time: u8,
    pub emb_func_stand_by: u8,
    pub emb_func_time_exceed: u8,
    pub tilt: u8,
    pub sig_mot: u8,
    pub fsm_lc: u8,
    pub fsm1: u8,
    pub fsm2: u8,
    pub fsm3: u8,
    pub fsm4: u8,
    pub fsm5: u8,
    pub fsm6: u8,
    pub fsm7: u8,
    pub fsm8: u8,
    pub mlc1: u8,
    pub mlc2: u8,
    pub mlc3: u8,
    pub mlc4: u8,
    pub mlc5: u8,
    pub mlc6: u8,
    pub mlc7: u8,
    pub mlc8: u8,
    pub sh_endop: u8,
    pub sh_target0_nack: u8,
    pub sh_target1_nack: u8,
    pub sh_target2_nack: u8,
    pub sh_target3_nack: u8,
    pub sh_wr_once: u8,
    pub fifo_bdr: u8,
    pub fifo_full: u8,
    pub fifo_ovr: u8,
    pub fifo_th: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum I2CAddress {
    I2cAddL = 0x6a,
    I2cAddH = 0x6b,
}

#[allow(dead_code)]
pub const ID: u8 = 0x73;

pub const CHUNK_SIZE: usize = 256;
