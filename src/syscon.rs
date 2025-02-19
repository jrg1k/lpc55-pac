#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memoryremap: Memoryremap,
    _reserved1: [u8; 0x0c],
    ahbmatprio: Ahbmatprio,
    _reserved2: [u8; 0x24],
    cpu0stckcal: Cpu0stckcal,
    cpu0nstckcal: Cpu0nstckcal,
    cpu1stckcal: Cpu1stckcal,
    _reserved5: [u8; 0x04],
    nmisrc: Nmisrc,
    _reserved6: [u8; 0xb4],
    presetctrl0: Presetctrl0,
    presetctrl1: Presetctrl1,
    presetctrl2: Presetctrl2,
    _reserved9: [u8; 0x14],
    presetctrlset: [Presetctrlset; 3],
    _reserved10: [u8; 0x14],
    presetctrlclr: [Presetctrlclr; 3],
    _reserved11: [u8; 0x14],
    swr_reset: SwrReset,
    _reserved12: [u8; 0x9c],
    ahbclkctrl0: Ahbclkctrl0,
    ahbclkctrl1: Ahbclkctrl1,
    ahbclkctrl2: Ahbclkctrl2,
    _reserved15: [u8; 0x14],
    ahbclkctrlset: [Ahbclkctrlset; 3],
    _reserved16: [u8; 0x14],
    ahbclkctrlclr: [Ahbclkctrlclr; 3],
    _reserved17: [u8; 0x14],
    _reserved_17_systickclksel0: [u8; 0x04],
    _reserved_18_systickclksel1: [u8; 0x04],
    traceclksel: Traceclksel,
    _reserved_20_ctimerclksel0: [u8; 0x04],
    _reserved_21_ctimerclksel1: [u8; 0x04],
    _reserved_22_ctimerclksel2: [u8; 0x04],
    _reserved_23_ctimerclksel3: [u8; 0x04],
    _reserved_24_ctimerclksel4: [u8; 0x04],
    mainclksela: Mainclksela,
    mainclkselb: Mainclkselb,
    clkoutsel: Clkoutsel,
    _reserved28: [u8; 0x04],
    pll0clksel: Pll0clksel,
    pll1clksel: Pll1clksel,
    _reserved30: [u8; 0x0c],
    adcclksel: Adcclksel,
    usb0clksel: Usb0clksel,
    _reserved32: [u8; 0x04],
    _reserved_32_fcclksel0: [u8; 0x04],
    _reserved_33_fcclksel1: [u8; 0x04],
    _reserved_34_fcclksel2: [u8; 0x04],
    _reserved_35_fcclksel3: [u8; 0x04],
    _reserved_36_fcclksel4: [u8; 0x04],
    _reserved_37_fcclksel5: [u8; 0x04],
    _reserved_38_fcclksel6: [u8; 0x04],
    _reserved_39_fcclksel7: [u8; 0x04],
    hslspiclksel: Hslspiclksel,
    _reserved41: [u8; 0x0c],
    mclkclksel: Mclkclksel,
    _reserved42: [u8; 0x0c],
    sctclksel: Sctclksel,
    _reserved43: [u8; 0x04],
    sdioclksel: Sdioclksel,
    _reserved44: [u8; 0x04],
    systickclkdiv0: Systickclkdiv0,
    systickclkdiv1: Systickclkdiv1,
    traceclkdiv: Traceclkdiv,
    _reserved47: [u8; 0x14],
    _reserved_47_flexfrg0ctrl: [u8; 0x04],
    _reserved_48_flexfrg1ctrl: [u8; 0x04],
    _reserved_49_flexfrg2ctrl: [u8; 0x04],
    _reserved_50_flexfrg3ctrl: [u8; 0x04],
    _reserved_51_flexfrg4ctrl: [u8; 0x04],
    _reserved_52_flexfrg5ctrl: [u8; 0x04],
    _reserved_53_flexfrg6ctrl: [u8; 0x04],
    _reserved_54_flexfrg7ctrl: [u8; 0x04],
    _reserved55: [u8; 0x40],
    ahbclkdiv: Ahbclkdiv,
    clkoutdiv: Clkoutdiv,
    frohfdiv: Frohfdiv,
    wdtclkdiv: Wdtclkdiv,
    _reserved59: [u8; 0x04],
    adcclkdiv: Adcclkdiv,
    usb0clkdiv: Usb0clkdiv,
    _reserved61: [u8; 0x10],
    mclkdiv: Mclkdiv,
    _reserved62: [u8; 0x04],
    sctclkdiv: Sctclkdiv,
    _reserved63: [u8; 0x04],
    sdioclkdiv: Sdioclkdiv,
    _reserved64: [u8; 0x04],
    pll0clkdiv: Pll0clkdiv,
    _reserved65: [u8; 0x34],
    clockgenupdatelockout: Clockgenupdatelockout,
    fmccr: Fmccr,
    _reserved67: [u8; 0x08],
    usb0needclkctrl: Usb0needclkctrl,
    usb0needclkstat: Usb0needclkstat,
    _reserved69: [u8; 0x08],
    fmcflush: Fmcflush,
    mclkio: Mclkio,
    usb1needclkctrl: Usb1needclkctrl,
    usb1needclkstat: Usb1needclkstat,
    _reserved73: [u8; 0x34],
    sdioclkctrl: Sdioclkctrl,
    _reserved74: [u8; 0xfc],
    pll1ctrl: Pll1ctrl,
    pll1stat: Pll1stat,
    pll1ndec: Pll1ndec,
    pll1mdec: Pll1mdec,
    pll1pdec: Pll1pdec,
    _reserved79: [u8; 0x0c],
    pll0ctrl: Pll0ctrl,
    pll0stat: Pll0stat,
    pll0ndec: Pll0ndec,
    pll0pdec: Pll0pdec,
    pll0sscg0: Pll0sscg0,
    pll0sscg1: Pll0sscg1,
    _reserved85: [u8; 0x016c],
    funcretentionctrl: Funcretentionctrl,
    _reserved86: [u8; 0xf8],
    cpuctrl: Cpuctrl,
    cpboot: Cpboot,
    _reserved88: [u8; 0x04],
    cpstat: Cpstat,
    _reserved89: [u8; 0x0208],
    clock_ctrl: ClockCtrl,
    _reserved90: [u8; 0xf4],
    comp_int_ctrl: CompIntCtrl,
    comp_int_status: CompIntStatus,
    _reserved92: [u8; 0x02ec],
    autoclkgateoverride: Autoclkgateoverride,
    gpiopsync: Gpiopsync,
    _reserved94: [u8; 0x0194],
    debug_lock_en: DebugLockEn,
    debug_features: DebugFeatures,
    debug_features_dp: DebugFeaturesDp,
    _reserved97: [u8; 0x10],
    key_block: KeyBlock,
    debug_auth_beacon: DebugAuthBeacon,
    _reserved99: [u8; 0x10],
    cpucfg: Cpucfg,
    _reserved100: [u8; 0x20],
    device_id0: DeviceId0,
    dieid: Dieid,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    #[inline(always)]
    pub const fn memoryremap(&self) -> &Memoryremap {
        &self.memoryremap
    }
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    #[inline(always)]
    pub const fn ahbmatprio(&self) -> &Ahbmatprio {
        &self.ahbmatprio
    }
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0stckcal(&self) -> &Cpu0stckcal {
        &self.cpu0stckcal
    }
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0nstckcal(&self) -> &Cpu0nstckcal {
        &self.cpu0nstckcal
    }
    #[doc = "0x40 - System tick calibration for CPU1"]
    #[inline(always)]
    pub const fn cpu1stckcal(&self) -> &Cpu1stckcal {
        &self.cpu1stckcal
    }
    #[doc = "0x48 - NMI Source Select"]
    #[inline(always)]
    pub const fn nmisrc(&self) -> &Nmisrc {
        &self.nmisrc
    }
    #[doc = "0x100 - Peripheral reset control 0"]
    #[inline(always)]
    pub const fn presetctrl0(&self) -> &Presetctrl0 {
        &self.presetctrl0
    }
    #[doc = "0x104 - Peripheral reset control 1"]
    #[inline(always)]
    pub const fn presetctrl1(&self) -> &Presetctrl1 {
        &self.presetctrl1
    }
    #[doc = "0x108 - Peripheral reset control 2"]
    #[inline(always)]
    pub const fn presetctrl2(&self) -> &Presetctrl2 {
        &self.presetctrl2
    }
    #[doc = "0x120..0x12c - Peripheral reset control set register"]
    #[inline(always)]
    pub const fn presetctrlset(&self, n: usize) -> &Presetctrlset {
        &self.presetctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x12c - Peripheral reset control set register"]
    #[inline(always)]
    pub fn presetctrlset_iter(&self) -> impl Iterator<Item = &Presetctrlset> {
        self.presetctrlset.iter()
    }
    #[doc = "0x140..0x14c - Peripheral reset control clear register"]
    #[inline(always)]
    pub const fn presetctrlclr(&self, n: usize) -> &Presetctrlclr {
        &self.presetctrlclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x14c - Peripheral reset control clear register"]
    #[inline(always)]
    pub fn presetctrlclr_iter(&self) -> impl Iterator<Item = &Presetctrlclr> {
        self.presetctrlclr.iter()
    }
    #[doc = "0x160 - generate a software_reset"]
    #[inline(always)]
    pub const fn swr_reset(&self) -> &SwrReset {
        &self.swr_reset
    }
    #[doc = "0x200 - AHB Clock control 0"]
    #[inline(always)]
    pub const fn ahbclkctrl0(&self) -> &Ahbclkctrl0 {
        &self.ahbclkctrl0
    }
    #[doc = "0x204 - AHB Clock control 1"]
    #[inline(always)]
    pub const fn ahbclkctrl1(&self) -> &Ahbclkctrl1 {
        &self.ahbclkctrl1
    }
    #[doc = "0x208 - AHB Clock control 2"]
    #[inline(always)]
    pub const fn ahbclkctrl2(&self) -> &Ahbclkctrl2 {
        &self.ahbclkctrl2
    }
    #[doc = "0x220..0x22c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlset(&self, n: usize) -> &Ahbclkctrlset {
        &self.ahbclkctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x22c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ahbclkctrlset_iter(&self) -> impl Iterator<Item = &Ahbclkctrlset> {
        self.ahbclkctrlset.iter()
    }
    #[doc = "0x240..0x24c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlclr(&self, n: usize) -> &Ahbclkctrlclr {
        &self.ahbclkctrlclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x24c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ahbclkctrlclr_iter(&self) -> impl Iterator<Item = &Ahbclkctrlclr> {
        self.ahbclkctrlclr.iter()
    }
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclkselx0(&self) -> &Systickclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub const fn systickclksel0(&self) -> &Systickclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x264 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclkselx1(&self) -> &Systickclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub const fn systickclksel1(&self) -> &Systickclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x268 - Trace clock source select"]
    #[inline(always)]
    pub const fn traceclksel(&self) -> &Traceclksel {
        &self.traceclksel
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx0(&self) -> &Ctimerclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel0(&self) -> &Ctimerclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx1(&self) -> &Ctimerclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel1(&self) -> &Ctimerclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx2(&self) -> &Ctimerclkselx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel2(&self) -> &Ctimerclksel2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx3(&self) -> &Ctimerclkselx3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel3(&self) -> &Ctimerclksel3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclkselx4(&self) -> &Ctimerclkselx4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel4(&self) -> &Ctimerclksel4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x280 - Main clock A source select"]
    #[inline(always)]
    pub const fn mainclksela(&self) -> &Mainclksela {
        &self.mainclksela
    }
    #[doc = "0x284 - Main clock source select"]
    #[inline(always)]
    pub const fn mainclkselb(&self) -> &Mainclkselb {
        &self.mainclkselb
    }
    #[doc = "0x288 - CLKOUT clock source select"]
    #[inline(always)]
    pub const fn clkoutsel(&self) -> &Clkoutsel {
        &self.clkoutsel
    }
    #[doc = "0x290 - PLL0 clock source select"]
    #[inline(always)]
    pub const fn pll0clksel(&self) -> &Pll0clksel {
        &self.pll0clksel
    }
    #[doc = "0x294 - PLL1 clock source select"]
    #[inline(always)]
    pub const fn pll1clksel(&self) -> &Pll1clksel {
        &self.pll1clksel
    }
    #[doc = "0x2a4 - ADC clock source select"]
    #[inline(always)]
    pub const fn adcclksel(&self) -> &Adcclksel {
        &self.adcclksel
    }
    #[doc = "0x2a8 - FS USB clock source select"]
    #[inline(always)]
    pub const fn usb0clksel(&self) -> &Usb0clksel {
        &self.usb0clksel
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx0(&self) -> &Fcclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel0(&self) -> &Fcclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx1(&self) -> &Fcclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel1(&self) -> &Fcclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx2(&self) -> &Fcclkselx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel2(&self) -> &Fcclksel2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx3(&self) -> &Fcclkselx3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(700).cast() }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel3(&self) -> &Fcclksel3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(700).cast() }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx4(&self) -> &Fcclkselx4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel4(&self) -> &Fcclksel4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx5(&self) -> &Fcclkselx5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(708).cast() }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel5(&self) -> &Fcclksel5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(708).cast() }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx6(&self) -> &Fcclkselx6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel6(&self) -> &Fcclksel6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclkselx7(&self) -> &Fcclkselx7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(716).cast() }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel7(&self) -> &Fcclksel7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(716).cast() }
    }
    #[doc = "0x2d0 - HS LSPI clock source select"]
    #[inline(always)]
    pub const fn hslspiclksel(&self) -> &Hslspiclksel {
        &self.hslspiclksel
    }
    #[doc = "0x2e0 - MCLK clock source select"]
    #[inline(always)]
    pub const fn mclkclksel(&self) -> &Mclkclksel {
        &self.mclkclksel
    }
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    #[inline(always)]
    pub const fn sctclksel(&self) -> &Sctclksel {
        &self.sctclksel
    }
    #[doc = "0x2f8 - SDIO clock source select"]
    #[inline(always)]
    pub const fn sdioclksel(&self) -> &Sdioclksel {
        &self.sdioclksel
    }
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    #[inline(always)]
    pub const fn systickclkdiv0(&self) -> &Systickclkdiv0 {
        &self.systickclkdiv0
    }
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    #[inline(always)]
    pub const fn systickclkdiv1(&self) -> &Systickclkdiv1 {
        &self.systickclkdiv1
    }
    #[doc = "0x308 - TRACE clock divider"]
    #[inline(always)]
    pub const fn traceclkdiv(&self) -> &Traceclkdiv {
        &self.traceclkdiv
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl0(&self) -> &Flexfrgxctrl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(800).cast() }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub const fn flexfrg0ctrl(&self) -> &Flexfrg0ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(800).cast() }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl1(&self) -> &Flexfrgxctrl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(804).cast() }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub const fn flexfrg1ctrl(&self) -> &Flexfrg1ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(804).cast() }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl2(&self) -> &Flexfrgxctrl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(808).cast() }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub const fn flexfrg2ctrl(&self) -> &Flexfrg2ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(808).cast() }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl3(&self) -> &Flexfrgxctrl3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(812).cast() }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub const fn flexfrg3ctrl(&self) -> &Flexfrg3ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(812).cast() }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl4(&self) -> &Flexfrgxctrl4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(816).cast() }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub const fn flexfrg4ctrl(&self) -> &Flexfrg4ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(816).cast() }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl5(&self) -> &Flexfrgxctrl5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(820).cast() }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub const fn flexfrg5ctrl(&self) -> &Flexfrg5ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(820).cast() }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl6(&self) -> &Flexfrgxctrl6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(824).cast() }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub const fn flexfrg6ctrl(&self) -> &Flexfrg6ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(824).cast() }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgxctrl7(&self) -> &Flexfrgxctrl7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(828).cast() }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub const fn flexfrg7ctrl(&self) -> &Flexfrg7ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(828).cast() }
    }
    #[doc = "0x380 - System clock divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(&self) -> &Ahbclkdiv {
        &self.ahbclkdiv
    }
    #[doc = "0x384 - CLKOUT clock divider"]
    #[inline(always)]
    pub const fn clkoutdiv(&self) -> &Clkoutdiv {
        &self.clkoutdiv
    }
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    #[inline(always)]
    pub const fn frohfdiv(&self) -> &Frohfdiv {
        &self.frohfdiv
    }
    #[doc = "0x38c - WDT clock divider"]
    #[inline(always)]
    pub const fn wdtclkdiv(&self) -> &Wdtclkdiv {
        &self.wdtclkdiv
    }
    #[doc = "0x394 - ADC clock divider"]
    #[inline(always)]
    pub const fn adcclkdiv(&self) -> &Adcclkdiv {
        &self.adcclkdiv
    }
    #[doc = "0x398 - USB0 Clock divider"]
    #[inline(always)]
    pub const fn usb0clkdiv(&self) -> &Usb0clkdiv {
        &self.usb0clkdiv
    }
    #[doc = "0x3ac - I2S MCLK clock divider"]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> &Mclkdiv {
        &self.mclkdiv
    }
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    #[inline(always)]
    pub const fn sctclkdiv(&self) -> &Sctclkdiv {
        &self.sctclkdiv
    }
    #[doc = "0x3bc - SDIO clock divider"]
    #[inline(always)]
    pub const fn sdioclkdiv(&self) -> &Sdioclkdiv {
        &self.sdioclkdiv
    }
    #[doc = "0x3c4 - PLL0 clock divider"]
    #[inline(always)]
    pub const fn pll0clkdiv(&self) -> &Pll0clkdiv {
        &self.pll0clkdiv
    }
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    #[inline(always)]
    pub const fn clockgenupdatelockout(&self) -> &Clockgenupdatelockout {
        &self.clockgenupdatelockout
    }
    #[doc = "0x400 - FMC configuration register"]
    #[inline(always)]
    pub const fn fmccr(&self) -> &Fmccr {
        &self.fmccr
    }
    #[doc = "0x40c - USB0 need clock control"]
    #[inline(always)]
    pub const fn usb0needclkctrl(&self) -> &Usb0needclkctrl {
        &self.usb0needclkctrl
    }
    #[doc = "0x410 - USB0 need clock status"]
    #[inline(always)]
    pub const fn usb0needclkstat(&self) -> &Usb0needclkstat {
        &self.usb0needclkstat
    }
    #[doc = "0x41c - FMCflush control"]
    #[inline(always)]
    pub const fn fmcflush(&self) -> &Fmcflush {
        &self.fmcflush
    }
    #[doc = "0x420 - MCLK control"]
    #[inline(always)]
    pub const fn mclkio(&self) -> &Mclkio {
        &self.mclkio
    }
    #[doc = "0x424 - USB1 need clock control"]
    #[inline(always)]
    pub const fn usb1needclkctrl(&self) -> &Usb1needclkctrl {
        &self.usb1needclkctrl
    }
    #[doc = "0x428 - USB1 need clock status"]
    #[inline(always)]
    pub const fn usb1needclkstat(&self) -> &Usb1needclkstat {
        &self.usb1needclkstat
    }
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    #[inline(always)]
    pub const fn sdioclkctrl(&self) -> &Sdioclkctrl {
        &self.sdioclkctrl
    }
    #[doc = "0x560 - PLL1 550m control"]
    #[inline(always)]
    pub const fn pll1ctrl(&self) -> &Pll1ctrl {
        &self.pll1ctrl
    }
    #[doc = "0x564 - PLL1 550m status"]
    #[inline(always)]
    pub const fn pll1stat(&self) -> &Pll1stat {
        &self.pll1stat
    }
    #[doc = "0x568 - PLL1 550m N divider"]
    #[inline(always)]
    pub const fn pll1ndec(&self) -> &Pll1ndec {
        &self.pll1ndec
    }
    #[doc = "0x56c - PLL1 550m M divider"]
    #[inline(always)]
    pub const fn pll1mdec(&self) -> &Pll1mdec {
        &self.pll1mdec
    }
    #[doc = "0x570 - PLL1 550m P divider"]
    #[inline(always)]
    pub const fn pll1pdec(&self) -> &Pll1pdec {
        &self.pll1pdec
    }
    #[doc = "0x580 - PLL0 550m control"]
    #[inline(always)]
    pub const fn pll0ctrl(&self) -> &Pll0ctrl {
        &self.pll0ctrl
    }
    #[doc = "0x584 - PLL0 550m status"]
    #[inline(always)]
    pub const fn pll0stat(&self) -> &Pll0stat {
        &self.pll0stat
    }
    #[doc = "0x588 - PLL0 550m N divider"]
    #[inline(always)]
    pub const fn pll0ndec(&self) -> &Pll0ndec {
        &self.pll0ndec
    }
    #[doc = "0x58c - PLL0 550m P divider"]
    #[inline(always)]
    pub const fn pll0pdec(&self) -> &Pll0pdec {
        &self.pll0pdec
    }
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0"]
    #[inline(always)]
    pub const fn pll0sscg0(&self) -> &Pll0sscg0 {
        &self.pll0sscg0
    }
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1"]
    #[inline(always)]
    pub const fn pll0sscg1(&self) -> &Pll0sscg1 {
        &self.pll0sscg1
    }
    #[doc = "0x704 - Functional retention control register"]
    #[inline(always)]
    pub const fn funcretentionctrl(&self) -> &Funcretentionctrl {
        &self.funcretentionctrl
    }
    #[doc = "0x800 - CPU Control for multiple processors"]
    #[inline(always)]
    pub const fn cpuctrl(&self) -> &Cpuctrl {
        &self.cpuctrl
    }
    #[doc = "0x804 - Coprocessor Boot Address"]
    #[inline(always)]
    pub const fn cpboot(&self) -> &Cpboot {
        &self.cpboot
    }
    #[doc = "0x80c - CPU Status"]
    #[inline(always)]
    pub const fn cpstat(&self) -> &Cpstat {
        &self.cpstat
    }
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0xb10 - Comparator Interrupt control"]
    #[inline(always)]
    pub const fn comp_int_ctrl(&self) -> &CompIntCtrl {
        &self.comp_int_ctrl
    }
    #[doc = "0xb14 - Comparator Interrupt status"]
    #[inline(always)]
    pub const fn comp_int_status(&self) -> &CompIntStatus {
        &self.comp_int_status
    }
    #[doc = "0xe04 - Control automatic clock gating"]
    #[inline(always)]
    pub const fn autoclkgateoverride(&self) -> &Autoclkgateoverride {
        &self.autoclkgateoverride
    }
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    #[inline(always)]
    pub const fn gpiopsync(&self) -> &Gpiopsync {
        &self.gpiopsync
    }
    #[doc = "0xfa0 - Control write access to security registers."]
    #[inline(always)]
    pub const fn debug_lock_en(&self) -> &DebugLockEn {
        &self.debug_lock_en
    }
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    #[inline(always)]
    pub const fn debug_features(&self) -> &DebugFeatures {
        &self.debug_features
    }
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    #[inline(always)]
    pub const fn debug_features_dp(&self) -> &DebugFeaturesDp {
        &self.debug_features_dp
    }
    #[doc = "0xfbc - block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn key_block(&self) -> &KeyBlock {
        &self.key_block
    }
    #[doc = "0xfc0 - Debug authentication BEACON register"]
    #[inline(always)]
    pub const fn debug_auth_beacon(&self) -> &DebugAuthBeacon {
        &self.debug_auth_beacon
    }
    #[doc = "0xfd4 - CPUs configuration register"]
    #[inline(always)]
    pub const fn cpucfg(&self) -> &Cpucfg {
        &self.cpucfg
    }
    #[doc = "0xff8 - Device ID"]
    #[inline(always)]
    pub const fn device_id0(&self) -> &DeviceId0 {
        &self.device_id0
    }
    #[doc = "0xffc - Chip revision ID and Number"]
    #[inline(always)]
    pub const fn dieid(&self) -> &Dieid {
        &self.dieid
    }
}
#[doc = "MEMORYREMAP (rw) register accessor: Memory Remap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`memoryremap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memoryremap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memoryremap`]
module"]
#[doc(alias = "MEMORYREMAP")]
pub type Memoryremap = crate::Reg<memoryremap::MemoryremapSpec>;
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHBMATPRIO (rw) register accessor: AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmatprio`]
module"]
#[doc(alias = "AHBMATPRIO")]
pub type Ahbmatprio = crate::Reg<ahbmatprio::AhbmatprioSpec>;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "CPU0STCKCAL (rw) register accessor: System tick calibration for secure part of CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0stckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0stckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0stckcal`]
module"]
#[doc(alias = "CPU0STCKCAL")]
pub type Cpu0stckcal = crate::Reg<cpu0stckcal::Cpu0stckcalSpec>;
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "CPU0NSTCKCAL (rw) register accessor: System tick calibration for non-secure part of CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0nstckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0nstckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0nstckcal`]
module"]
#[doc(alias = "CPU0NSTCKCAL")]
pub type Cpu0nstckcal = crate::Reg<cpu0nstckcal::Cpu0nstckcalSpec>;
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "CPU1STCKCAL (rw) register accessor: System tick calibration for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1stckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1stckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1stckcal`]
module"]
#[doc(alias = "CPU1STCKCAL")]
pub type Cpu1stckcal = crate::Reg<cpu1stckcal::Cpu1stckcalSpec>;
#[doc = "System tick calibration for CPU1"]
pub mod cpu1stckcal;
#[doc = "NMISRC (rw) register accessor: NMI Source Select\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmisrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisrc`]
module"]
#[doc(alias = "NMISRC")]
pub type Nmisrc = crate::Reg<nmisrc::NmisrcSpec>;
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "PRESETCTRL0 (rw) register accessor: Peripheral reset control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl0`]
module"]
#[doc(alias = "PRESETCTRL0")]
pub type Presetctrl0 = crate::Reg<presetctrl0::Presetctrl0Spec>;
#[doc = "Peripheral reset control 0"]
pub mod presetctrl0;
#[doc = "PRESETCTRL1 (rw) register accessor: Peripheral reset control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl1`]
module"]
#[doc(alias = "PRESETCTRL1")]
pub type Presetctrl1 = crate::Reg<presetctrl1::Presetctrl1Spec>;
#[doc = "Peripheral reset control 1"]
pub mod presetctrl1;
#[doc = "PRESETCTRL2 (rw) register accessor: Peripheral reset control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl2`]
module"]
#[doc(alias = "PRESETCTRL2")]
pub type Presetctrl2 = crate::Reg<presetctrl2::Presetctrl2Spec>;
#[doc = "Peripheral reset control 2"]
pub mod presetctrl2;
#[doc = "PRESETCTRLSET (rw) register accessor: Peripheral reset control set register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrlset`]
module"]
#[doc(alias = "PRESETCTRLSET")]
pub type Presetctrlset = crate::Reg<presetctrlset::PresetctrlsetSpec>;
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "PRESETCTRLCLR (rw) register accessor: Peripheral reset control clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrlclr`]
module"]
#[doc(alias = "PRESETCTRLCLR")]
pub type Presetctrlclr = crate::Reg<presetctrlclr::PresetctrlclrSpec>;
#[doc = "Peripheral reset control clear register"]
pub mod presetctrlclr;
#[doc = "SWR_RESET (w) register accessor: generate a software_reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr_reset`]
module"]
#[doc(alias = "SWR_RESET")]
pub type SwrReset = crate::Reg<swr_reset::SwrResetSpec>;
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHBCLKCTRL0 (rw) register accessor: AHB Clock control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl0`]
module"]
#[doc(alias = "AHBCLKCTRL0")]
pub type Ahbclkctrl0 = crate::Reg<ahbclkctrl0::Ahbclkctrl0Spec>;
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl0;
#[doc = "AHBCLKCTRL1 (rw) register accessor: AHB Clock control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl1`]
module"]
#[doc(alias = "AHBCLKCTRL1")]
pub type Ahbclkctrl1 = crate::Reg<ahbclkctrl1::Ahbclkctrl1Spec>;
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl1;
#[doc = "AHBCLKCTRL2 (rw) register accessor: AHB Clock control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl2`]
module"]
#[doc(alias = "AHBCLKCTRL2")]
pub type Ahbclkctrl2 = crate::Reg<ahbclkctrl2::Ahbclkctrl2Spec>;
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl2;
#[doc = "AHBCLKCTRLSET (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrlset`]
module"]
#[doc(alias = "AHBCLKCTRLSET")]
pub type Ahbclkctrlset = crate::Reg<ahbclkctrlset::AhbclkctrlsetSpec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "AHBCLKCTRLCLR (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrlclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrlclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrlclr`]
module"]
#[doc(alias = "AHBCLKCTRLCLR")]
pub type Ahbclkctrlclr = crate::Reg<ahbclkctrlclr::AhbclkctrlclrSpec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "SYSTICKCLKSEL0 (rw) register accessor: System Tick Timer for CPU0 source select\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel0`]
module"]
#[doc(alias = "SYSTICKCLKSEL0")]
pub type Systickclksel0 = crate::Reg<systickclksel0::Systickclksel0Spec>;
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel0;
#[doc = "SYSTICKCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkselx0`]
module"]
#[doc(alias = "SYSTICKCLKSELX0")]
pub type Systickclkselx0 = crate::Reg<systickclkselx0::Systickclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod systickclkselx0;
#[doc = "SYSTICKCLKSEL1 (rw) register accessor: System Tick Timer for CPU1 source select\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel1`]
module"]
#[doc(alias = "SYSTICKCLKSEL1")]
pub type Systickclksel1 = crate::Reg<systickclksel1::Systickclksel1Spec>;
#[doc = "System Tick Timer for CPU1 source select"]
pub mod systickclksel1;
#[doc = "SYSTICKCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkselx1`]
module"]
#[doc(alias = "SYSTICKCLKSELX1")]
pub type Systickclkselx1 = crate::Reg<systickclkselx1::Systickclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod systickclkselx1;
#[doc = "TRACECLKSEL (rw) register accessor: Trace clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclksel`]
module"]
#[doc(alias = "TRACECLKSEL")]
pub type Traceclksel = crate::Reg<traceclksel::TraceclkselSpec>;
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTIMERCLKSEL0 (rw) register accessor: CTimer 0 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel0`]
module"]
#[doc(alias = "CTIMERCLKSEL0")]
pub type Ctimerclksel0 = crate::Reg<ctimerclksel0::Ctimerclksel0Spec>;
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel0;
#[doc = "CTIMERCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclkselx0`]
module"]
#[doc(alias = "CTIMERCLKSELX0")]
pub type Ctimerclkselx0 = crate::Reg<ctimerclkselx0::Ctimerclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx0;
#[doc = "CTIMERCLKSEL1 (rw) register accessor: CTimer 1 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel1`]
module"]
#[doc(alias = "CTIMERCLKSEL1")]
pub type Ctimerclksel1 = crate::Reg<ctimerclksel1::Ctimerclksel1Spec>;
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel1;
#[doc = "CTIMERCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclkselx1`]
module"]
#[doc(alias = "CTIMERCLKSELX1")]
pub type Ctimerclkselx1 = crate::Reg<ctimerclkselx1::Ctimerclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx1;
#[doc = "CTIMERCLKSEL2 (rw) register accessor: CTimer 2 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel2`]
module"]
#[doc(alias = "CTIMERCLKSEL2")]
pub type Ctimerclksel2 = crate::Reg<ctimerclksel2::Ctimerclksel2Spec>;
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel2;
#[doc = "CTIMERCLKSELX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclkselx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclkselx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclkselx2`]
module"]
#[doc(alias = "CTIMERCLKSELX2")]
pub type Ctimerclkselx2 = crate::Reg<ctimerclkselx2::Ctimerclkselx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx2;
#[doc = "CTIMERCLKSEL3 (rw) register accessor: CTimer 3 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel3`]
module"]
#[doc(alias = "CTIMERCLKSEL3")]
pub type Ctimerclksel3 = crate::Reg<ctimerclksel3::Ctimerclksel3Spec>;
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel3;
#[doc = "CTIMERCLKSELX3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclkselx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclkselx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclkselx3`]
module"]
#[doc(alias = "CTIMERCLKSELX3")]
pub type Ctimerclkselx3 = crate::Reg<ctimerclkselx3::Ctimerclkselx3Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx3;
#[doc = "CTIMERCLKSEL4 (rw) register accessor: CTimer 4 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel4`]
module"]
#[doc(alias = "CTIMERCLKSEL4")]
pub type Ctimerclksel4 = crate::Reg<ctimerclksel4::Ctimerclksel4Spec>;
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel4;
#[doc = "CTIMERCLKSELX4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclkselx4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclkselx4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclkselx4`]
module"]
#[doc(alias = "CTIMERCLKSELX4")]
pub type Ctimerclkselx4 = crate::Reg<ctimerclkselx4::Ctimerclkselx4Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclkselx4;
#[doc = "MAINCLKSELA (rw) register accessor: Main clock A source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksela::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksela::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclksela`]
module"]
#[doc(alias = "MAINCLKSELA")]
pub type Mainclksela = crate::Reg<mainclksela::MainclkselaSpec>;
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: Main clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclkselb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclkselb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclkselb`]
module"]
#[doc(alias = "MAINCLKSELB")]
pub type Mainclkselb = crate::Reg<mainclkselb::MainclkselbSpec>;
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUTSEL (rw) register accessor: CLKOUT clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutsel`]
module"]
#[doc(alias = "CLKOUTSEL")]
pub type Clkoutsel = crate::Reg<clkoutsel::ClkoutselSpec>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0CLKSEL (rw) register accessor: PLL0 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0clksel`]
module"]
#[doc(alias = "PLL0CLKSEL")]
pub type Pll0clksel = crate::Reg<pll0clksel::Pll0clkselSpec>;
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1CLKSEL (rw) register accessor: PLL1 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1clksel`]
module"]
#[doc(alias = "PLL1CLKSEL")]
pub type Pll1clksel = crate::Reg<pll1clksel::Pll1clkselSpec>;
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADCCLKSEL (rw) register accessor: ADC clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`adcclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclksel`]
module"]
#[doc(alias = "ADCCLKSEL")]
pub type Adcclksel = crate::Reg<adcclksel::AdcclkselSpec>;
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0CLKSEL (rw) register accessor: FS USB clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0clksel`]
module"]
#[doc(alias = "USB0CLKSEL")]
pub type Usb0clksel = crate::Reg<usb0clksel::Usb0clkselSpec>;
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "FCCLKSEL0 (rw) register accessor: Flexcomm Interface 0 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel0`]
module"]
#[doc(alias = "FCCLKSEL0")]
pub type Fcclksel0 = crate::Reg<fcclksel0::Fcclksel0Spec>;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel0;
#[doc = "FCCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx0`]
module"]
#[doc(alias = "FCCLKSELX0")]
pub type Fcclkselx0 = crate::Reg<fcclkselx0::Fcclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx0;
#[doc = "FCCLKSEL1 (rw) register accessor: Flexcomm Interface 1 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel1`]
module"]
#[doc(alias = "FCCLKSEL1")]
pub type Fcclksel1 = crate::Reg<fcclksel1::Fcclksel1Spec>;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel1;
#[doc = "FCCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx1`]
module"]
#[doc(alias = "FCCLKSELX1")]
pub type Fcclkselx1 = crate::Reg<fcclkselx1::Fcclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx1;
#[doc = "FCCLKSEL2 (rw) register accessor: Flexcomm Interface 2 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel2`]
module"]
#[doc(alias = "FCCLKSEL2")]
pub type Fcclksel2 = crate::Reg<fcclksel2::Fcclksel2Spec>;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel2;
#[doc = "FCCLKSELX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx2`]
module"]
#[doc(alias = "FCCLKSELX2")]
pub type Fcclkselx2 = crate::Reg<fcclkselx2::Fcclkselx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx2;
#[doc = "FCCLKSEL3 (rw) register accessor: Flexcomm Interface 3 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel3`]
module"]
#[doc(alias = "FCCLKSEL3")]
pub type Fcclksel3 = crate::Reg<fcclksel3::Fcclksel3Spec>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel3;
#[doc = "FCCLKSELX3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx3`]
module"]
#[doc(alias = "FCCLKSELX3")]
pub type Fcclkselx3 = crate::Reg<fcclkselx3::Fcclkselx3Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx3;
#[doc = "FCCLKSEL4 (rw) register accessor: Flexcomm Interface 4 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel4`]
module"]
#[doc(alias = "FCCLKSEL4")]
pub type Fcclksel4 = crate::Reg<fcclksel4::Fcclksel4Spec>;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel4;
#[doc = "FCCLKSELX4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx4`]
module"]
#[doc(alias = "FCCLKSELX4")]
pub type Fcclkselx4 = crate::Reg<fcclkselx4::Fcclkselx4Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx4;
#[doc = "FCCLKSEL5 (rw) register accessor: Flexcomm Interface 5 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel5`]
module"]
#[doc(alias = "FCCLKSEL5")]
pub type Fcclksel5 = crate::Reg<fcclksel5::Fcclksel5Spec>;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel5;
#[doc = "FCCLKSELX5 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx5`]
module"]
#[doc(alias = "FCCLKSELX5")]
pub type Fcclkselx5 = crate::Reg<fcclkselx5::Fcclkselx5Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx5;
#[doc = "FCCLKSEL6 (rw) register accessor: Flexcomm Interface 6 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel6`]
module"]
#[doc(alias = "FCCLKSEL6")]
pub type Fcclksel6 = crate::Reg<fcclksel6::Fcclksel6Spec>;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel6;
#[doc = "FCCLKSELX6 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx6`]
module"]
#[doc(alias = "FCCLKSELX6")]
pub type Fcclkselx6 = crate::Reg<fcclkselx6::Fcclkselx6Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx6;
#[doc = "FCCLKSEL7 (rw) register accessor: Flexcomm Interface 7 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel7`]
module"]
#[doc(alias = "FCCLKSEL7")]
pub type Fcclksel7 = crate::Reg<fcclksel7::Fcclksel7Spec>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel7;
#[doc = "FCCLKSELX7 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclkselx7`]
module"]
#[doc(alias = "FCCLKSELX7")]
pub type Fcclkselx7 = crate::Reg<fcclkselx7::Fcclkselx7Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclkselx7;
#[doc = "HSLSPICLKSEL (rw) register accessor: HS LSPI clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`hslspiclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslspiclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hslspiclksel`]
module"]
#[doc(alias = "HSLSPICLKSEL")]
pub type Hslspiclksel = crate::Reg<hslspiclksel::HslspiclkselSpec>;
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLKCLKSEL (rw) register accessor: MCLK clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkclksel`]
module"]
#[doc(alias = "MCLKCLKSEL")]
pub type Mclkclksel = crate::Reg<mclkclksel::MclkclkselSpec>;
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTCLKSEL (rw) register accessor: SCTimer/PWM clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`sctclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctclksel`]
module"]
#[doc(alias = "SCTCLKSEL")]
pub type Sctclksel = crate::Reg<sctclksel::SctclkselSpec>;
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIOCLKSEL (rw) register accessor: SDIO clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclksel`]
module"]
#[doc(alias = "SDIOCLKSEL")]
pub type Sdioclksel = crate::Reg<sdioclksel::SdioclkselSpec>;
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICKCLKDIV0 (rw) register accessor: System Tick Timer divider for CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkdiv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkdiv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkdiv0`]
module"]
#[doc(alias = "SYSTICKCLKDIV0")]
pub type Systickclkdiv0 = crate::Reg<systickclkdiv0::Systickclkdiv0Spec>;
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "SYSTICKCLKDIV1 (rw) register accessor: System Tick Timer divider for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkdiv1`]
module"]
#[doc(alias = "SYSTICKCLKDIV1")]
pub type Systickclkdiv1 = crate::Reg<systickclkdiv1::Systickclkdiv1Spec>;
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACECLKDIV (rw) register accessor: TRACE clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclkdiv`]
module"]
#[doc(alias = "TRACECLKDIV")]
pub type Traceclkdiv = crate::Reg<traceclkdiv::TraceclkdivSpec>;
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "FLEXFRG0CTRL (rw) register accessor: Fractional rate divider for flexcomm 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg0ctrl`]
module"]
#[doc(alias = "FLEXFRG0CTRL")]
pub type Flexfrg0ctrl = crate::Reg<flexfrg0ctrl::Flexfrg0ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrg0ctrl;
#[doc = "FLEXFRGXCTRL0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl0`]
module"]
#[doc(alias = "FLEXFRGXCTRL0")]
pub type Flexfrgxctrl0 = crate::Reg<flexfrgxctrl0::Flexfrgxctrl0Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl0;
#[doc = "FLEXFRG1CTRL (rw) register accessor: Fractional rate divider for flexcomm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg1ctrl`]
module"]
#[doc(alias = "FLEXFRG1CTRL")]
pub type Flexfrg1ctrl = crate::Reg<flexfrg1ctrl::Flexfrg1ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrg1ctrl;
#[doc = "FLEXFRGXCTRL1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl1`]
module"]
#[doc(alias = "FLEXFRGXCTRL1")]
pub type Flexfrgxctrl1 = crate::Reg<flexfrgxctrl1::Flexfrgxctrl1Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl1;
#[doc = "FLEXFRG2CTRL (rw) register accessor: Fractional rate divider for flexcomm 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg2ctrl`]
module"]
#[doc(alias = "FLEXFRG2CTRL")]
pub type Flexfrg2ctrl = crate::Reg<flexfrg2ctrl::Flexfrg2ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrg2ctrl;
#[doc = "FLEXFRGXCTRL2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl2`]
module"]
#[doc(alias = "FLEXFRGXCTRL2")]
pub type Flexfrgxctrl2 = crate::Reg<flexfrgxctrl2::Flexfrgxctrl2Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl2;
#[doc = "FLEXFRG3CTRL (rw) register accessor: Fractional rate divider for flexcomm 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg3ctrl`]
module"]
#[doc(alias = "FLEXFRG3CTRL")]
pub type Flexfrg3ctrl = crate::Reg<flexfrg3ctrl::Flexfrg3ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrg3ctrl;
#[doc = "FLEXFRGXCTRL3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl3`]
module"]
#[doc(alias = "FLEXFRGXCTRL3")]
pub type Flexfrgxctrl3 = crate::Reg<flexfrgxctrl3::Flexfrgxctrl3Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl3;
#[doc = "FLEXFRG4CTRL (rw) register accessor: Fractional rate divider for flexcomm 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg4ctrl`]
module"]
#[doc(alias = "FLEXFRG4CTRL")]
pub type Flexfrg4ctrl = crate::Reg<flexfrg4ctrl::Flexfrg4ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrg4ctrl;
#[doc = "FLEXFRGXCTRL4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl4`]
module"]
#[doc(alias = "FLEXFRGXCTRL4")]
pub type Flexfrgxctrl4 = crate::Reg<flexfrgxctrl4::Flexfrgxctrl4Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl4;
#[doc = "FLEXFRG5CTRL (rw) register accessor: Fractional rate divider for flexcomm 5\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg5ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg5ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg5ctrl`]
module"]
#[doc(alias = "FLEXFRG5CTRL")]
pub type Flexfrg5ctrl = crate::Reg<flexfrg5ctrl::Flexfrg5ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrg5ctrl;
#[doc = "FLEXFRGXCTRL5 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl5`]
module"]
#[doc(alias = "FLEXFRGXCTRL5")]
pub type Flexfrgxctrl5 = crate::Reg<flexfrgxctrl5::Flexfrgxctrl5Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl5;
#[doc = "FLEXFRG6CTRL (rw) register accessor: Fractional rate divider for flexcomm 6\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg6ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg6ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg6ctrl`]
module"]
#[doc(alias = "FLEXFRG6CTRL")]
pub type Flexfrg6ctrl = crate::Reg<flexfrg6ctrl::Flexfrg6ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrg6ctrl;
#[doc = "FLEXFRGXCTRL6 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl6`]
module"]
#[doc(alias = "FLEXFRGXCTRL6")]
pub type Flexfrgxctrl6 = crate::Reg<flexfrgxctrl6::Flexfrgxctrl6Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl6;
#[doc = "FLEXFRG7CTRL (rw) register accessor: Fractional rate divider for flexcomm 7\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg7ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg7ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrg7ctrl`]
module"]
#[doc(alias = "FLEXFRG7CTRL")]
pub type Flexfrg7ctrl = crate::Reg<flexfrg7ctrl::Flexfrg7ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrg7ctrl;
#[doc = "FLEXFRGXCTRL7 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgxctrl7`]
module"]
#[doc(alias = "FLEXFRGXCTRL7")]
pub type Flexfrgxctrl7 = crate::Reg<flexfrgxctrl7::Flexfrgxctrl7Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgxctrl7;
#[doc = "AHBCLKDIV (rw) register accessor: System clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkdiv`]
module"]
#[doc(alias = "AHBCLKDIV")]
pub type Ahbclkdiv = crate::Reg<ahbclkdiv::AhbclkdivSpec>;
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUTDIV (rw) register accessor: CLKOUT clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutdiv`]
module"]
#[doc(alias = "CLKOUTDIV")]
pub type Clkoutdiv = crate::Reg<clkoutdiv::ClkoutdivSpec>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHFDIV (rw) register accessor: FRO_HF (96MHz) clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`frohfdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frohfdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frohfdiv`]
module"]
#[doc(alias = "FROHFDIV")]
pub type Frohfdiv = crate::Reg<frohfdiv::FrohfdivSpec>;
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDTCLKDIV (rw) register accessor: WDT clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkdiv`]
module"]
#[doc(alias = "WDTCLKDIV")]
pub type Wdtclkdiv = crate::Reg<wdtclkdiv::WdtclkdivSpec>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADCCLKDIV (rw) register accessor: ADC clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`adcclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclkdiv`]
module"]
#[doc(alias = "ADCCLKDIV")]
pub type Adcclkdiv = crate::Reg<adcclkdiv::AdcclkdivSpec>;
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0CLKDIV (rw) register accessor: USB0 Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0clkdiv`]
module"]
#[doc(alias = "USB0CLKDIV")]
pub type Usb0clkdiv = crate::Reg<usb0clkdiv::Usb0clkdivSpec>;
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "MCLKDIV (rw) register accessor: I2S MCLK clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkdiv`]
module"]
#[doc(alias = "MCLKDIV")]
pub type Mclkdiv = crate::Reg<mclkdiv::MclkdivSpec>;
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCTCLKDIV (rw) register accessor: SCT/PWM clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sctclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctclkdiv`]
module"]
#[doc(alias = "SCTCLKDIV")]
pub type Sctclkdiv = crate::Reg<sctclkdiv::SctclkdivSpec>;
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIOCLKDIV (rw) register accessor: SDIO clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclkdiv`]
module"]
#[doc(alias = "SDIOCLKDIV")]
pub type Sdioclkdiv = crate::Reg<sdioclkdiv::SdioclkdivSpec>;
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0CLKDIV (rw) register accessor: PLL0 clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0clkdiv`]
module"]
#[doc(alias = "PLL0CLKDIV")]
pub type Pll0clkdiv = crate::Reg<pll0clkdiv::Pll0clkdivSpec>;
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "CLOCKGENUPDATELOCKOUT (rw) register accessor: Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`clockgenupdatelockout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockgenupdatelockout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockgenupdatelockout`]
module"]
#[doc(alias = "CLOCKGENUPDATELOCKOUT")]
pub type Clockgenupdatelockout =
    crate::Reg<clockgenupdatelockout::ClockgenupdatelockoutSpec>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMCCR (rw) register accessor: FMC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmccr`]
