/// Bandwidth options for [`FskModParams`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FskBandwidth {
    /// 4.8 kHz DSB
    Bw4 = 0x1F,
    /// 5.8 kHz DSB
    Bw5 = 0x17,
    /// 7.3 kHz DSB
    Bw7 = 0x0F,
    /// 9.7 kHz DSB
    Bw9 = 0x1E,
    /// 11.7 kHz DSB
    Bw11 = 0x16,
    /// 14.6 kHz DSB
    Bw14 = 0x0E,
    /// 19.5 kHz DSB
    Bw19 = 0x1D,
    /// 23.4 kHz DSB
    Bw23 = 0x15,
    /// 29.3 kHz DSB
    Bw29 = 0x0D,
    /// 39.0 kHz DSB
    Bw39 = 0x1C,
    /// 46.9 kHz DSB
    Bw46 = 0x14,
    /// 58.6 kHz DSB
    Bw58 = 0x0C,
    /// 78.2 kHz DSB
    Bw78 = 0x1B,
    /// 93.8 kHz DSB
    Bw93 = 0x13,
    /// 117.3 kHz DSB
    Bw117 = 0x0B,
    /// 156.2 kHz DSB
    Bw156 = 0x1A,
    /// 187.2 kHz DSB
    Bw187 = 0x12,
    /// 234.3 kHz DSB
    Bw234 = 0x0A,
    /// 312.0 kHz DSB
    Bw312 = 0x19,
    /// 373.6 kHz DSB
    Bw373 = 0x11,
    /// 467.0 kHz DSB
    Bw467 = 0x09,
}

impl FskBandwidth {
    /// Get the bandwidth in hertz.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskBandwidth;
    ///
    /// assert_eq!(FskBandwidth::Bw4.hertz(), 4_800);
    /// assert_eq!(FskBandwidth::Bw5.hertz(), 5_800);
    /// assert_eq!(FskBandwidth::Bw7.hertz(), 7_300);
    /// assert_eq!(FskBandwidth::Bw9.hertz(), 9_700);
    /// assert_eq!(FskBandwidth::Bw11.hertz(), 11_700);
    /// assert_eq!(FskBandwidth::Bw14.hertz(), 14_600);
    /// assert_eq!(FskBandwidth::Bw19.hertz(), 19_500);
    /// assert_eq!(FskBandwidth::Bw23.hertz(), 23_400);
    /// assert_eq!(FskBandwidth::Bw29.hertz(), 29_300);
    /// assert_eq!(FskBandwidth::Bw39.hertz(), 39_000);
    /// assert_eq!(FskBandwidth::Bw46.hertz(), 46_900);
    /// assert_eq!(FskBandwidth::Bw58.hertz(), 58_600);
    /// assert_eq!(FskBandwidth::Bw78.hertz(), 78_200);
    /// assert_eq!(FskBandwidth::Bw93.hertz(), 93_800);
    /// assert_eq!(FskBandwidth::Bw117.hertz(), 117_300);
    /// assert_eq!(FskBandwidth::Bw156.hertz(), 156_200);
    /// assert_eq!(FskBandwidth::Bw187.hertz(), 187_200);
    /// assert_eq!(FskBandwidth::Bw234.hertz(), 234_300);
    /// assert_eq!(FskBandwidth::Bw312.hertz(), 312_000);
    /// assert_eq!(FskBandwidth::Bw373.hertz(), 373_600);
    /// assert_eq!(FskBandwidth::Bw467.hertz(), 467_000);
    /// ```
    pub fn hertz(&self) -> u32 {
        match self {
            FskBandwidth::Bw4 => 4_800,
            FskBandwidth::Bw5 => 5_800,
            FskBandwidth::Bw7 => 7_300,
            FskBandwidth::Bw9 => 9_700,
            FskBandwidth::Bw11 => 11_700,
            FskBandwidth::Bw14 => 14_600,
            FskBandwidth::Bw19 => 19_500,
            FskBandwidth::Bw23 => 23_400,
            FskBandwidth::Bw29 => 29_300,
            FskBandwidth::Bw39 => 39_000,
            FskBandwidth::Bw46 => 46_900,
            FskBandwidth::Bw58 => 58_600,
            FskBandwidth::Bw78 => 78_200,
            FskBandwidth::Bw93 => 93_800,
            FskBandwidth::Bw117 => 117_300,
            FskBandwidth::Bw156 => 156_200,
            FskBandwidth::Bw187 => 187_200,
            FskBandwidth::Bw234 => 234_300,
            FskBandwidth::Bw312 => 312_000,
            FskBandwidth::Bw373 => 373_600,
            FskBandwidth::Bw467 => 467_000,
        }
    }
}

