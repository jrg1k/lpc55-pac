#[doc = "Register `ADCCLKSEL` reader"]
pub type R = crate::R<AdcclkselSpec>;
#[doc = "Register `ADCCLKSEL` writer"]
pub type W = crate::W<AdcclkselSpec>;
#[doc = "ADC clock source select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Main clk."]
    Mainclk = 0,
    #[doc = "1: PLL0 clk."]
    Pll0 = 1,
    #[doc = "2: FRO 96 MHZ clk."]
    Fro96 = 2,
    #[doc = "4: No clk."]
    None = 4,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - ADC clock source select"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Mainclk),
            1 => Some(Sel::Pll0),
            2 => Some(Sel::Fro96),
            4 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Main clk."]
    #[inline(always)]
    pub fn is_mainclk(&self) -> bool {
        *self == Sel::Mainclk
    }
    #[doc = "PLL0 clk."]
    #[inline(always)]
    pub fn is_pll0(&self) -> bool {
        *self == Sel::Pll0
    }
    #[doc = "FRO 96 MHZ clk."]
    #[inline(always)]
    pub fn is_fro96(&self) -> bool {
        *self == Sel::Fro96
    }
    #[doc = "No clk."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - ADC clock source select"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main clk."]
    #[inline(always)]
    pub fn mainclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Mainclk)
    }
    #[doc = "PLL0 clk."]
    #[inline(always)]
    pub fn pll0(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Pll0)
    }
    #[doc = "FRO 96 MHZ clk."]
    #[inline(always)]
    pub fn fro96(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Fro96)
    }
    #[doc = "No clk."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC clock source select"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC clock source select"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<AdcclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "ADC clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`adcclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcclkselSpec;
impl crate::RegisterSpec for AdcclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcclksel::R`](R) reader structure"]
impl crate::Readable for AdcclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcclksel::W`](W) writer structure"]
impl crate::Writable for AdcclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCLKSEL to value 0x07"]
impl crate::Resettable for AdcclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}
