#[doc = "Register `FCCLKSELX6` reader"]
pub type R = crate::R<Fcclkselx6Spec>;
#[doc = "Register `FCCLKSELX6` writer"]
pub type W = crate::W<Fcclkselx6Spec>;
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
    pub fn data(&mut self) -> DataW<Fcclkselx6Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclkselx6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclkselx6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcclkselx6Spec;
impl crate::RegisterSpec for Fcclkselx6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcclkselx6::R`](R) reader structure"]
impl crate::Readable for Fcclkselx6Spec {}
#[doc = "`write(|w| ..)` method takes [`fcclkselx6::W`](W) writer structure"]
impl crate::Writable for Fcclkselx6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCLKSELX6 to value 0"]
impl crate::Resettable for Fcclkselx6Spec {
    const RESET_VALUE: u32 = 0;
}