impl Ord for FskBandwidth {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.hertz().cmp(&other.hertz())
    }
}

impl PartialOrd for FskBandwidth {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.hertz().cmp(&other.hertz()))
    }
}

/// Pulse shaping options for [`FskModParams`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum FskPulseShape {
    /// No filtering applied.
    None = 0b00,
    /// Gaussian BT 0.3
    Bt03 = 0x08,
    /// Gaussian BT 0.5
    Bt05 = 0x09,
    /// Gaussian BT 0.7
    Bt07 = 0x0A,
    /// Gaussian BT 1.0
    Bt10 = 0x0B,
}

/// Bitrate argument for [`FskModParams::set_bitrate`] and
/// [`BpskModParams::set_bitrate`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FskBitrate {
    bits: u32,
}

impl FskBitrate {
    /// Create a new `FskBitrate` from a bitrate in bits per second.
    ///
    /// This the resulting value will be rounded down, and will saturate if
    /// `bps` is outside of the theoretical limits.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskBitrate;
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(9600);
    /// assert_eq!(BITRATE.as_bps(), 9600);
    /// ```
    pub const fn from_bps(bps: u32) -> Self {
        const MAX: u32 = 0x00FF_FFFF;
        if bps == 0 {
            Self { bits: MAX }
        } else {
            let bits: u32 = 32 * 32_000_000 / bps;
            if bits > MAX {
                Self { bits: MAX }
            } else {
                Self { bits }
            }
        }
    }

    /// Create a new `FskBitrate` from a raw bit value.
    ///
    /// bits = 32 × 32 MHz / bitrate
    ///
    /// **Note:** Only the first 24 bits of the `u32` are used, the `bits`
    /// argument will be masked.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskBitrate;
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_raw(0x7D00);
    /// assert_eq!(BITRATE.as_bps(), 32_000);
    /// ```
    pub const fn from_raw(bits: u32) -> Self {
        Self {
            bits: bits & 0x00FF_FFFF,
        }
    }

    /// Return the bitrate in bits per second, rounded down.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskBitrate;
    ///
    /// const BITS_PER_SEC: u32 = 9600;
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(BITS_PER_SEC);
    /// assert_eq!(BITRATE.as_bps(), BITS_PER_SEC);
    /// ```
    pub const fn as_bps(&self) -> u32 {
        if self.bits == 0 {
            0
        } else {
            32 * 32_000_000 / self.bits
        }
    }

    pub(crate) const fn into_bits(self) -> u32 {
        self.bits
    }
}

impl Ord for FskBitrate {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bps().cmp(&other.as_bps())
    }
}

impl PartialOrd for FskBitrate {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.as_bps().cmp(&other.as_bps()))
    }
}

/// Frequency deviation argument for [`FskModParams::set_fdev`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct FskFdev {
    bits: u32,
}

impl FskFdev {
    /// Create a new `FskFdev` from a frequency deviation in hertz, rounded
    /// down.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskFdev;
    ///
    /// const FDEV: FskFdev = FskFdev::from_hertz(31_250);
    /// assert_eq!(FDEV.as_hertz(), 31_250);
    /// ```
    pub const fn from_hertz(hz: u32) -> Self {
        Self {
            bits: ((hz as u64) * (1 << 25) / 32_000_000) as u32 & 0x00FF_FFFF,
        }
    }

