#[doc = "Register `VALUE3` reader"]
pub struct R(crate::R<VALUE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VALUE3_SPEC>> for R {
    fn from(reader: crate::R<VALUE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VALUE3` writer"]
pub struct W(crate::W<VALUE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VALUE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<VALUE3_SPEC>> for W {
    fn from(writer: crate::W<VALUE3_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value replacement 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value3](index.html) module"]
pub struct VALUE3_SPEC;
impl crate::RegisterSpec for VALUE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value3::R](R) reader structure"]
impl crate::Readable for VALUE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [value3::W](W) writer structure"]
impl crate::Writable for VALUE3_SPEC {
    type Writer = W;
}
