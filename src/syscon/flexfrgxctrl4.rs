#[doc = "Register `FLEXFRGXCTRL4` reader"]
pub type R = crate::R<Flexfrgxctrl4Spec>;
#[doc = "Register `FLEXFRGXCTRL4` writer"]
pub type W = crate::W<Flexfrgxctrl4Spec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Flexfrgxctrl4Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgxctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgxctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flexfrgxctrl4Spec;
impl crate::RegisterSpec for Flexfrgxctrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrgxctrl4::R`](R) reader structure"]
impl crate::Readable for Flexfrgxctrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`flexfrgxctrl4::W`](W) writer structure"]
impl crate::Writable for Flexfrgxctrl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLEXFRGXCTRL4 to value 0"]
impl crate::Resettable for Flexfrgxctrl4Spec {
    const RESET_VALUE: u32 = 0;
}
