#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `PATCH0` reader - Patch 0 control bit"]
pub type Patch0R = crate::BitReader;
#[doc = "Field `PATCH0` writer - Patch 0 control bit"]
pub type Patch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH1` reader - Patch 1 control bit"]
pub type Patch1R = crate::BitReader;
#[doc = "Field `PATCH1` writer - Patch 1 control bit"]
pub type Patch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH2` reader - Patch 2 control bit"]
pub type Patch2R = crate::BitReader;
#[doc = "Field `PATCH2` writer - Patch 2 control bit"]
pub type Patch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH3` reader - Patch 3 control bit"]
pub type Patch3R = crate::BitReader;
#[doc = "Field `PATCH3` writer - Patch 3 control bit"]
pub type Patch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH4` reader - Patch 4 control bit"]
pub type Patch4R = crate::BitReader;
#[doc = "Field `PATCH4` writer - Patch 4 control bit"]
pub type Patch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH5` reader - Patch 5 control bit"]
pub type Patch5R = crate::BitReader;
#[doc = "Field `PATCH5` writer - Patch 5 control bit"]
pub type Patch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH6` reader - Patch 6 control bit"]
pub type Patch6R = crate::BitReader;
#[doc = "Field `PATCH6` writer - Patch 6 control bit"]
pub type Patch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH7` reader - Patch 7 control bit"]
pub type Patch7R = crate::BitReader;
#[doc = "Field `PATCH7` writer - Patch 7 control bit"]
pub type Patch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH8` reader - Patch 8 control bit"]
pub type Patch8R = crate::BitReader;
#[doc = "Field `PATCH8` writer - Patch 8 control bit"]
pub type Patch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH9` reader - Patch 9 control bit"]
pub type Patch9R = crate::BitReader;
#[doc = "Field `PATCH9` writer - Patch 9 control bit"]
pub type Patch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH10` reader - Patch 10 control bit"]
pub type Patch10R = crate::BitReader;
#[doc = "Field `PATCH10` writer - Patch 10 control bit"]
pub type Patch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH11` reader - Patch 11 control bit"]
pub type Patch11R = crate::BitReader;
#[doc = "Field `PATCH11` writer - Patch 11 control bit"]
pub type Patch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH12` reader - Patch 12 control bit"]
pub type Patch12R = crate::BitReader;
#[doc = "Field `PATCH12` writer - Patch 12 control bit"]
pub type Patch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH13` reader - Patch 13 control bit"]
pub type Patch13R = crate::BitReader;
#[doc = "Field `PATCH13` writer - Patch 13 control bit"]
pub type Patch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH14` reader - Patch 14 control bit"]
pub type Patch14R = crate::BitReader;
#[doc = "Field `PATCH14` writer - Patch 14 control bit"]
pub type Patch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PATCH15` reader - Patch 15 control bit"]
pub type Patch15R = crate::BitReader;
#[doc = "Field `PATCH15` writer - Patch 15 control bit"]
pub type Patch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE` reader - disable bit"]
pub type DisableR = crate::BitReader;
#[doc = "Field `DISABLE` writer - disable bit"]
pub type DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Patch 0 control bit"]
    #[inline(always)]
    pub fn patch0(&self) -> Patch0R {
        Patch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Patch 1 control bit"]
    #[inline(always)]
    pub fn patch1(&self) -> Patch1R {
        Patch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Patch 2 control bit"]
    #[inline(always)]
    pub fn patch2(&self) -> Patch2R {
        Patch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Patch 3 control bit"]
    #[inline(always)]
    pub fn patch3(&self) -> Patch3R {
        Patch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Patch 4 control bit"]
    #[inline(always)]
    pub fn patch4(&self) -> Patch4R {
        Patch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Patch 5 control bit"]
    #[inline(always)]
    pub fn patch5(&self) -> Patch5R {
        Patch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Patch 6 control bit"]
    #[inline(always)]
    pub fn patch6(&self) -> Patch6R {
        Patch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Patch 7 control bit"]
    #[inline(always)]
    pub fn patch7(&self) -> Patch7R {
        Patch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Patch 8 control bit"]
    #[inline(always)]
    pub fn patch8(&self) -> Patch8R {
        Patch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Patch 9 control bit"]
    #[inline(always)]
    pub fn patch9(&self) -> Patch9R {
        Patch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Patch 10 control bit"]
    #[inline(always)]
    pub fn patch10(&self) -> Patch10R {
        Patch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Patch 11 control bit"]
    #[inline(always)]
    pub fn patch11(&self) -> Patch11R {
        Patch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Patch 12 control bit"]
    #[inline(always)]
    pub fn patch12(&self) -> Patch12R {
        Patch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Patch 13 control bit"]
    #[inline(always)]
    pub fn patch13(&self) -> Patch13R {
        Patch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Patch 14 control bit"]
    #[inline(always)]
    pub fn patch14(&self) -> Patch14R {
        Patch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Patch 15 control bit"]
    #[inline(always)]
    pub fn patch15(&self) -> Patch15R {
        Patch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - disable bit"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Patch 0 control bit"]
    #[inline(always)]
    pub fn patch0(&mut self) -> Patch0W<ControlSpec> {
        Patch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Patch 1 control bit"]
    #[inline(always)]
    pub fn patch1(&mut self) -> Patch1W<ControlSpec> {
        Patch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Patch 2 control bit"]
    #[inline(always)]
    pub fn patch2(&mut self) -> Patch2W<ControlSpec> {
        Patch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Patch 3 control bit"]
    #[inline(always)]
    pub fn patch3(&mut self) -> Patch3W<ControlSpec> {
        Patch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Patch 4 control bit"]
    #[inline(always)]
    pub fn patch4(&mut self) -> Patch4W<ControlSpec> {
        Patch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Patch 5 control bit"]
    #[inline(always)]
    pub fn patch5(&mut self) -> Patch5W<ControlSpec> {
        Patch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Patch 6 control bit"]
    #[inline(always)]
    pub fn patch6(&mut self) -> Patch6W<ControlSpec> {
        Patch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Patch 7 control bit"]
    #[inline(always)]
    pub fn patch7(&mut self) -> Patch7W<ControlSpec> {
        Patch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Patch 8 control bit"]
    #[inline(always)]
    pub fn patch8(&mut self) -> Patch8W<ControlSpec> {
        Patch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Patch 9 control bit"]
    #[inline(always)]
    pub fn patch9(&mut self) -> Patch9W<ControlSpec> {
        Patch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Patch 10 control bit"]
    #[inline(always)]
    pub fn patch10(&mut self) -> Patch10W<ControlSpec> {
        Patch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Patch 11 control bit"]
    #[inline(always)]
    pub fn patch11(&mut self) -> Patch11W<ControlSpec> {
        Patch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Patch 12 control bit"]
    #[inline(always)]
    pub fn patch12(&mut self) -> Patch12W<ControlSpec> {
        Patch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Patch 13 control bit"]
    #[inline(always)]
    pub fn patch13(&mut self) -> Patch13W<ControlSpec> {
        Patch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Patch 14 control bit"]
    #[inline(always)]
    pub fn patch14(&mut self) -> Patch14W<ControlSpec> {
        Patch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Patch 15 control bit"]
    #[inline(always)]
    pub fn patch15(&mut self) -> Patch15W<ControlSpec> {
        Patch15W::new(self, 15)
    }
    #[doc = "Bit 29 - disable bit"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<ControlSpec> {
        DisableW::new(self, 29)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
