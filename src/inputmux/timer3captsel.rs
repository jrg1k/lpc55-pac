#[doc = "Register `TIMER3CAPTSEL[%s]` reader"]
pub type R = crate::R<Timer3captselSpec>;
#[doc = "Register `TIMER3CAPTSEL[%s]` writer"]
pub type W = crate::W<Timer3captselSpec>;
#[doc = "Field `CAPTSEL` reader - Input number to TIMER%s capture inputs 0 to 4"]
pub type CaptselR = crate::FieldReader;
#[doc = "Field `CAPTSEL` writer - Input number to TIMER%s capture inputs 0 to 4"]
pub type CaptselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CaptselR {
        CaptselR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&mut self) -> CaptselW<Timer3captselSpec> {
        CaptselW::new(self, 0)
    }
}
#[doc = "Capture select registers for TIMER3 inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`timer3captsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer3captsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer3captselSpec;
impl crate::RegisterSpec for Timer3captselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3captsel::R`](R) reader structure"]
impl crate::Readable for Timer3captselSpec {}
#[doc = "`write(|w| ..)` method takes [`timer3captsel::W`](W) writer structure"]
impl crate::Writable for Timer3captselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER3CAPTSEL[%s]
to value 0x1f"]
impl crate::Resettable for Timer3captselSpec {
    const RESET_VALUE: u32 = 0x1f;
}
