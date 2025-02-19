#[doc = "Register `SCT0_INMUX[%s]` reader"]
pub type R = crate::R<Sct0InmuxSpec>;
#[doc = "Register `SCT0_INMUX[%s]` writer"]
pub type W = crate::W<Sct0InmuxSpec>;
#[doc = "Field `INP_N` reader - Input number to SCT0 inputs 0 to 6."]
pub type InpNR = crate::FieldReader;
#[doc = "Field `INP_N` writer - Input number to SCT0 inputs 0 to 6."]
pub type InpNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6."]
    #[inline(always)]
    pub fn inp_n(&self) -> InpNR {
        InpNR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6."]
    #[inline(always)]
    pub fn inp_n(&mut self) -> InpNW<Sct0InmuxSpec> {
        InpNW::new(self, 0)
    }
}
#[doc = "Input mux register for SCT0 input\n\nYou can [`read`](crate::Reg::read) this register and get [`sct0_inmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sct0_inmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sct0InmuxSpec;
impl crate::RegisterSpec for Sct0InmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sct0_inmux::R`](R) reader structure"]
impl crate::Readable for Sct0InmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`sct0_inmux::W`](W) writer structure"]
impl crate::Writable for Sct0InmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCT0_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for Sct0InmuxSpec {
    const RESET_VALUE: u32 = 0x1f;
}
