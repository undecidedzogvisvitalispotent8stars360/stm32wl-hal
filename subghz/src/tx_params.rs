/// Power amplifier ramp time for FSK, MSK, and LoRa modulation.
///
/// Argument of [`set_ramp_time`][`crate::TxParams::set_ramp_time`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RampTime {
    /// 10µs
    Micros10 = 0x00,
    /// 20µs
    Micros20 = 0x01,
    /// 40µs
    Micros40 = 0x02,
    /// 80µs
    Micros80 = 0x03,
    /// 200µs
    Micros200 = 0x04,
    /// 800µs
    Micros800 = 0x05,
    /// 1.7ms
    Micros1700 = 0x06,
    /// 3.4ms
    Micros3400 = 0x07,
}

impl From<RampTime> for u8 {
    fn from(rt: RampTime) -> Self {
        rt as u8
    }
}

impl From<RampTime> for core::time::Duration {
    fn from(rt: RampTime) -> Self {
        match rt {
            RampTime::Micros10 => core::time::Duration::from_micros(10),
            RampTime::Micros20 => core::time::Duration::from_micros(20),
            RampTime::Micros40 => core::time::Duration::from_micros(40),
            RampTime::Micros80 => core::time::Duration::from_micros(80),
            RampTime::Micros200 => core::time::Duration::from_micros(200),
            RampTime::Micros800 => core::time::Duration::from_micros(800),
            RampTime::Micros1700 => core::time::Duration::from_micros(1700),
            RampTime::Micros3400 => core::time::Duration::from_micros(3400),
        }
    }
}

/// Transmit parameters, output power and power amplifier ramp up time.
///
/// Argument of [`set_tx_params`][`crate::SubGhz::set_tx_params`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TxParams {
    buf: [u8; 3],
}

impl TxParams {
    /// Create a new `TxParams` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::TxParams;
    ///
    /// const TX_PARAMS: TxParams = TxParams::new();
    /// assert_eq!(TX_PARAMS, TxParams::default());
    /// ```
    pub const fn new() -> TxParams {
        TxParams {
            buf: [crate::OpCode::SetTxParams as u8, 0x00, 0x00],
        }
    }

    /// Set the output power.
    ///
    /// For low power selected in [`set_pa_config`]:
    ///
    /// * 0x0E: +14 dB
    /// * ...
    /// * 0x00: 0 dB
    /// * ...
    /// * 0xEF: -17 dB
    /// * Others: reserved
    ///
    /// For high power selected in [`set_pa_config`]:
    ///
    /// * 0x16: +22 dB
    /// * ...
    /// * 0x00: 0 dB
    /// * ...
    /// * 0xF7: -9 dB
    /// * Others: reserved
    ///
    /// # Example
    ///
    /// Set the output power to 0 dB.
    ///
    /// ```
    /// use stm32wl_hal_subghz::{RampTime, TxParams};
    ///
    /// const TX_PARAMS: TxParams = TxParams::new().set_power(0x00);
    /// # assert_eq!(TX_PARAMS.as_slice()[1], 0x00);
    /// ```
    ///
    /// [`set_pa_config`]: crate::SubGhz::set_pa_config
    #[must_use = "set_power returns a modified TxParams"]
    pub const fn set_power(mut self, power: u8) -> TxParams {
        self.buf[1] = power;
        self
    }

    /// Set the Power amplifier ramp time for FSK, MSK, and LoRa modulation.
    ///
    /// # Example
    ///
    /// Set the ramp time to 200 microseconds.
    ///
    /// ```
    /// use stm32wl_hal_subghz::{RampTime, TxParams};
    ///
    /// const TX_PARAMS: TxParams = TxParams::new().set_ramp_time(RampTime::Micros200);
    /// # assert_eq!(TX_PARAMS.as_slice()[2], 0x04);
    /// ```
    #[must_use = "set_ramp_time returns a modified TxParams"]
    pub const fn set_ramp_time(mut self, rt: RampTime) -> TxParams {
        self.buf[2] = rt as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{RampTime, TxParams};
    ///
    /// const TX_PARAMS: TxParams = TxParams::new()
    ///     .set_ramp_time(RampTime::Micros80)
    ///     .set_power(0x0E);
    /// assert_eq!(TX_PARAMS.as_slice(), &[0x8E, 0x0E, 0x03]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for TxParams {
    fn default() -> Self {
        Self::new()
    }
}