    /// Create a new `FskFdev` from a raw bit value.
    ///
    /// bits = fdev × 2<sup>25</sup> / 32 MHz
    ///
    /// **Note:** Only the first 24 bits of the `u32` are used, the `bits`
    /// argument will be masked.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskFdev;
    ///
    /// const FDEV: FskFdev = FskFdev::from_raw(0x8000);
    /// assert_eq!(FDEV.as_hertz(), 31_250);
    /// ```
    pub const fn from_raw(bits: u32) -> Self {
        Self {
            bits: bits & 0x00FF_FFFF,
        }
    }

    /// Return the frequency deviation in hertz, rounded down.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskFdev;
    ///
    /// const HERTZ: u32 = 31_250;
    /// const FDEV: FskFdev = FskFdev::from_hertz(HERTZ);
    /// assert_eq!(FDEV.as_hertz(), HERTZ);
    /// ```
    pub const fn as_hertz(&self) -> u32 {
        ((self.bits as u64) * 32_000_000 / (1 << 25)) as u32
    }

    pub(crate) const fn into_bits(self) -> u32 {
        self.bits
    }
}

/// (G)FSK modulation paramters.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FskModParams {
    buf: [u8; 9],
}

impl FskModParams {
    /// Create a new `FskModParams` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::FskModParams;
    ///
    /// const MOD_PARAMS: FskModParams = FskModParams::new();
    /// ```
    pub const fn new() -> FskModParams {
        FskModParams {
            buf: [
                crate::OpCode::SetModulationParams as u8,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ],
        }
        .set_bitrate(FskBitrate::from_bps(50_000))
        .set_pulse_shape(FskPulseShape::None)
        .set_bandwidth(FskBandwidth::Bw58)
        .set_fdev(FskFdev::from_hertz(25_000))
    }

    /// Set the bitrate.
    ///
    /// # Example
    ///
    /// Setting the bitrate to 32,000 bits per second.
    ///
    /// ```
    /// use stm32wl_hal_subghz::{FskBitrate, FskModParams};
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(32_000);
    /// const MOD_PARAMS: FskModParams = FskModParams::new().set_bitrate(BITRATE);
    /// # assert_eq!(MOD_PARAMS.as_slice()[1], 0x00);
    /// # assert_eq!(MOD_PARAMS.as_slice()[2], 0x7D);
    /// # assert_eq!(MOD_PARAMS.as_slice()[3], 0x00);
    /// ```
    #[must_use = "set_bitrate returns a new FskModParams"]
    pub const fn set_bitrate(mut self, bitrate: FskBitrate) -> FskModParams {
        let bits: u32 = bitrate.into_bits();
        self.buf[1] = ((bits >> 16) & 0xFF) as u8;
        self.buf[2] = ((bits >> 8) & 0xFF) as u8;
        self.buf[3] = (bits & 0xFF) as u8;
        self
    }

    /// Set the pulse shaping.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{FskModParams, FskPulseShape};
    ///
    /// const MOD_PARAMS: FskModParams = FskModParams::new().set_pulse_shape(FskPulseShape::Bt03);
    /// # assert_eq!(MOD_PARAMS.as_slice()[4], 0x08);
    /// ```
    #[must_use = "set_pulse_shape returns a new FskModParams"]
    pub const fn set_pulse_shape(mut self, shape: FskPulseShape) -> FskModParams {
        self.buf[4] = shape as u8;
        self
    }

    /// Set the bandwidth.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{FskBandwidth, FskModParams};
    ///
    /// const MOD_PARAMS: FskModParams = FskModParams::new().set_bandwidth(FskBandwidth::Bw9);
    /// # assert_eq!(MOD_PARAMS.as_slice()[5], 0x1E);
    /// ```
    #[must_use = "set_pulse_shape returns a new FskModParams"]
    pub const fn set_bandwidth(mut self, bw: FskBandwidth) -> FskModParams {
        self.buf[5] = bw as u8;
        self
    }

