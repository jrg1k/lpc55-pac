#[doc = "Register `FLEXFRG6CTRL` reader"]
pub type R = crate::R<Flexfrg6ctrlSpec>;
#[doc = "Register `FLEXFRG6CTRL` writer"]
pub type W = crate::W<Flexfrg6ctrlSpec>;
#[doc = "Field `DIV` reader - Denominator of the fractional rate divider."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Denominator of the fractional rate divider."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULT` reader - Numerator of the fractional rate divider."]
pub type MultR = crate::FieldReader;
#[doc = "Field `MULT` writer - Numerator of the fractional rate divider."]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<Flexfrg6ctrlSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&mut self) -> MultW<Flexfrg6ctrlSpec> {
        MultW::new(self, 8)
    }
}
#[doc = "Fractional rate divider for flexcomm 6\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrg6ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrg6ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flexfrg6ctrlSpec;
impl crate::RegisterSpec for Flexfrg6ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flexfrg6ctrl::R`](R) reader structure"]
impl crate::Readable for Flexfrg6ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flexfrg6ctrl::W`](W) writer structure"]
impl crate::Writable for Flexfrg6ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLEXFRG6CTRL to value 0xff"]
impl crate::Resettable for Flexfrg6ctrlSpec {
    const RESET_VALUE: u32 = 0xff;
}
