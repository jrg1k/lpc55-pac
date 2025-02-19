#[doc = "Register `VALUE1` reader"]
pub type R = crate::R<Value1Spec>;
#[doc = "Register `VALUE1` writer"]
pub type W = crate::W<Value1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Value replacement 1\n\nYou can [`read`](crate::Reg::read) this register and get [`value1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Value1Spec;
impl crate::RegisterSpec for Value1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value1::R`](R) reader structure"]
impl crate::Readable for Value1Spec {}
#[doc = "`write(|w| ..)` method takes [`value1::W`](W) writer structure"]
impl crate::Writable for Value1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE1 to value 0"]
impl crate::Resettable for Value1Spec {
    const RESET_VALUE: u32 = 0;
}