    /// Set the frequency deviation.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{FskFdev, FskModParams};
    ///
    /// const FDEV: FskFdev = FskFdev::from_hertz(31_250);
    /// const MOD_PARAMS: FskModParams = FskModParams::new().set_fdev(FDEV);
    /// # assert_eq!(MOD_PARAMS.as_slice()[6], 0x00);
    /// # assert_eq!(MOD_PARAMS.as_slice()[7], 0x80);
    /// # assert_eq!(MOD_PARAMS.as_slice()[8], 0x00);
    /// ```
    #[must_use = "set_fdev returns a new FskModParams"]
    pub const fn set_fdev(mut self, fdev: FskFdev) -> FskModParams {
        let bits: u32 = fdev.into_bits();
        self.buf[6] = ((bits >> 16) & 0xFF) as u8;
        self.buf[7] = ((bits >> 8) & 0xFF) as u8;
        self.buf[8] = (bits & 0xFF) as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{FskBandwidth, FskBitrate, FskFdev, FskModParams, FskPulseShape};
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(32_000);
    /// const PULSE_SHAPE: FskPulseShape = FskPulseShape::Bt03;
    /// const BW: FskBandwidth = FskBandwidth::Bw9;
    /// const FDEV: FskFdev = FskFdev::from_hertz(31_250);
    ///
    /// const MOD_PARAMS: FskModParams = FskModParams::new()
    ///     .set_bitrate(BITRATE)
    ///     .set_pulse_shape(PULSE_SHAPE)
    ///     .set_bandwidth(BW)
    ///     .set_fdev(FDEV);
    ///
    /// assert_eq!(
    ///     MOD_PARAMS.as_slice(),
    ///     &[0x8B, 0x00, 0x7D, 0x00, 0x08, 0x1E, 0x00, 0x80, 0x00]
    /// );
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for FskModParams {
    fn default() -> Self {
        Self::new()
    }
}

/// LoRa spreading factor.
///
/// Argument of [`LoRaModParams::set_sf`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SpreadingFactor {
    /// Spreading factor 5.
    Sf5 = 0x05,
    /// Spreading factor 6.
    Sf6 = 0x06,
    /// Spreading factor 7.
    Sf7 = 0x07,
    /// Spreading factor 8.
    Sf8 = 0x08,
    /// Spreading factor 9.
    Sf9 = 0x09,
    /// Spreading factor 10.
    Sf10 = 0xA0,
    /// Spreading factor 11.
    Sf11 = 0xB0,
    /// Spreading factor 12.
    Sf12 = 0xC0,
}

impl From<SpreadingFactor> for u8 {
    fn from(sf: SpreadingFactor) -> Self {
        sf as u8
    }
}

/// LoRa bandwidth.
///
/// Argument of [`LoRaModParams::set_bw`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum LoRaBandwidth {
    /// 7.81 kHz
    Bw7 = 0x00,
    /// 10.42 kHz
    Bw10 = 0x08,
    /// 15.63 kHz
    Bw15 = 0x01,
    /// 20.83 kHz
    Bw20 = 0x09,
    /// 31.25 kHz
    Bw31 = 0x02,
    /// 41.67 kHz
    Bw41 = 0x0A,
    /// 62.50 kHz
    Bw62 = 0x03,
    /// 125 kHz
    Bw125 = 0x04,
    /// 250 kHz
    Bw250 = 0x05,
    /// 500 kHz
    Bw500 = 0x06,
}

