#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    value7: Value7,
    value6: Value6,
    value5: Value5,
    value4: Value4,
    value3: Value3,
    value2: Value2,
    value1: Value1,
    value0: Value0,
    control: Control,
    _reserved9: [u8; 0x04],
    enable: Enable,
    address0: Address0,
    address1: Address1,
    address2: Address2,
    address3: Address3,
    address4: Address4,
    address5: Address5,
    address6: Address6,
    address7: Address7,
    address8: Address8,
    address9: Address9,
    address10: Address10,
    address11: Address11,
    address12: Address12,
    address13: Address13,
    address14: Address14,
    address15: Address15,
}
impl RegisterBlock {
    #[doc = "0xd4 - Value replacement 7"]
    #[inline(always)]
    pub const fn value7(&self) -> &Value7 {
        &self.value7
    }
    #[doc = "0xd8 - Value replacement 6"]
    #[inline(always)]
    pub const fn value6(&self) -> &Value6 {
        &self.value6
    }
    #[doc = "0xdc - Value replacement 5"]
    #[inline(always)]
    pub const fn value5(&self) -> &Value5 {
        &self.value5
    }
    #[doc = "0xe0 - Value replacement 4"]
    #[inline(always)]
    pub const fn value4(&self) -> &Value4 {
        &self.value4
    }
    #[doc = "0xe4 - Value replacement 3"]
    #[inline(always)]
    pub const fn value3(&self) -> &Value3 {
        &self.value3
    }
    #[doc = "0xe8 - Value replacement 2"]
    #[inline(always)]
    pub const fn value2(&self) -> &Value2 {
        &self.value2
    }
    #[doc = "0xec - Value replacement 1"]
    #[inline(always)]
    pub const fn value1(&self) -> &Value1 {
        &self.value1
    }
    #[doc = "0xf0 - Value replacement 0"]
    #[inline(always)]
    pub const fn value0(&self) -> &Value0 {
        &self.value0
    }
    #[doc = "0xf4 - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0xfc - Enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x100 - Replacement address 0"]
    #[inline(always)]
    pub const fn address0(&self) -> &Address0 {
        &self.address0
    }
    #[doc = "0x104 - Replacement address 2"]
    #[inline(always)]
    pub const fn address1(&self) -> &Address1 {
        &self.address1
    }
    #[doc = "0x108 - Replacement address 2"]
    #[inline(always)]
    pub const fn address2(&self) -> &Address2 {
        &self.address2
    }
    #[doc = "0x10c - Replacement address 3"]
    #[inline(always)]
    pub const fn address3(&self) -> &Address3 {
        &self.address3
    }
    #[doc = "0x110 - Replacement address 4"]
    #[inline(always)]
    pub const fn address4(&self) -> &Address4 {
        &self.address4
    }
    #[doc = "0x114 - Replacement address 5"]
    #[inline(always)]
    pub const fn address5(&self) -> &Address5 {
        &self.address5
    }
    #[doc = "0x118 - Replacement address 6"]
    #[inline(always)]
    pub const fn address6(&self) -> &Address6 {
        &self.address6
    }
    #[doc = "0x11c - Replacement address 7"]
    #[inline(always)]
    pub const fn address7(&self) -> &Address7 {
        &self.address7
    }
    #[doc = "0x120 - Replacement address 8"]
    #[inline(always)]
    pub const fn address8(&self) -> &Address8 {
        &self.address8
    }
    #[doc = "0x124 - Replacement address 9"]
    #[inline(always)]
    pub const fn address9(&self) -> &Address9 {
        &self.address9
    }
    #[doc = "0x128 - Replacement address 10"]
    #[inline(always)]
    pub const fn address10(&self) -> &Address10 {
        &self.address10
    }
    #[doc = "0x12c - Replacement address 11"]
    #[inline(always)]
    pub const fn address11(&self) -> &Address11 {
        &self.address11
    }
    #[doc = "0x130 - Replacement address 12"]
    #[inline(always)]
    pub const fn address12(&self) -> &Address12 {
        &self.address12
    }
    #[doc = "0x134 - Replacement address 13"]
    #[inline(always)]
    pub const fn address13(&self) -> &Address13 {
        &self.address13
    }
    #[doc = "0x138 - Replacement address 14"]
    #[inline(always)]
    pub const fn address14(&self) -> &Address14 {
        &self.address14
    }
    #[doc = "0x13c - Replacement address 15"]
    #[inline(always)]
    pub const fn address15(&self) -> &Address15 {
        &self.address15
    }
}
#[doc = "VALUE7 (rw) register accessor: Value replacement 7\n\nYou can [`read`](crate::Reg::read) this register and get [`value7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value7`]
module"]
#[doc(alias = "VALUE7")]
pub type Value7 = crate::Reg<value7::Value7Spec>;
#[doc = "Value replacement 7"]
pub mod value7;
#[doc = "VALUE6 (rw) register accessor: Value replacement 6\n\nYou can [`read`](crate::Reg::read) this register and get [`value6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value6`]
module"]
#[doc(alias = "VALUE6")]
pub type Value6 = crate::Reg<value6::Value6Spec>;
#[doc = "Value replacement 6"]
pub mod value6;
#[doc = "VALUE5 (rw) register accessor: Value replacement 5\n\nYou can [`read`](crate::Reg::read) this register and get [`value5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value5`]
module"]
#[doc(alias = "VALUE5")]
pub type Value5 = crate::Reg<value5::Value5Spec>;
#[doc = "Value replacement 5"]
pub mod value5;
#[doc = "VALUE4 (rw) register accessor: Value replacement 4\n\nYou can [`read`](crate::Reg::read) this register and get [`value4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value4`]
module"]
#[doc(alias = "VALUE4")]
pub type Value4 = crate::Reg<value4::Value4Spec>;
#[doc = "Value replacement 4"]
pub mod value4;
#[doc = "VALUE3 (rw) register accessor: Value replacement 3\n\nYou can [`read`](crate::Reg::read) this register and get [`value3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value3`]
module"]
#[doc(alias = "VALUE3")]
pub type Value3 = crate::Reg<value3::Value3Spec>;
#[doc = "Value replacement 3"]
pub mod value3;
#[doc = "VALUE2 (rw) register accessor: Value replacement 2\n\nYou can [`read`](crate::Reg::read) this register and get [`value2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value2`]
module"]
#[doc(alias = "VALUE2")]
pub type Value2 = crate::Reg<value2::Value2Spec>;
#[doc = "Value replacement 2"]
pub mod value2;
#[doc = "VALUE1 (rw) register accessor: Value replacement 1\n\nYou can [`read`](crate::Reg::read) this register and get [`value1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value1`]
module"]
#[doc(alias = "VALUE1")]
pub type Value1 = crate::Reg<value1::Value1Spec>;
#[doc = "Value replacement 1"]
pub mod value1;
#[doc = "VALUE0 (rw) register accessor: Value replacement 0\n\nYou can [`read`](crate::Reg::read) this register and get [`value0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value0`]
module"]
#[doc(alias = "VALUE0")]
pub type Value0 = crate::Reg<value0::Value0Spec>;
#[doc = "Value replacement 0"]
pub mod value0;
#[doc = "CONTROL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control register"]
pub mod control;
#[doc = "ENABLE (rw) register accessor: Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable register"]
pub mod enable;
#[doc = "ADDRESS0 (rw) register accessor: Replacement address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`address0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address0`]
module"]
#[doc(alias = "ADDRESS0")]
pub type Address0 = crate::Reg<address0::Address0Spec>;
#[doc = "Replacement address 0"]
pub mod address0;
#[doc = "ADDRESS1 (rw) register accessor: Replacement address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`address1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address1`]
module"]
#[doc(alias = "ADDRESS1")]
pub type Address1 = crate::Reg<address1::Address1Spec>;
#[doc = "Replacement address 2"]
pub mod address1;
#[doc = "ADDRESS2 (rw) register accessor: Replacement address 2\n\nYou can [`read`](crate::Reg::read) this register and get [`address2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address2`]
module"]
#[doc(alias = "ADDRESS2")]
pub type Address2 = crate::Reg<address2::Address2Spec>;
#[doc = "Replacement address 2"]
pub mod address2;
#[doc = "ADDRESS3 (rw) register accessor: Replacement address 3\n\nYou can [`read`](crate::Reg::read) this register and get [`address3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address3`]
module"]
#[doc(alias = "ADDRESS3")]
pub type Address3 = crate::Reg<address3::Address3Spec>;
#[doc = "Replacement address 3"]
pub mod address3;
#[doc = "ADDRESS4 (rw) register accessor: Replacement address 4\n\nYou can [`read`](crate::Reg::read) this register and get [`address4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address4`]
module"]
#[doc(alias = "ADDRESS4")]
pub type Address4 = crate::Reg<address4::Address4Spec>;
#[doc = "Replacement address 4"]
pub mod address4;
#[doc = "ADDRESS5 (rw) register accessor: Replacement address 5\n\nYou can [`read`](crate::Reg::read) this register and get [`address5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address5`]
module"]
#[doc(alias = "ADDRESS5")]
pub type Address5 = crate::Reg<address5::Address5Spec>;
#[doc = "Replacement address 5"]
pub mod address5;
#[doc = "ADDRESS6 (rw) register accessor: Replacement address 6\n\nYou can [`read`](crate::Reg::read) this register and get [`address6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address6`]
module"]
#[doc(alias = "ADDRESS6")]
pub type Address6 = crate::Reg<address6::Address6Spec>;
#[doc = "Replacement address 6"]
pub mod address6;
#[doc = "ADDRESS7 (rw) register accessor: Replacement address 7\n\nYou can [`read`](crate::Reg::read) this register and get [`address7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address7`]
module"]
#[doc(alias = "ADDRESS7")]
pub type Address7 = crate::Reg<address7::Address7Spec>;
#[doc = "Replacement address 7"]
pub mod address7;
#[doc = "ADDRESS8 (rw) register accessor: Replacement address 8\n\nYou can [`read`](crate::Reg::read) this register and get [`address8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address8`]
module"]
#[doc(alias = "ADDRESS8")]
pub type Address8 = crate::Reg<address8::Address8Spec>;
#[doc = "Replacement address 8"]
pub mod address8;
#[doc = "ADDRESS9 (rw) register accessor: Replacement address 9\n\nYou can [`read`](crate::Reg::read) this register and get [`address9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address9`]
module"]
#[doc(alias = "ADDRESS9")]
pub type Address9 = crate::Reg<address9::Address9Spec>;
#[doc = "Replacement address 9"]
pub mod address9;
#[doc = "ADDRESS10 (rw) register accessor: Replacement address 10\n\nYou can [`read`](crate::Reg::read) this register and get [`address10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address10`]
module"]
#[doc(alias = "ADDRESS10")]
pub type Address10 = crate::Reg<address10::Address10Spec>;
#[doc = "Replacement address 10"]
pub mod address10;
#[doc = "ADDRESS11 (rw) register accessor: Replacement address 11\n\nYou can [`read`](crate::Reg::read) this register and get [`address11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address11`]
module"]
#[doc(alias = "ADDRESS11")]
pub type Address11 = crate::Reg<address11::Address11Spec>;
#[doc = "Replacement address 11"]
pub mod address11;
#[doc = "ADDRESS12 (rw) register accessor: Replacement address 12\n\nYou can [`read`](crate::Reg::read) this register and get [`address12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address12`]
module"]
#[doc(alias = "ADDRESS12")]
pub type Address12 = crate::Reg<address12::Address12Spec>;
#[doc = "Replacement address 12"]
pub mod address12;
#[doc = "ADDRESS13 (rw) register accessor: Replacement address 13\n\nYou can [`read`](crate::Reg::read) this register and get [`address13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address13`]
module"]
#[doc(alias = "ADDRESS13")]
pub type Address13 = crate::Reg<address13::Address13Spec>;
#[doc = "Replacement address 13"]
pub mod address13;
#[doc = "ADDRESS14 (rw) register accessor: Replacement address 14\n\nYou can [`read`](crate::Reg::read) this register and get [`address14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address14`]
module"]
#[doc(alias = "ADDRESS14")]
pub type Address14 = crate::Reg<address14::Address14Spec>;
#[doc = "Replacement address 14"]
pub mod address14;
#[doc = "ADDRESS15 (rw) register accessor: Replacement address 15\n\nYou can [`read`](crate::Reg::read) this register and get [`address15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address15`]
module"]
#[doc(alias = "ADDRESS15")]
pub type Address15 = crate::Reg<address15::Address15Spec>;
#[doc = "Replacement address 15"]
pub mod address15;
