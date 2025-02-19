#[doc = "Register `VALUE3` reader"]
pub type R = crate::R<Value3Spec>;
#[doc = "Register `VALUE3` writer"]
pub type W = crate::W<Value3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Value replacement 3\n\nYou can [`read`](crate::Reg::read) this register and get [`value3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Value3Spec;
impl crate::RegisterSpec for Value3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value3::R`](R) reader structure"]
impl crate::Readable for Value3Spec {}
#[doc = "`write(|w| ..)` method takes [`value3::W`](W) writer structure"]
impl crate::Writable for Value3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE3 to value 0"]
impl crate::Resettable for Value3Spec {
    const RESET_VALUE: u32 = 0;
}
