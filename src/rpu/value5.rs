#[doc = "Register `VALUE5` reader"]
pub type R = crate::R<Value5Spec>;
#[doc = "Register `VALUE5` writer"]
pub type W = crate::W<Value5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Value replacement 5\n\nYou can [`read`](crate::Reg::read) this register and get [`value5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Value5Spec;
impl crate::RegisterSpec for Value5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value5::R`](R) reader structure"]
impl crate::Readable for Value5Spec {}
#[doc = "`write(|w| ..)` method takes [`value5::W`](W) writer structure"]
impl crate::Writable for Value5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE5 to value 0"]
impl crate::Resettable for Value5Spec {
    const RESET_VALUE: u32 = 0;
}
