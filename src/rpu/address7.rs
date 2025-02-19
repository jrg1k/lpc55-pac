#[doc = "Register `ADDRESS7` reader"]
pub type R = crate::R<Address7Spec>;
#[doc = "Register `ADDRESS7` writer"]
pub type W = crate::W<Address7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Replacement address 7\n\nYou can [`read`](crate::Reg::read) this register and get [`address7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Address7Spec;
impl crate::RegisterSpec for Address7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`address7::R`](R) reader structure"]
impl crate::Readable for Address7Spec {}
#[doc = "`write(|w| ..)` method takes [`address7::W`](W) writer structure"]
impl crate::Writable for Address7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDRESS7 to value 0"]
impl crate::Resettable for Address7Spec {
    const RESET_VALUE: u32 = 0;
}
