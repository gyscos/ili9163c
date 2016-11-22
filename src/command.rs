#[repr(u8)]
pub enum Command {
    /// NOP
    Nop = 0x00,

    /// SWRESET
    SoftwareReset = 0x01,

    /// RDDIDIF
    ///
    /// Reads 3 data parameters after dummy clock
    ReadDisplayId = 0x04,

    /// RDDST
    ///
    /// Reads 4 data parameters after dummy clock
    ReadDisplayStatus = 0x09,

    /// RDDPM
    ReadDisplayPowerMode = 0x0A,

    /// RDDMADCTL
    ReadDisplayMADCTL = 0x0B,

    /// RDDCOLMOD
    ReadDisplayPixelFormat = 0x0C,

    /// RDDIM
    ReadDisplayImageMode = 0x0D,

    /// RDDSM
    ReadDisplaySignalMode = 0x0E,

    /// RDDSM
    ReadDisplaySignalMode2 = 0x0F,

    /// SLPIN
    ///
    /// Enable low-power mode
    SleepIn = 0x10,

    /// SLPOUT
    ///
    /// Leaves low-power mode. Better wait a bit after that.
    SleepOut = 0x11,

    /// PTLON
    ///
    /// Enable partial mode
    PartialModeOn = 0x12,

    /// NORON
    ///
    /// Leaves partial mode, enter normal mode.
    NormalModeOn = 0x13,

    /// INVOFF
    DisplayInversionOff = 0x20,

    /// INVON
    ///
    /// Invert color of every pixel.
    ///
    /// (This is NOT a symetry)
    DisplayInversionOn = 0x21,

    /// GAMSET
    GammaSet = 0x26,

    /// DISPOFF
    ///
    /// Enter DISPLAY OFF mode.
    ///
    /// Does not affect the memory.
    DisplayOff = 0x28,

    /// DISPON
    DisplayOn = 0x29,

    /// CASET
    ///
    /// Sets the active X window
    ///
    /// Takes 4 data bytes: (start_MSB start_LSB end_MSB end_LSB)
    ColumnAddressSet = 0x2A,

    /// PASET
    ///
    /// Sets the active Y window
    ///
    /// Takes 4 data bytes: (start_MSB start_LSB end_MSB end_LSB)
    PageAddressSet = 0x2B,

    /// RAMWR
    ///
    /// Writes data to the memory.
    ///
    /// Takes all following data bytes until the next command byte.
    MemoryWrite = 0x2C,

    /// RGBSET
    ///
    /// Sets the color translation table when using 12 or 16-bit colors.
    ColorSettings = 0x2D,

    /// RAMRD
    MemoryRead = 0x2E,

    /// PLTAR
    ///
    /// Specifies the partial rows to be enabled with PTLON
    PartialArea = 0x30,

    /// VSCRDEF
    ///
    ///
    VerticalScrollingDefinition = 0x33,

    /// TEOFF
    TearingEffectLineOff = 0x34,

    /// TEON
    ///
    /// Defines a V-Blanking setting
    TearingEffectLineOn = 0x35,

    /// MADCTL
    ///
    /// Specifies X and Y symetry, X/Y swap, vertical refresh direction,
    /// RGB/BGR order, and horizontal refresh direction.
    MemoryAccessControl = 0x36,

    /// VSCRSADD
    VerticalScrollingStartAddress = 0x37,

    /// IDMOFF
    IdleModeOff = 0x38,

    /// IDMON
    ///
    /// - 8 colors mode
    /// - lower frequency
    IdleModeOd = 0x39,

    /// COLMOD
    InterfacePixelFormat = 0x3A,

    /// FRMCTR1
    ///
    /// Sets the division ratio for internal clocks of
    /// Normal mode at CPU interface mode.
    FrameRateControlNormal = 0xB1,

    /// FRMCTR2
    ///
    /// Sets the division ratio for internal clocks of
    /// Idle mode at CPU interface mode.
    FrameRateControlIdle = 0xB2,

    /// FRMCTR3
    ///
    /// Sets the division ratio for internal clocks of
    /// Partial mode at CPU interface mode.
    FrameRateControlPartial = 0xB3,

    /// INVCTR
    DisplayInversionControl = 0xB4,

    /// BPCTR
    RgbInterfaceBlankingPorchSetting = 0xB5,

    /// DISSET5
    DisplayFunctionSet5 = 0xB6,

    /// SDOCTR
    SourceDriverDirectionControl = 0xB7,

    /// GDOCTR
    GateDriverDirectionControl = 0xB8,

    /// PWCTR1
    ///
    /// Sets the GVDD and voltage
    PowerControl1 = 0xC0,

    /// PWCTR2
    ///
    /// Sets the AVDD, VCL, VGH and VGL supply power level.
    PowerControl2 = 0xC1,

    /// PWCTR3
    ///
    /// Set the amount of current in Operational
    /// amplifier in normal mode/full colors.
    ///
    /// Adjust the amount of fixed current from the fixed current sources in
    /// the operational amplifier for the source driver.
    PowerControl3 = 0xC2,

    /// PWCTR4
    ///
    /// Set the amount of current in Operational
    /// amplifier in Idle mode/8-colors
    ///
    /// Adjust the amount of fixed current from the fixed current source in
    /// the operational amplifier for the source driver.
    PowerControl4 = 0xC3,

    /// PWCTR4
    ///
    /// Set the amount of current in Operational
    /// amplifier in Partial mode/full-colors
    PowerControl5 = 0xC4,

    /// VMCTR
    VcomControl1 = 0xC5,

    /// VMOFCTR
    VcomOffsetControl = 0xC7,

    /// RDID4
    WriteId4Value = 0xD3,

    NvMemoryFunctionController1 = 0xD5,
    NvMemoryFunctionController2 = 0xD6,
    NvMemoryFunctionController3 = 0xD7,

    /// RDID1
    ReadId1 = 0xDA,

    /// RDID2
    ReadId2 = 0xDB,

    /// RDID3
    ReadId3 = 0xDC,

    /// GAMCTRP0
    PositiveGammaCorrectionSetting = 0xE0,

    /// GAMCTRN0
    NegativeGammaCorrectionSetting = 0xE1,

    /// GAM_R_SEL
    ///
    /// Gamma adjustment E0h and E1h enable control
    GamRSel = 0xF2,
}

#[repr(u8)]
pub enum PixelFormat {
    Bpp16 = 0b0101,
    Bpp18_1 = 0b0110,
    Bpp18_3 = 0b1110,
}

#[repr(u8)]
pub enum GammaCurve {
    Curve1 = 1,
    Curve2 = 2,
    Curve3 = 4,
    Curve4 = 8,
}
