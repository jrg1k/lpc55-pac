#[doc = "Register `DMA0_ITRIG_INMUX[%s]` reader"]
pub type R = crate::R<Dma0ItrigInmuxSpec>;
#[doc = "Register `DMA0_ITRIG_INMUX[%s]` writer"]
pub type W = crate::W<Dma0ItrigInmuxSpec>;
#[doc = "Field `INP` reader - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpR = crate::FieldReader;
#[doc = "Field `INP` writer - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
pub type InpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&self) -> InpR {
        InpR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub fn inp(&mut self) -> InpW<Dma0ItrigInmuxSpec> {
        InpW::new(self, 0)
    }
}
#[doc = "Trigger select register for DMA0 channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dma0_itrig_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma0_itrig_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma0ItrigInmuxSpec;
impl crate::RegisterSpec for Dma0ItrigInmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma0_itrig_inmux::R`](R) reader structure"]
impl crate::Readable for Dma0ItrigInmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`dma0_itrig_inmux::W`](W) writer structure"]
impl crate::Writable for Dma0ItrigInmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA0_ITRIG_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for Dma0ItrigInmuxSpec {
    const RESET_VALUE: u32 = 0x1f;
}
