#[doc = "Register `UDS_BODY6` reader"]
pub type R = crate::R<UdsBody6Spec>;
#[doc = "Register `UDS_BODY6` writer"]
pub type W = crate::W<UdsBody6Spec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<UdsBody6Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdsBody6Spec;
impl crate::RegisterSpec for UdsBody6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uds_body6::R`](R) reader structure"]
impl crate::Readable for UdsBody6Spec {}
#[doc = "`write(|w| ..)` method takes [`uds_body6::W`](W) writer structure"]
impl crate::Writable for UdsBody6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDS_BODY6 to value 0"]
impl crate::Resettable for UdsBody6Spec {
    const RESET_VALUE: u32 = 0;
}
