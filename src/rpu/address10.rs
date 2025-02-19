#[doc = "Register `ADDRESS10` reader"]
pub type R = crate::R<Address10Spec>;
#[doc = "Register `ADDRESS10` writer"]
pub type W = crate::W<Address10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Replacement address 10\n\nYou can [`read`](crate::Reg::read) this register and get [`address10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Address10Spec;
impl crate::RegisterSpec for Address10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address10::R`](R) reader structure"]
impl crate::Readable for Address10Spec {}
#[doc = "`write(|w| ..)` method takes [`address10::W`](W) writer structure"]
impl crate::Writable for Address10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS10 to value 0"]
impl crate::Resettable for Address10Spec {
    const RESET_VALUE: u32 = 0;
}