module"]
#[doc(alias = "FMCCR")]
pub type Fmccr = crate::Reg<fmccr::FmccrSpec>;
#[doc = "FMC configuration register"]
pub mod fmccr;
#[doc = "USB0NEEDCLKCTRL (rw) register accessor: USB0 need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0needclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0needclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0needclkctrl`]
module"]
#[doc(alias = "USB0NEEDCLKCTRL")]
pub type Usb0needclkctrl = crate::Reg<usb0needclkctrl::Usb0needclkctrlSpec>;
#[doc = "USB0 need clock control"]
pub mod usb0needclkctrl;
#[doc = "USB0NEEDCLKSTAT (rw) register accessor: USB0 need clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0needclkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0needclkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0needclkstat`]
module"]
#[doc(alias = "USB0NEEDCLKSTAT")]
pub type Usb0needclkstat = crate::Reg<usb0needclkstat::Usb0needclkstatSpec>;
#[doc = "USB0 need clock status"]
pub mod usb0needclkstat;
#[doc = "FMCFLUSH (w) register accessor: FMCflush control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcflush::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcflush`]
module"]
#[doc(alias = "FMCFLUSH")]
pub type Fmcflush = crate::Reg<fmcflush::FmcflushSpec>;
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLKIO (rw) register accessor: MCLK control\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkio`]
module"]
#[doc(alias = "MCLKIO")]
pub type Mclkio = crate::Reg<mclkio::MclkioSpec>;
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1NEEDCLKCTRL (rw) register accessor: USB1 need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1needclkctrl`]
module"]
#[doc(alias = "USB1NEEDCLKCTRL")]
pub type Usb1needclkctrl = crate::Reg<usb1needclkctrl::Usb1needclkctrlSpec>;
#[doc = "USB1 need clock control"]
pub mod usb1needclkctrl;
#[doc = "USB1NEEDCLKSTAT (rw) register accessor: USB1 need clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1needclkstat`]
module"]
#[doc(alias = "USB1NEEDCLKSTAT")]
pub type Usb1needclkstat = crate::Reg<usb1needclkstat::Usb1needclkstatSpec>;
#[doc = "USB1 need clock status"]
pub mod usb1needclkstat;
#[doc = "SDIOCLKCTRL (rw) register accessor: SDIO CCLKIN phase and delay control\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclkctrl`]
module"]
#[doc(alias = "SDIOCLKCTRL")]
pub type Sdioclkctrl = crate::Reg<sdioclkctrl::SdioclkctrlSpec>;
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1CTRL (rw) register accessor: PLL1 550m control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1ctrl`]
module"]
#[doc(alias = "PLL1CTRL")]
pub type Pll1ctrl = crate::Reg<pll1ctrl::Pll1ctrlSpec>;
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1STAT (rw) register accessor: PLL1 550m status\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1stat`]
module"]
#[doc(alias = "PLL1STAT")]
pub type Pll1stat = crate::Reg<pll1stat::Pll1statSpec>;
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1NDEC (rw) register accessor: PLL1 550m N divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1ndec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1ndec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1ndec`]
module"]
#[doc(alias = "PLL1NDEC")]
pub type Pll1ndec = crate::Reg<pll1ndec::Pll1ndecSpec>;
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1MDEC (rw) register accessor: PLL1 550m M divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1mdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1mdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1mdec`]
module"]
#[doc(alias = "PLL1MDEC")]
pub type Pll1mdec = crate::Reg<pll1mdec::Pll1mdecSpec>;
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1PDEC (rw) register accessor: PLL1 550m P divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1pdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1pdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1pdec`]
module"]
#[doc(alias = "PLL1PDEC")]
pub type Pll1pdec = crate::Reg<pll1pdec::Pll1pdecSpec>;
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0CTRL (rw) register accessor: PLL0 550m control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0ctrl`]
module"]
#[doc(alias = "PLL0CTRL")]
pub type Pll0ctrl = crate::Reg<pll0ctrl::Pll0ctrlSpec>;
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0STAT (rw) register accessor: PLL0 550m status\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0stat`]
module"]
#[doc(alias = "PLL0STAT")]
pub type Pll0stat = crate::Reg<pll0stat::Pll0statSpec>;
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0NDEC (rw) register accessor: PLL0 550m N divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ndec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ndec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0ndec`]
module"]
#[doc(alias = "PLL0NDEC")]
pub type Pll0ndec = crate::Reg<pll0ndec::Pll0ndecSpec>;
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0PDEC (rw) register accessor: PLL0 550m P divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0pdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0pdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0pdec`]
module"]
#[doc(alias = "PLL0PDEC")]
pub type Pll0pdec = crate::Reg<pll0pdec::Pll0pdecSpec>;
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0SSCG0 (rw) register accessor: PLL0 Spread Spectrum Wrapper control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0sscg0`]
module"]
#[doc(alias = "PLL0SSCG0")]
pub type Pll0sscg0 = crate::Reg<pll0sscg0::Pll0sscg0Spec>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0SSCG1 (rw) register accessor: PLL0 Spread Spectrum Wrapper control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0sscg1`]
module"]
#[doc(alias = "PLL0SSCG1")]
pub type Pll0sscg1 = crate::Reg<pll0sscg1::Pll0sscg1Spec>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "FUNCRETENTIONCTRL (rw) register accessor: Functional retention control register\n\nYou can [`read`](crate::Reg::read) this register and get [`funcretentionctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`funcretentionctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@funcretentionctrl`]
module"]
#[doc(alias = "FUNCRETENTIONCTRL")]
pub type Funcretentionctrl = crate::Reg<funcretentionctrl::FuncretentionctrlSpec>;
#[doc = "Functional retention control register"]
pub mod funcretentionctrl;
#[doc = "CPUCTRL (rw) register accessor: CPU Control for multiple processors\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuctrl`]
module"]
#[doc(alias = "CPUCTRL")]
pub type Cpuctrl = crate::Reg<cpuctrl::CpuctrlSpec>;
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "CPBOOT (rw) register accessor: Coprocessor Boot Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpboot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpboot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpboot`]
module"]
#[doc(alias = "CPBOOT")]
pub type Cpboot = crate::Reg<cpboot::CpbootSpec>;
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "CPSTAT (rw) register accessor: CPU Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpstat`]
module"]
#[doc(alias = "CPSTAT")]
pub type Cpstat = crate::Reg<cpstat::CpstatSpec>;
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "CLOCK_CTRL (rw) register accessor: Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "COMP_INT_CTRL (rw) register accessor: Comparator Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_int_ctrl`]
module"]
#[doc(alias = "COMP_INT_CTRL")]
pub type CompIntCtrl = crate::Reg<comp_int_ctrl::CompIntCtrlSpec>;
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "COMP_INT_STATUS (rw) register accessor: Comparator Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_int_status`]
module"]
#[doc(alias = "COMP_INT_STATUS")]
pub type CompIntStatus = crate::Reg<comp_int_status::CompIntStatusSpec>;
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "AUTOCLKGATEOVERRIDE (rw) register accessor: Control automatic clock gating\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoclkgateoverride`]
module"]
#[doc(alias = "AUTOCLKGATEOVERRIDE")]
pub type Autoclkgateoverride = crate::Reg<autoclkgateoverride::AutoclkgateoverrideSpec>;
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "GPIOPSYNC (rw) register accessor: Enable bypass of the first stage of synchonization inside GPIO_INT module\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiopsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiopsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiopsync`]
module"]
#[doc(alias = "GPIOPSYNC")]
pub type Gpiopsync = crate::Reg<gpiopsync::GpiopsyncSpec>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "DEBUG_LOCK_EN (rw) register accessor: Control write access to security registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_lock_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_lock_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_lock_en`]
module"]
#[doc(alias = "DEBUG_LOCK_EN")]
pub type DebugLockEn = crate::Reg<debug_lock_en::DebugLockEnSpec>;
#[doc = "Control write access to security registers."]
pub mod debug_lock_en;
#[doc = "DEBUG_FEATURES (rw) register accessor: Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_features::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_features::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_features`]
module"]
#[doc(alias = "DEBUG_FEATURES")]
pub type DebugFeatures = crate::Reg<debug_features::DebugFeaturesSpec>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
pub mod debug_features;
#[doc = "DEBUG_FEATURES_DP (rw) register accessor: Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_features_dp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_features_dp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_features_dp`]
module"]
#[doc(alias = "DEBUG_FEATURES_DP")]
pub type DebugFeaturesDp = crate::Reg<debug_features_dp::DebugFeaturesDpSpec>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
pub mod debug_features_dp;
#[doc = "KEY_BLOCK (w) register accessor: block quiddikey/PUF all index.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_block`]
module"]
#[doc(alias = "KEY_BLOCK")]
pub type KeyBlock = crate::Reg<key_block::KeyBlockSpec>;
#[doc = "block quiddikey/PUF all index."]
pub mod key_block;
#[doc = "DEBUG_AUTH_BEACON (rw) register accessor: Debug authentication BEACON register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_auth_beacon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_auth_beacon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_auth_beacon`]
module"]
#[doc(alias = "DEBUG_AUTH_BEACON")]
pub type DebugAuthBeacon = crate::Reg<debug_auth_beacon::DebugAuthBeaconSpec>;
#[doc = "Debug authentication BEACON register"]
pub mod debug_auth_beacon;
#[doc = "CPUCFG (rw) register accessor: CPUs configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucfg`]
module"]
#[doc(alias = "CPUCFG")]
pub type Cpucfg = crate::Reg<cpucfg::CpucfgSpec>;
#[doc = "CPUs configuration register"]
pub mod cpucfg;
#[doc = "DEVICE_ID0 (r) register accessor: Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id0`]
module"]
#[doc(alias = "DEVICE_ID0")]
pub type DeviceId0 = crate::Reg<device_id0::DeviceId0Spec>;
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "DIEID (r) register accessor: Chip revision ID and Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dieid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieid`]
module"]
#[doc(alias = "DIEID")]
pub type Dieid = crate::Reg<dieid::DieidSpec>;
#[doc = "Chip revision ID and Number"]
pub mod dieid;