impl LoRaBandwidth {
    /// Get the bandwidth in hertz.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::LoRaBandwidth;
    ///
    /// assert_eq!(LoRaBandwidth::Bw7.hertz(), 7_810);
    /// assert_eq!(LoRaBandwidth::Bw10.hertz(), 10_420);
    /// assert_eq!(LoRaBandwidth::Bw15.hertz(), 15_630);
    /// assert_eq!(LoRaBandwidth::Bw20.hertz(), 20_830);
    /// assert_eq!(LoRaBandwidth::Bw31.hertz(), 31_250);
    /// assert_eq!(LoRaBandwidth::Bw41.hertz(), 41_670);
    /// assert_eq!(LoRaBandwidth::Bw62.hertz(), 62_500);
    /// assert_eq!(LoRaBandwidth::Bw125.hertz(), 125_000);
    /// assert_eq!(LoRaBandwidth::Bw250.hertz(), 250_000);
    /// assert_eq!(LoRaBandwidth::Bw500.hertz(), 500_000);
    /// ```
    pub const fn hertz(&self) -> u32 {
        match self {
            LoRaBandwidth::Bw7 => 7_810,
            LoRaBandwidth::Bw10 => 10_420,
            LoRaBandwidth::Bw15 => 15_630,
            LoRaBandwidth::Bw20 => 20_830,
            LoRaBandwidth::Bw31 => 31_250,
            LoRaBandwidth::Bw41 => 41_670,
            LoRaBandwidth::Bw62 => 62_500,
            LoRaBandwidth::Bw125 => 125_000,
            LoRaBandwidth::Bw250 => 250_000,
            LoRaBandwidth::Bw500 => 500_000,
        }
    }
}

impl Ord for LoRaBandwidth {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.hertz().cmp(&other.hertz())
    }
}

impl PartialOrd for LoRaBandwidth {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.hertz().cmp(&other.hertz()))
    }
}

/// LoRa forward error correction coding rate.
///
/// Argument of [`LoRaModParams::set_cr`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum CodingRate {
    /// No forward error correction coding rate 4/4
    Cr44 = 0x00,
    /// Forward error correction coding rate 4/5
    Cr45 = 0x1,
    /// Forward error correction coding rate 4/6
    Cr46 = 0x2,
    /// Forward error correction coding rate 4/7
    Cr47 = 0x3,
    /// Forward error correction coding rate 4/8
    Cr48 = 0x4,
}

/// LoRa modulation paramters.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LoRaModParams {
    buf: [u8; 5],
}

impl LoRaModParams {
    /// Create a new `LoRaModParams` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::LoRaModParams;
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new();
    /// assert_eq!(MOD_PARAMS, LoRaModParams::default());
    /// ```
    pub const fn new() -> LoRaModParams {
        LoRaModParams {
            buf: [
                crate::OpCode::SetModulationParams as u8,
                0x05, // valid spreading factor
                0x00,
                0x00,
                0x00,
            ],
        }
    }

    /// Set the spreading factor.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{LoRaModParams, SpreadingFactor};
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new().set_sf(SpreadingFactor::Sf7);
    /// # assert_eq!(MOD_PARAMS.as_slice(), &[0x8B, 0x07, 0x00, 0x00, 0x00]);
    /// ```
    #[must_use = "set_sf returns a modified LoRaModParams"]
    pub const fn set_sf(mut self, sf: SpreadingFactor) -> Self {
        self.buf[1] = sf as u8;
        self
    }

    /// Set the bandwidth.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{LoRaBandwidth, LoRaModParams};
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new().set_bw(LoRaBandwidth::Bw125);
    /// # assert_eq!(MOD_PARAMS.as_slice(), &[0x8B, 0x05, 0x04, 0x00, 0x00]);
    /// ```
    #[must_use = "set_bw returns a modified LoRaModParams"]
    pub const fn set_bw(mut self, bw: LoRaBandwidth) -> Self {
        self.buf[2] = bw as u8;
        self
    }

