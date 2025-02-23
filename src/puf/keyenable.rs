#[doc = "Register `KEYENABLE` reader"]
pub type R = crate::R<KeyenableSpec>;
#[doc = "Register `KEYENABLE` writer"]
pub type W = crate::W<KeyenableSpec>;
#[doc = "Field `KEY0` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type Key0R = crate::FieldReader;
#[doc = "Field `KEY0` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type Key0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Key destination for PUF key.\n\nValue on reset: 85"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "85: Do not send key to any hardware engine."]
    None = 85,
    #[doc = "86: Send key to AES engine."]
    Aes = 86,
    #[doc = "89: Send key to PRINCE engine for memory layout 0."]
    Prince0 = 89,
    #[doc = "101: Send key to PRINCE engine for memory layout 1."]
    Prince1 = 101,
    #[doc = "149: Send key to PRINCE engine for memory layout 2."]
    Prince2 = 149,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Key destination for PUF key."]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            85 => Some(Key::None),
            86 => Some(Key::Aes),
            89 => Some(Key::Prince0),
            101 => Some(Key::Prince1),
            149 => Some(Key::Prince2),
            _ => None,
        }
    }
    #[doc = "Do not send key to any hardware engine."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Key::None
    }
    #[doc = "Send key to AES engine."]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == Key::Aes
    }
    #[doc = "Send key to PRINCE engine for memory layout 0."]
    #[inline(always)]
    pub fn is_prince0(&self) -> bool {
        *self == Key::Prince0
    }
    #[doc = "Send key to PRINCE engine for memory layout 1."]
    #[inline(always)]
    pub fn is_prince1(&self) -> bool {
        *self == Key::Prince1
    }
    #[doc = "Send key to PRINCE engine for memory layout 2."]
    #[inline(always)]
    pub fn is_prince2(&self) -> bool {
        *self == Key::Prince2
    }
}
#[doc = "Field `KEY` writer - Key destination for PUF key."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not send key to any hardware engine."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Key::None)
    }
    #[doc = "Send key to AES engine."]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Aes)
    }
    #[doc = "Send key to PRINCE engine for memory layout 0."]
    #[inline(always)]
    pub fn prince0(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Prince0)
    }
    #[doc = "Send key to PRINCE engine for memory layout 1."]
    #[inline(always)]
    pub fn prince1(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Prince1)
    }
    #[doc = "Send key to PRINCE engine for memory layout 2."]
    #[inline(always)]
    pub fn prince2(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Prince2)
    }
}
#[doc = "Field `KEY1` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type Key1R = crate::FieldReader;
#[doc = "Field `KEY1` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type Key1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY2` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type Key2R = crate::FieldReader;
#[doc = "Field `KEY2` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type Key2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEY3` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type Key3R = crate::FieldReader;
#[doc = "Field `KEY3` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type Key3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&self) -> Key0R {
        Key0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&self) -> Key1R {
        Key1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&self) -> Key2R {
        Key2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&self) -> Key3R {
        Key3R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&mut self) -> Key0W<KeyenableSpec> {
        Key0W::new(self, 0)
    }
    #[doc = "Bits 0:7 - Key destination for PUF key."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<KeyenableSpec> {
        KeyW::new(self, 0)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&mut self) -> Key1W<KeyenableSpec> {
        Key1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&mut self) -> Key2W<KeyenableSpec> {
        Key2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&mut self) -> Key3W<KeyenableSpec> {
        Key3W::new(self, 6)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`keyenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyenableSpec;
impl crate::RegisterSpec for KeyenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyenable::R`](R) reader structure"]
impl crate::Readable for KeyenableSpec {}
#[doc = "`write(|w| ..)` method takes [`keyenable::W`](W) writer structure"]
impl crate::Writable for KeyenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KeyenableSpec {
    const RESET_VALUE: u32 = 0x55;
}
