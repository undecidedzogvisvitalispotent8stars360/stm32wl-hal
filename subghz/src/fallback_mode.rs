/// Fallback mode after successful packet transmission or packet reception.
///
/// Argument of [`set_tx_rx_fallback_mode`].
///
/// [`set_tx_rx_fallback_mode`]: crate::SubGhz::set_tx_rx_fallback_mode.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum FallbackMode {
    /// Standby mode entry.
    Standby = 0x20,
    /// Standby with HSE32 enabled.
    StandbyHse32 = 0x30,
    /// Frequency synthesizer entry.
    Fs = 0x40,
}

impl From<FallbackMode> for u8 {
    fn from(fm: FallbackMode) -> Self {
        fm as u8
    }
}

impl Default for FallbackMode {
    /// Default fallback mode after power-on reset.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FallbackMode;
    ///
    /// assert_eq!(FallbackMode::default(), FallbackMode::Standby);
    /// ```
    fn default() -> Self {
        FallbackMode::Standby
    }
}