    /// Set the forward error correction coding rate.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{CodingRate, LoRaModParams};
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new().set_cr(CodingRate::Cr45);
    /// # assert_eq!(MOD_PARAMS.as_slice(), &[0x8B, 0x05, 0x00, 0x01, 0x00]);
    /// ```
    #[must_use = "set_cr returns a modified LoRaModParams"]
    pub const fn set_cr(mut self, cr: CodingRate) -> Self {
        self.buf[3] = cr as u8;
        self
    }

    /// Set low data rate optimization enable.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::LoRaModParams;
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new().set_ldro_en(true);
    /// # assert_eq!(MOD_PARAMS.as_slice(), &[0x8B, 0x05, 0x00, 0x00, 0x01]);
    /// ```
    #[must_use = "set_ldro_en returns a modified LoRaModParams"]
    pub const fn set_ldro_en(mut self, en: bool) -> Self {
        self.buf[4] = en as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{CodingRate, LoRaBandwidth, LoRaModParams, SpreadingFactor};
    ///
    /// const MOD_PARAMS: LoRaModParams = LoRaModParams::new()
    ///     .set_sf(SpreadingFactor::Sf7)
    ///     .set_bw(LoRaBandwidth::Bw125)
    ///     .set_cr(CodingRate::Cr45)
    ///     .set_ldro_en(false);
    ///
    /// assert_eq!(MOD_PARAMS.as_slice(), &[0x8B, 0x07, 0x04, 0x01, 0x00]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for LoRaModParams {
    fn default() -> Self {
        Self::new()
    }
}

/// BPSK modulation paramters.
///
/// **Note:** There is no method to set the pulse shape because there is only
/// one valid pulse shape (Gaussian BT 0.5).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BpskModParams {
    buf: [u8; 5],
}

impl BpskModParams {
    /// Create a new `BpskModParams` struct.
    ///
    /// This is the same as `default`, but in a `const` function.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::BpskModParams;
    ///
    /// const MOD_PARAMS: BpskModParams = BpskModParams::new();
    /// assert_eq!(MOD_PARAMS, BpskModParams::default());
    /// ```
    pub const fn new() -> BpskModParams {
        const OPCODE: u8 = crate::OpCode::SetModulationParams as u8;
        BpskModParams {
            buf: [OPCODE, 0x1A, 0x0A, 0xAA, 0x16],
        }
    }

    /// Set the bitrate.
    ///
    /// # Example
    ///
    /// Setting the bitrate to 600 bits per second.
    ///
    /// ```
    /// use stm32wl_hal_subghz::{BpskModParams, FskBitrate};
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(600);
    /// const MOD_PARAMS: BpskModParams = BpskModParams::new().set_bitrate(BITRATE);
    /// # assert_eq!(MOD_PARAMS.as_slice()[1], 0x1A);
    /// # assert_eq!(MOD_PARAMS.as_slice()[2], 0x0A);
    /// # assert_eq!(MOD_PARAMS.as_slice()[3], 0xAA);
    /// ```
    #[must_use = "set_bitrate returns a new BpskModParams"]
    pub const fn set_bitrate(mut self, bitrate: FskBitrate) -> BpskModParams {
        let bits: u32 = bitrate.into_bits();
        self.buf[1] = ((bits >> 16) & 0xFF) as u8;
        self.buf[2] = ((bits >> 8) & 0xFF) as u8;
        self.buf[3] = (bits & 0xFF) as u8;
        self
    }

    /// Extracts a slice containing the packet.
    ///
    /// # Example
    ///
    /// ```
    /// use stm32wl_hal_subghz::{BpskModParams, FskBitrate};
    ///
    /// const BITRATE: FskBitrate = FskBitrate::from_bps(100);
    /// const MOD_PARAMS: BpskModParams = BpskModParams::new().set_bitrate(BITRATE);
    /// assert_eq!(MOD_PARAMS.as_slice(), [0x8B, 0x9C, 0x40, 0x00, 0x16]);
    /// ```
    pub const fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}

impl Default for BpskModParams {
    fn default() -> Self {
        Self::new()
    }
}
