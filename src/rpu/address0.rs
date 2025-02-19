#[doc = "Register `ADDRESS0` reader"]
pub type R = crate::R<Address0Spec>;
#[doc = "Register `ADDRESS0` writer"]
pub type W = crate::W<Address0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Replacement address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`address0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Address0Spec;
impl crate::RegisterSpec for Address0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address0::R`](R) reader structure"]
impl crate::Readable for Address0Spec {}
#[doc = "`write(|w| ..)` method takes [`address0::W`](W) writer structure"]
impl crate::Writable for Address0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS0 to value 0"]
impl crate::Resettable for Address0Spec {
    const RESET_VALUE: u32 = 0;
}
