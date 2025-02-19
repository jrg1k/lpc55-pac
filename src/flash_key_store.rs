#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    header: Header,
    puf_discharge_time_in_ms: PufDischargeTimeInMs,
    activation_code: [ActivationCode; 298],
    _reserved_3_sbkey: [u8; 0x04],
    _reserved_4_sbkey: [u8; 0x04],
    _reserved_5_sbkey: [u8; 0x04],
    _reserved_6_sbkey: [u8; 0x04],
    _reserved_7_sbkey: [u8; 0x04],
    _reserved_8_sbkey: [u8; 0x04],
    _reserved_9_sbkey: [u8; 0x04],
    _reserved_10_sbkey: [u8; 0x04],
    _reserved_11_sbkey: [u8; 0x04],
    _reserved_12_sbkey: [u8; 0x04],
    _reserved_13_sbkey: [u8; 0x04],
    _reserved_14_sbkey: [u8; 0x04],
    _reserved_15_sbkey: [u8; 0x04],
    _reserved_16_sbkey: [u8; 0x04],
    _reserved_17_user_kek: [u8; 0x04],
    _reserved_18_user_kek: [u8; 0x04],
    _reserved_19_user_kek: [u8; 0x04],
    _reserved_20_user_kek: [u8; 0x04],
    _reserved_21_user_kek: [u8; 0x04],
    _reserved_22_user_kek: [u8; 0x04],
    _reserved_23_user_kek: [u8; 0x04],
    _reserved_24_user_kek: [u8; 0x04],
    _reserved_25_user_kek: [u8; 0x04],
    _reserved_26_user_kek: [u8; 0x04],
    _reserved_27_user_kek: [u8; 0x04],
    _reserved_28_user_kek: [u8; 0x04],
    _reserved_29_user_kek: [u8; 0x04],
    _reserved_30_user_kek: [u8; 0x04],
    _reserved_31_uds: [u8; 0x04],
    _reserved_32_uds: [u8; 0x04],
    _reserved_33_uds: [u8; 0x04],
    _reserved_34_uds: [u8; 0x04],
    _reserved_35_uds: [u8; 0x04],
    _reserved_36_uds: [u8; 0x04],
    _reserved_37_uds: [u8; 0x04],
    _reserved_38_uds: [u8; 0x04],
    _reserved_39_uds: [u8; 0x04],
    _reserved_40_uds: [u8; 0x04],
    _reserved_41_uds: [u8; 0x04],
    _reserved_42_uds: [u8; 0x04],
    _reserved_43_uds: [u8; 0x04],
    _reserved_44_uds: [u8; 0x04],
    _reserved_45_prince_region0: [u8; 0x04],
    _reserved_46_prince_region0: [u8; 0x04],
    _reserved_47_prince_region0: [u8; 0x04],
    _reserved_48_prince_region0: [u8; 0x04],
    _reserved_49_prince_region0: [u8; 0x04],
    _reserved_50_prince_region0: [u8; 0x04],
    _reserved_51_prince_region0: [u8; 0x04],
    _reserved_52_prince_region0: [u8; 0x04],
    _reserved_53_prince_region0: [u8; 0x04],
    _reserved_54_prince_region0: [u8; 0x04],
    _reserved_55_prince_region0: [u8; 0x04],
    _reserved_56_prince_region0: [u8; 0x04],
    _reserved_57_prince_region0: [u8; 0x04],
    _reserved_58_prince_region0: [u8; 0x04],
    _reserved_59_prince_region1: [u8; 0x04],
    _reserved_60_prince_region1: [u8; 0x04],
    _reserved_61_prince_region1: [u8; 0x04],
    _reserved_62_prince_region1: [u8; 0x04],
    _reserved_63_prince_region1: [u8; 0x04],
    _reserved_64_prince_region1: [u8; 0x04],
    _reserved_65_prince_region1: [u8; 0x04],
    _reserved_66_prince_region1: [u8; 0x04],
    _reserved_67_prince_region1: [u8; 0x04],
    _reserved_68_prince_region1: [u8; 0x04],
    _reserved_69_prince_region1: [u8; 0x04],
    _reserved_70_prince_region1: [u8; 0x04],
    _reserved_71_prince_region1: [u8; 0x04],
    _reserved_72_prince_region1: [u8; 0x04],
    _reserved_73_prince_region2: [u8; 0x04],
    _reserved_74_prince_region2: [u8; 0x04],
    _reserved_75_prince_region2: [u8; 0x04],
    _reserved_76_prince_region2: [u8; 0x04],
    _reserved_77_prince_region2: [u8; 0x04],
    _reserved_78_prince_region2: [u8; 0x04],
    _reserved_79_prince_region2: [u8; 0x04],
    _reserved_80_prince_region2: [u8; 0x04],
    _reserved_81_prince_region2: [u8; 0x04],
    _reserved_82_prince_region2: [u8; 0x04],
    _reserved_83_prince_region2: [u8; 0x04],
    _reserved_84_prince_region2: [u8; 0x04],
    _reserved_85_prince_region2: [u8; 0x04],
    _reserved_86_prince_region2: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Valid Key Sore Header : 0x95959595"]
    #[inline(always)]
    pub const fn header(&self) -> &Header {
        &self.header
    }
    #[doc = "0x04 - puf discharge time in ms."]
    #[inline(always)]
    pub const fn puf_discharge_time_in_ms(&self) -> &PufDischargeTimeInMs {
        &self.puf_discharge_time_in_ms
    }
    #[doc = "0x08..0x4b0 - ."]
    #[inline(always)]
    pub const fn activation_code(&self, n: usize) -> &ActivationCode {
        &self.activation_code[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x4b0 - ."]
    #[inline(always)]
    pub fn activation_code_iter(&self) -> impl Iterator<Item = &ActivationCode> {
        self.activation_code.iter()
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code0(&self) -> &SbkeyKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1200).cast() }
    }
    #[doc = "0x4b0 - ."]
    #[inline(always)]
    pub const fn sbkey_header0(&self) -> &SbkeyHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1200).cast() }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code1(&self) -> &SbkeyKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1204).cast() }
    }
    #[doc = "0x4b4 - ."]
    #[inline(always)]
    pub const fn sbkey_header1(&self) -> &SbkeyHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1204).cast() }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code2(&self) -> &SbkeyKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1208).cast() }
    }
    #[doc = "0x4b8 - ."]
    #[inline(always)]
    pub const fn sbkey_body0(&self) -> &SbkeyBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1208).cast() }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code3(&self) -> &SbkeyKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1212).cast() }
    }
    #[doc = "0x4bc - ."]
    #[inline(always)]
    pub const fn sbkey_body1(&self) -> &SbkeyBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1212).cast() }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code4(&self) -> &SbkeyKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1216).cast() }
    }
    #[doc = "0x4c0 - ."]
    #[inline(always)]
    pub const fn sbkey_body2(&self) -> &SbkeyBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1216).cast() }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code5(&self) -> &SbkeyKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1220).cast() }
    }
    #[doc = "0x4c4 - ."]
    #[inline(always)]
    pub const fn sbkey_body3(&self) -> &SbkeyBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1220).cast() }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code6(&self) -> &SbkeyKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1224).cast() }
    }
    #[doc = "0x4c8 - ."]
    #[inline(always)]
    pub const fn sbkey_body4(&self) -> &SbkeyBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1224).cast() }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code7(&self) -> &SbkeyKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1228).cast() }
    }
    #[doc = "0x4cc - ."]
    #[inline(always)]
    pub const fn sbkey_body5(&self) -> &SbkeyBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1228).cast() }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code8(&self) -> &SbkeyKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1232).cast() }
    }
    #[doc = "0x4d0 - ."]
    #[inline(always)]
    pub const fn sbkey_body6(&self) -> &SbkeyBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1232).cast() }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code9(&self) -> &SbkeyKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1236).cast() }
    }
    #[doc = "0x4d4 - ."]
    #[inline(always)]
    pub const fn sbkey_body7(&self) -> &SbkeyBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1236).cast() }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code10(&self) -> &SbkeyKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1240).cast() }
    }
    #[doc = "0x4d8 - ."]
    #[inline(always)]
    pub const fn sbkey_body8(&self) -> &SbkeyBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1240).cast() }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub const fn sbkey_key_code11(&self) -> &SbkeyKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1244).cast() }
    }
    #[doc = "0x4dc - ."]
    #[inline(always)]
    pub const fn sbkey_body9(&self) -> &SbkeyBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1244).cast() }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code12(&self) -> &SbkeyKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1248).cast() }
    }
    #[doc = "0x4e0 - ."]
    #[inline(always)]
    pub const fn sbkey_body10(&self) -> &SbkeyBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1248).cast() }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub const fn sbkey_key_code13(&self) -> &SbkeyKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1252).cast() }
    }
    #[doc = "0x4e4 - ."]
    #[inline(always)]
    pub const fn sbkey_body11(&self) -> &SbkeyBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1252).cast() }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code0(&self) -> &UserKekKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1256).cast() }
    }
    #[doc = "0x4e8 - ."]
    #[inline(always)]
    pub const fn user_kek_header0(&self) -> &UserKekHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1256).cast() }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub const fn user_kek_key_code1(&self) -> &UserKekKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1260).cast() }
    }
    #[doc = "0x4ec - ."]
    #[inline(always)]
    pub const fn user_kek_header1(&self) -> &UserKekHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1260).cast() }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code2(&self) -> &UserKekKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1264).cast() }
    }
    #[doc = "0x4f0 - ."]
    #[inline(always)]
    pub const fn user_kek_body0(&self) -> &UserKekBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1264).cast() }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code3(&self) -> &UserKekKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1268).cast() }
    }
    #[doc = "0x4f4 - ."]
    #[inline(always)]
    pub const fn user_kek_body1(&self) -> &UserKekBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1268).cast() }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code4(&self) -> &UserKekKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1272).cast() }
    }
    #[doc = "0x4f8 - ."]
    #[inline(always)]
    pub const fn user_kek_body2(&self) -> &UserKekBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1272).cast() }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub const fn user_kek_key_code5(&self) -> &UserKekKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1276).cast() }
    }
    #[doc = "0x4fc - ."]
    #[inline(always)]
    pub const fn user_kek_body3(&self) -> &UserKekBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1276).cast() }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code6(&self) -> &UserKekKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x500 - ."]
    #[inline(always)]
    pub const fn user_kek_body4(&self) -> &UserKekBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code7(&self) -> &UserKekKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x504 - ."]
    #[inline(always)]
    pub const fn user_kek_body5(&self) -> &UserKekBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code8(&self) -> &UserKekKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x508 - ."]
    #[inline(always)]
    pub const fn user_kek_body6(&self) -> &UserKekBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code9(&self) -> &UserKekKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1292).cast() }
    }
    #[doc = "0x50c - ."]
    #[inline(always)]
    pub const fn user_kek_body7(&self) -> &UserKekBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1292).cast() }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code10(&self) -> &UserKekKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1296).cast() }
    }
    #[doc = "0x510 - ."]
    #[inline(always)]
    pub const fn user_kek_body8(&self) -> &UserKekBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1296).cast() }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code11(&self) -> &UserKekKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1300).cast() }
    }
    #[doc = "0x514 - ."]
    #[inline(always)]
    pub const fn user_kek_body9(&self) -> &UserKekBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1300).cast() }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub const fn user_kek_key_code12(&self) -> &UserKekKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1304).cast() }
    }
    #[doc = "0x518 - ."]
    #[inline(always)]
    pub const fn user_kek_body10(&self) -> &UserKekBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1304).cast() }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub const fn user_kek_key_code13(&self) -> &UserKekKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1308).cast() }
    }
    #[doc = "0x51c - ."]
    #[inline(always)]
    pub const fn user_kek_body11(&self) -> &UserKekBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1308).cast() }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub const fn uds_key_code0(&self) -> &UdsKeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x520 - ."]
    #[inline(always)]
    pub const fn uds_header0(&self) -> &UdsHeader0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub const fn uds_key_code1(&self) -> &UdsKeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x524 - ."]
    #[inline(always)]
    pub const fn uds_header1(&self) -> &UdsHeader1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub const fn uds_key_code2(&self) -> &UdsKeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x528 - ."]
    #[inline(always)]
    pub const fn uds_body0(&self) -> &UdsBody0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub const fn uds_key_code3(&self) -> &UdsKeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1324).cast() }
    }
    #[doc = "0x52c - ."]
    #[inline(always)]
    pub const fn uds_body1(&self) -> &UdsBody1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1324).cast() }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub const fn uds_key_code4(&self) -> &UdsKeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x530 - ."]
    #[inline(always)]
    pub const fn uds_body2(&self) -> &UdsBody2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1328).cast() }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub const fn uds_key_code5(&self) -> &UdsKeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1332).cast() }
    }
    #[doc = "0x534 - ."]
    #[inline(always)]
    pub const fn uds_body3(&self) -> &UdsBody3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1332).cast() }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub const fn uds_key_code6(&self) -> &UdsKeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1336).cast() }
    }
    #[doc = "0x538 - ."]
    #[inline(always)]
    pub const fn uds_body4(&self) -> &UdsBody4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1336).cast() }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub const fn uds_key_code7(&self) -> &UdsKeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1340).cast() }
    }
    #[doc = "0x53c - ."]
    #[inline(always)]
    pub const fn uds_body5(&self) -> &UdsBody5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1340).cast() }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub const fn uds_key_code8(&self) -> &UdsKeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x540 - ."]
    #[inline(always)]
    pub const fn uds_body6(&self) -> &UdsBody6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub const fn uds_key_code9(&self) -> &UdsKeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x544 - ."]
    #[inline(always)]
    pub const fn uds_body7(&self) -> &UdsBody7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub const fn uds_key_code10(&self) -> &UdsKeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1352).cast() }
    }
    #[doc = "0x548 - ."]
    #[inline(always)]
    pub const fn uds_body8(&self) -> &UdsBody8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1352).cast() }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub const fn uds_key_code11(&self) -> &UdsKeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1356).cast() }
    }
    #[doc = "0x54c - ."]
    #[inline(always)]
    pub const fn uds_body9(&self) -> &UdsBody9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1356).cast() }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub const fn uds_key_code12(&self) -> &UdsKeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1360).cast() }
    }
    #[doc = "0x550 - ."]
    #[inline(always)]
    pub const fn uds_body10(&self) -> &UdsBody10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1360).cast() }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub const fn uds_key_code13(&self) -> &UdsKeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1364).cast() }
    }
    #[doc = "0x554 - ."]
    #[inline(always)]
    pub const fn uds_body11(&self) -> &UdsBody11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1364).cast() }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code0(&self) -> &PrinceRegion0KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1368).cast() }
    }
    #[doc = "0x558 - ."]
    #[inline(always)]
    pub const fn prince_region0_header0(&self) -> &PrinceRegion0Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1368).cast() }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code1(&self) -> &PrinceRegion0KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1372).cast() }
    }
    #[doc = "0x55c - ."]
    #[inline(always)]
    pub const fn prince_region0_header1(&self) -> &PrinceRegion0Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1372).cast() }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code2(&self) -> &PrinceRegion0KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - ."]
    #[inline(always)]
    pub const fn prince_region0_body0(&self) -> &PrinceRegion0Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code3(&self) -> &PrinceRegion0KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x564 - ."]
    #[inline(always)]
    pub const fn prince_region0_body1(&self) -> &PrinceRegion0Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code4(&self) -> &PrinceRegion0KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x568 - ."]
    #[inline(always)]
    pub const fn prince_region0_body2(&self) -> &PrinceRegion0Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code5(&self) -> &PrinceRegion0KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x56c - ."]
    #[inline(always)]
    pub const fn prince_region0_body3(&self) -> &PrinceRegion0Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code6(&self) -> &PrinceRegion0KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1392).cast() }
    }
    #[doc = "0x570 - ."]
    #[inline(always)]
    pub const fn prince_region0_body4(&self) -> &PrinceRegion0Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1392).cast() }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code7(&self) -> &PrinceRegion0KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1396).cast() }
    }
    #[doc = "0x574 - ."]
    #[inline(always)]
    pub const fn prince_region0_body5(&self) -> &PrinceRegion0Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1396).cast() }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code8(&self) -> &PrinceRegion0KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1400).cast() }
    }
    #[doc = "0x578 - ."]
    #[inline(always)]
    pub const fn prince_region0_body6(&self) -> &PrinceRegion0Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1400).cast() }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code9(&self) -> &PrinceRegion0KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1404).cast() }
    }
    #[doc = "0x57c - ."]
    #[inline(always)]
    pub const fn prince_region0_body7(&self) -> &PrinceRegion0Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1404).cast() }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code10(&self) -> &PrinceRegion0KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1408).cast() }
    }
    #[doc = "0x580 - ."]
    #[inline(always)]
    pub const fn prince_region0_body8(&self) -> &PrinceRegion0Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1408).cast() }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code11(&self) -> &PrinceRegion0KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1412).cast() }
    }
    #[doc = "0x584 - ."]
    #[inline(always)]
    pub const fn prince_region0_body9(&self) -> &PrinceRegion0Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1412).cast() }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code12(&self) -> &PrinceRegion0KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1416).cast() }
    }
    #[doc = "0x588 - ."]
    #[inline(always)]
    pub const fn prince_region0_body10(&self) -> &PrinceRegion0Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1416).cast() }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub const fn prince_region0_key_code13(&self) -> &PrinceRegion0KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1420).cast() }
    }
    #[doc = "0x58c - ."]
    #[inline(always)]
    pub const fn prince_region0_body11(&self) -> &PrinceRegion0Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1420).cast() }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code0(&self) -> &PrinceRegion1KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x590 - ."]
    #[inline(always)]
    pub const fn prince_region1_header0(&self) -> &PrinceRegion1Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1424).cast() }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code1(&self) -> &PrinceRegion1KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1428).cast() }
    }
    #[doc = "0x594 - ."]
    #[inline(always)]
    pub const fn prince_region1_header1(&self) -> &PrinceRegion1Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1428).cast() }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code2(&self) -> &PrinceRegion1KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1432).cast() }
    }
    #[doc = "0x598 - ."]
    #[inline(always)]
    pub const fn prince_region1_body0(&self) -> &PrinceRegion1Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1432).cast() }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code3(&self) -> &PrinceRegion1KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1436).cast() }
    }
    #[doc = "0x59c - ."]
    #[inline(always)]
    pub const fn prince_region1_body1(&self) -> &PrinceRegion1Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1436).cast() }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code4(&self) -> &PrinceRegion1KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1440).cast() }
    }
    #[doc = "0x5a0 - ."]
    #[inline(always)]
    pub const fn prince_region1_body2(&self) -> &PrinceRegion1Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1440).cast() }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code5(&self) -> &PrinceRegion1KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1444).cast() }
    }
    #[doc = "0x5a4 - ."]
    #[inline(always)]
    pub const fn prince_region1_body3(&self) -> &PrinceRegion1Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1444).cast() }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code6(&self) -> &PrinceRegion1KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1448).cast() }
    }
    #[doc = "0x5a8 - ."]
    #[inline(always)]
    pub const fn prince_region1_body4(&self) -> &PrinceRegion1Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1448).cast() }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code7(&self) -> &PrinceRegion1KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1452).cast() }
    }
    #[doc = "0x5ac - ."]
    #[inline(always)]
    pub const fn prince_region1_body5(&self) -> &PrinceRegion1Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1452).cast() }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code8(&self) -> &PrinceRegion1KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1456).cast() }
    }
    #[doc = "0x5b0 - ."]
    #[inline(always)]
    pub const fn prince_region1_body6(&self) -> &PrinceRegion1Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1456).cast() }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code9(&self) -> &PrinceRegion1KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1460).cast() }
    }
    #[doc = "0x5b4 - ."]
    #[inline(always)]
    pub const fn prince_region1_body7(&self) -> &PrinceRegion1Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1460).cast() }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code10(&self) -> &PrinceRegion1KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1464).cast() }
    }
    #[doc = "0x5b8 - ."]
    #[inline(always)]
    pub const fn prince_region1_body8(&self) -> &PrinceRegion1Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1464).cast() }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code11(&self) -> &PrinceRegion1KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1468).cast() }
    }
    #[doc = "0x5bc - ."]
    #[inline(always)]
    pub const fn prince_region1_body9(&self) -> &PrinceRegion1Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1468).cast() }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code12(&self) -> &PrinceRegion1KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c0 - ."]
    #[inline(always)]
    pub const fn prince_region1_body10(&self) -> &PrinceRegion1Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1472).cast() }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub const fn prince_region1_key_code13(&self) -> &PrinceRegion1KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1476).cast() }
    }
    #[doc = "0x5c4 - ."]
    #[inline(always)]
    pub const fn prince_region1_body11(&self) -> &PrinceRegion1Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1476).cast() }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code0(&self) -> &PrinceRegion2KeyCode0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1480).cast() }
    }
    #[doc = "0x5c8 - ."]
    #[inline(always)]
    pub const fn prince_region2_header0(&self) -> &PrinceRegion2Header0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1480).cast() }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code1(&self) -> &PrinceRegion2KeyCode1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1484).cast() }
    }
    #[doc = "0x5cc - ."]
    #[inline(always)]
    pub const fn prince_region2_header1(&self) -> &PrinceRegion2Header1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1484).cast() }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code2(&self) -> &PrinceRegion2KeyCode2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1488).cast() }
    }
    #[doc = "0x5d0 - ."]
    #[inline(always)]
    pub const fn prince_region2_body0(&self) -> &PrinceRegion2Body0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1488).cast() }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code3(&self) -> &PrinceRegion2KeyCode3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1492).cast() }
    }
    #[doc = "0x5d4 - ."]
    #[inline(always)]
    pub const fn prince_region2_body1(&self) -> &PrinceRegion2Body1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1492).cast() }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code4(&self) -> &PrinceRegion2KeyCode4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1496).cast() }
    }
    #[doc = "0x5d8 - ."]
    #[inline(always)]
    pub const fn prince_region2_body2(&self) -> &PrinceRegion2Body2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1496).cast() }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code5(&self) -> &PrinceRegion2KeyCode5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1500).cast() }
    }
    #[doc = "0x5dc - ."]
    #[inline(always)]
    pub const fn prince_region2_body3(&self) -> &PrinceRegion2Body3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1500).cast() }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code6(&self) -> &PrinceRegion2KeyCode6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1504).cast() }
    }
    #[doc = "0x5e0 - ."]
    #[inline(always)]
    pub const fn prince_region2_body4(&self) -> &PrinceRegion2Body4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1504).cast() }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code7(&self) -> &PrinceRegion2KeyCode7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1508).cast() }
    }
    #[doc = "0x5e4 - ."]
    #[inline(always)]
    pub const fn prince_region2_body5(&self) -> &PrinceRegion2Body5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1508).cast() }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code8(&self) -> &PrinceRegion2KeyCode8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1512).cast() }
    }
    #[doc = "0x5e8 - ."]
    #[inline(always)]
    pub const fn prince_region2_body6(&self) -> &PrinceRegion2Body6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1512).cast() }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code9(&self) -> &PrinceRegion2KeyCode9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1516).cast() }
    }
    #[doc = "0x5ec - ."]
    #[inline(always)]
    pub const fn prince_region2_body7(&self) -> &PrinceRegion2Body7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1516).cast() }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code10(&self) -> &PrinceRegion2KeyCode10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f0 - ."]
    #[inline(always)]
    pub const fn prince_region2_body8(&self) -> &PrinceRegion2Body8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1520).cast() }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code11(&self) -> &PrinceRegion2KeyCode11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1524).cast() }
    }
    #[doc = "0x5f4 - ."]
    #[inline(always)]
    pub const fn prince_region2_body9(&self) -> &PrinceRegion2Body9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1524).cast() }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code12(&self) -> &PrinceRegion2KeyCode12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1528).cast() }
    }
    #[doc = "0x5f8 - ."]
    #[inline(always)]
    pub const fn prince_region2_body10(&self) -> &PrinceRegion2Body10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1528).cast() }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub const fn prince_region2_key_code13(&self) -> &PrinceRegion2KeyCode13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1532).cast() }
    }
    #[doc = "0x5fc - ."]
    #[inline(always)]
    pub const fn prince_region2_body11(&self) -> &PrinceRegion2Body11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1532).cast() }
    }
}
#[doc = "HEADER (rw) register accessor: Valid Key Sore Header : 0x95959595\n\nYou can [`read`](crate::Reg::read) this register and get [`header::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`header::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@header`]
module"]
#[doc(alias = "HEADER")]
pub type Header = crate::Reg<header::HeaderSpec>;
#[doc = "Valid Key Sore Header : 0x95959595"]
pub mod header;
#[doc = "puf_discharge_time_in_ms (rw) register accessor: puf discharge time in ms.\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_discharge_time_in_ms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_discharge_time_in_ms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@puf_discharge_time_in_ms`]
module"]
#[doc(alias = "puf_discharge_time_in_ms")]
pub type PufDischargeTimeInMs =
    crate::Reg<puf_discharge_time_in_ms::PufDischargeTimeInMsSpec>;
#[doc = "puf discharge time in ms."]
pub mod puf_discharge_time_in_ms;
#[doc = "ACTIVATION_CODE (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`activation_code::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`activation_code::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@activation_code`]
module"]
#[doc(alias = "ACTIVATION_CODE")]
pub type ActivationCode = crate::Reg<activation_code::ActivationCodeSpec>;
#[doc = "."]
pub mod activation_code;
#[doc = "SBKEY_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_header0`]
module"]
#[doc(alias = "SBKEY_HEADER0")]
pub type SbkeyHeader0 = crate::Reg<sbkey_header0::SbkeyHeader0Spec>;
#[doc = "."]
pub mod sbkey_header0;
#[doc = "SBKEY_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code0`]
module"]
#[doc(alias = "SBKEY_KEY_CODE0")]
pub type SbkeyKeyCode0 = crate::Reg<sbkey_key_code0::SbkeyKeyCode0Spec>;
#[doc = "."]
pub mod sbkey_key_code0;
#[doc = "SBKEY_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_header1`]
module"]
#[doc(alias = "SBKEY_HEADER1")]
pub type SbkeyHeader1 = crate::Reg<sbkey_header1::SbkeyHeader1Spec>;
#[doc = "."]
pub mod sbkey_header1;
#[doc = "SBKEY_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code1`]
module"]
#[doc(alias = "SBKEY_KEY_CODE1")]
pub type SbkeyKeyCode1 = crate::Reg<sbkey_key_code1::SbkeyKeyCode1Spec>;
#[doc = "."]
pub mod sbkey_key_code1;
#[doc = "SBKEY_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body0`]
module"]
#[doc(alias = "SBKEY_BODY0")]
pub type SbkeyBody0 = crate::Reg<sbkey_body0::SbkeyBody0Spec>;
#[doc = "."]
pub mod sbkey_body0;
#[doc = "SBKEY_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code2`]
module"]
#[doc(alias = "SBKEY_KEY_CODE2")]
pub type SbkeyKeyCode2 = crate::Reg<sbkey_key_code2::SbkeyKeyCode2Spec>;
#[doc = "."]
pub mod sbkey_key_code2;
#[doc = "SBKEY_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body1`]
module"]
#[doc(alias = "SBKEY_BODY1")]
pub type SbkeyBody1 = crate::Reg<sbkey_body1::SbkeyBody1Spec>;
#[doc = "."]
pub mod sbkey_body1;
#[doc = "SBKEY_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code3`]
module"]
#[doc(alias = "SBKEY_KEY_CODE3")]
pub type SbkeyKeyCode3 = crate::Reg<sbkey_key_code3::SbkeyKeyCode3Spec>;
#[doc = "."]
pub mod sbkey_key_code3;
#[doc = "SBKEY_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body2`]
module"]
#[doc(alias = "SBKEY_BODY2")]
pub type SbkeyBody2 = crate::Reg<sbkey_body2::SbkeyBody2Spec>;
#[doc = "."]
pub mod sbkey_body2;
#[doc = "SBKEY_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code4`]
module"]
#[doc(alias = "SBKEY_KEY_CODE4")]
pub type SbkeyKeyCode4 = crate::Reg<sbkey_key_code4::SbkeyKeyCode4Spec>;
#[doc = "."]
pub mod sbkey_key_code4;
#[doc = "SBKEY_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body3`]
module"]
#[doc(alias = "SBKEY_BODY3")]
pub type SbkeyBody3 = crate::Reg<sbkey_body3::SbkeyBody3Spec>;
#[doc = "."]
pub mod sbkey_body3;
#[doc = "SBKEY_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code5`]
module"]
#[doc(alias = "SBKEY_KEY_CODE5")]
pub type SbkeyKeyCode5 = crate::Reg<sbkey_key_code5::SbkeyKeyCode5Spec>;
#[doc = "."]
pub mod sbkey_key_code5;
#[doc = "SBKEY_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body4`]
module"]
#[doc(alias = "SBKEY_BODY4")]
pub type SbkeyBody4 = crate::Reg<sbkey_body4::SbkeyBody4Spec>;
#[doc = "."]
pub mod sbkey_body4;
#[doc = "SBKEY_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code6`]
module"]
#[doc(alias = "SBKEY_KEY_CODE6")]
pub type SbkeyKeyCode6 = crate::Reg<sbkey_key_code6::SbkeyKeyCode6Spec>;
#[doc = "."]
pub mod sbkey_key_code6;
#[doc = "SBKEY_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body5`]
module"]
#[doc(alias = "SBKEY_BODY5")]
pub type SbkeyBody5 = crate::Reg<sbkey_body5::SbkeyBody5Spec>;
#[doc = "."]
pub mod sbkey_body5;
#[doc = "SBKEY_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code7`]
module"]
#[doc(alias = "SBKEY_KEY_CODE7")]
pub type SbkeyKeyCode7 = crate::Reg<sbkey_key_code7::SbkeyKeyCode7Spec>;
#[doc = "."]
pub mod sbkey_key_code7;
#[doc = "SBKEY_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body6`]
module"]
#[doc(alias = "SBKEY_BODY6")]
pub type SbkeyBody6 = crate::Reg<sbkey_body6::SbkeyBody6Spec>;
#[doc = "."]
pub mod sbkey_body6;
#[doc = "SBKEY_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code8`]
module"]
#[doc(alias = "SBKEY_KEY_CODE8")]
pub type SbkeyKeyCode8 = crate::Reg<sbkey_key_code8::SbkeyKeyCode8Spec>;
#[doc = "."]
pub mod sbkey_key_code8;
#[doc = "SBKEY_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body7`]
module"]
#[doc(alias = "SBKEY_BODY7")]
pub type SbkeyBody7 = crate::Reg<sbkey_body7::SbkeyBody7Spec>;
#[doc = "."]
pub mod sbkey_body7;
#[doc = "SBKEY_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code9`]
module"]
#[doc(alias = "SBKEY_KEY_CODE9")]
pub type SbkeyKeyCode9 = crate::Reg<sbkey_key_code9::SbkeyKeyCode9Spec>;
#[doc = "."]
pub mod sbkey_key_code9;
#[doc = "SBKEY_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body8`]
module"]
#[doc(alias = "SBKEY_BODY8")]
pub type SbkeyBody8 = crate::Reg<sbkey_body8::SbkeyBody8Spec>;
#[doc = "."]
pub mod sbkey_body8;
#[doc = "SBKEY_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code10`]
module"]
#[doc(alias = "SBKEY_KEY_CODE10")]
pub type SbkeyKeyCode10 = crate::Reg<sbkey_key_code10::SbkeyKeyCode10Spec>;
#[doc = "."]
pub mod sbkey_key_code10;
#[doc = "SBKEY_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body9`]
module"]
#[doc(alias = "SBKEY_BODY9")]
pub type SbkeyBody9 = crate::Reg<sbkey_body9::SbkeyBody9Spec>;
#[doc = "."]
pub mod sbkey_body9;
#[doc = "SBKEY_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code11`]
module"]
#[doc(alias = "SBKEY_KEY_CODE11")]
pub type SbkeyKeyCode11 = crate::Reg<sbkey_key_code11::SbkeyKeyCode11Spec>;
#[doc = "."]
pub mod sbkey_key_code11;
#[doc = "SBKEY_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body10`]
module"]
#[doc(alias = "SBKEY_BODY10")]
pub type SbkeyBody10 = crate::Reg<sbkey_body10::SbkeyBody10Spec>;
#[doc = "."]
pub mod sbkey_body10;
#[doc = "SBKEY_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code12`]
module"]
#[doc(alias = "SBKEY_KEY_CODE12")]
pub type SbkeyKeyCode12 = crate::Reg<sbkey_key_code12::SbkeyKeyCode12Spec>;
#[doc = "."]
pub mod sbkey_key_code12;
#[doc = "SBKEY_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_body11`]
module"]
#[doc(alias = "SBKEY_BODY11")]
pub type SbkeyBody11 = crate::Reg<sbkey_body11::SbkeyBody11Spec>;
#[doc = "."]
pub mod sbkey_body11;
#[doc = "SBKEY_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbkey_key_code13`]
module"]
#[doc(alias = "SBKEY_KEY_CODE13")]
pub type SbkeyKeyCode13 = crate::Reg<sbkey_key_code13::SbkeyKeyCode13Spec>;
#[doc = "."]
pub mod sbkey_key_code13;
#[doc = "USER_KEK_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_header0`]
module"]
#[doc(alias = "USER_KEK_HEADER0")]
pub type UserKekHeader0 = crate::Reg<user_kek_header0::UserKekHeader0Spec>;
#[doc = "."]
pub mod user_kek_header0;
#[doc = "USER_KEK_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code0`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE0")]
pub type UserKekKeyCode0 = crate::Reg<user_kek_key_code0::UserKekKeyCode0Spec>;
#[doc = "."]
pub mod user_kek_key_code0;
#[doc = "USER_KEK_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_header1`]
module"]
#[doc(alias = "USER_KEK_HEADER1")]
pub type UserKekHeader1 = crate::Reg<user_kek_header1::UserKekHeader1Spec>;
#[doc = "."]
pub mod user_kek_header1;
#[doc = "USER_KEK_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code1`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE1")]
pub type UserKekKeyCode1 = crate::Reg<user_kek_key_code1::UserKekKeyCode1Spec>;
#[doc = "."]
pub mod user_kek_key_code1;
#[doc = "USER_KEK_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body0`]
module"]
#[doc(alias = "USER_KEK_BODY0")]
pub type UserKekBody0 = crate::Reg<user_kek_body0::UserKekBody0Spec>;
#[doc = "."]
pub mod user_kek_body0;
#[doc = "USER_KEK_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code2`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE2")]
pub type UserKekKeyCode2 = crate::Reg<user_kek_key_code2::UserKekKeyCode2Spec>;
#[doc = "."]
pub mod user_kek_key_code2;
#[doc = "USER_KEK_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body1`]
module"]
#[doc(alias = "USER_KEK_BODY1")]
pub type UserKekBody1 = crate::Reg<user_kek_body1::UserKekBody1Spec>;
#[doc = "."]
pub mod user_kek_body1;
#[doc = "USER_KEK_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code3`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE3")]
pub type UserKekKeyCode3 = crate::Reg<user_kek_key_code3::UserKekKeyCode3Spec>;
#[doc = "."]
pub mod user_kek_key_code3;
#[doc = "USER_KEK_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body2`]
module"]
#[doc(alias = "USER_KEK_BODY2")]
pub type UserKekBody2 = crate::Reg<user_kek_body2::UserKekBody2Spec>;
#[doc = "."]
pub mod user_kek_body2;
#[doc = "USER_KEK_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code4`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE4")]
pub type UserKekKeyCode4 = crate::Reg<user_kek_key_code4::UserKekKeyCode4Spec>;
#[doc = "."]
pub mod user_kek_key_code4;
#[doc = "USER_KEK_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body3`]
module"]
#[doc(alias = "USER_KEK_BODY3")]
pub type UserKekBody3 = crate::Reg<user_kek_body3::UserKekBody3Spec>;
#[doc = "."]
pub mod user_kek_body3;
#[doc = "USER_KEK_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code5`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE5")]
pub type UserKekKeyCode5 = crate::Reg<user_kek_key_code5::UserKekKeyCode5Spec>;
#[doc = "."]
pub mod user_kek_key_code5;
#[doc = "USER_KEK_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body4`]
module"]
#[doc(alias = "USER_KEK_BODY4")]
pub type UserKekBody4 = crate::Reg<user_kek_body4::UserKekBody4Spec>;
#[doc = "."]
pub mod user_kek_body4;
#[doc = "USER_KEK_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code6`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE6")]
pub type UserKekKeyCode6 = crate::Reg<user_kek_key_code6::UserKekKeyCode6Spec>;
#[doc = "."]
pub mod user_kek_key_code6;
#[doc = "USER_KEK_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body5`]
module"]
#[doc(alias = "USER_KEK_BODY5")]
pub type UserKekBody5 = crate::Reg<user_kek_body5::UserKekBody5Spec>;
#[doc = "."]
pub mod user_kek_body5;
#[doc = "USER_KEK_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code7`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE7")]
pub type UserKekKeyCode7 = crate::Reg<user_kek_key_code7::UserKekKeyCode7Spec>;
#[doc = "."]
pub mod user_kek_key_code7;
#[doc = "USER_KEK_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body6`]
module"]
#[doc(alias = "USER_KEK_BODY6")]
pub type UserKekBody6 = crate::Reg<user_kek_body6::UserKekBody6Spec>;
#[doc = "."]
pub mod user_kek_body6;
#[doc = "USER_KEK_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code8`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE8")]
pub type UserKekKeyCode8 = crate::Reg<user_kek_key_code8::UserKekKeyCode8Spec>;
#[doc = "."]
pub mod user_kek_key_code8;
#[doc = "USER_KEK_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body7`]
module"]
#[doc(alias = "USER_KEK_BODY7")]
pub type UserKekBody7 = crate::Reg<user_kek_body7::UserKekBody7Spec>;
#[doc = "."]
pub mod user_kek_body7;
#[doc = "USER_KEK_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code9`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE9")]
pub type UserKekKeyCode9 = crate::Reg<user_kek_key_code9::UserKekKeyCode9Spec>;
#[doc = "."]
pub mod user_kek_key_code9;
#[doc = "USER_KEK_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body8`]
module"]
#[doc(alias = "USER_KEK_BODY8")]
pub type UserKekBody8 = crate::Reg<user_kek_body8::UserKekBody8Spec>;
#[doc = "."]
pub mod user_kek_body8;
#[doc = "USER_KEK_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code10`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE10")]
pub type UserKekKeyCode10 = crate::Reg<user_kek_key_code10::UserKekKeyCode10Spec>;
#[doc = "."]
pub mod user_kek_key_code10;
#[doc = "USER_KEK_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body9`]
module"]
#[doc(alias = "USER_KEK_BODY9")]
pub type UserKekBody9 = crate::Reg<user_kek_body9::UserKekBody9Spec>;
#[doc = "."]
pub mod user_kek_body9;
#[doc = "USER_KEK_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code11`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE11")]
pub type UserKekKeyCode11 = crate::Reg<user_kek_key_code11::UserKekKeyCode11Spec>;
#[doc = "."]
pub mod user_kek_key_code11;
#[doc = "USER_KEK_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body10`]
module"]
#[doc(alias = "USER_KEK_BODY10")]
pub type UserKekBody10 = crate::Reg<user_kek_body10::UserKekBody10Spec>;
#[doc = "."]
pub mod user_kek_body10;
#[doc = "USER_KEK_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code12`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE12")]
pub type UserKekKeyCode12 = crate::Reg<user_kek_key_code12::UserKekKeyCode12Spec>;
#[doc = "."]
pub mod user_kek_key_code12;
#[doc = "USER_KEK_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_body11`]
module"]
#[doc(alias = "USER_KEK_BODY11")]
pub type UserKekBody11 = crate::Reg<user_kek_body11::UserKekBody11Spec>;
#[doc = "."]
pub mod user_kek_body11;
#[doc = "USER_KEK_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user_kek_key_code13`]
module"]
#[doc(alias = "USER_KEK_KEY_CODE13")]
pub type UserKekKeyCode13 = crate::Reg<user_kek_key_code13::UserKekKeyCode13Spec>;
#[doc = "."]
pub mod user_kek_key_code13;
#[doc = "UDS_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_header0`]
module"]
#[doc(alias = "UDS_HEADER0")]
pub type UdsHeader0 = crate::Reg<uds_header0::UdsHeader0Spec>;
#[doc = "."]
pub mod uds_header0;
#[doc = "UDS_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code0`]
module"]
#[doc(alias = "UDS_KEY_CODE0")]
pub type UdsKeyCode0 = crate::Reg<uds_key_code0::UdsKeyCode0Spec>;
#[doc = "."]
pub mod uds_key_code0;
#[doc = "UDS_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_header1`]
module"]
#[doc(alias = "UDS_HEADER1")]
pub type UdsHeader1 = crate::Reg<uds_header1::UdsHeader1Spec>;
#[doc = "."]
pub mod uds_header1;
#[doc = "UDS_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code1`]
module"]
#[doc(alias = "UDS_KEY_CODE1")]
pub type UdsKeyCode1 = crate::Reg<uds_key_code1::UdsKeyCode1Spec>;
#[doc = "."]
pub mod uds_key_code1;
#[doc = "UDS_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body0`]
module"]
#[doc(alias = "UDS_BODY0")]
pub type UdsBody0 = crate::Reg<uds_body0::UdsBody0Spec>;
#[doc = "."]
pub mod uds_body0;
#[doc = "UDS_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code2`]
module"]
#[doc(alias = "UDS_KEY_CODE2")]
pub type UdsKeyCode2 = crate::Reg<uds_key_code2::UdsKeyCode2Spec>;
#[doc = "."]
pub mod uds_key_code2;
#[doc = "UDS_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body1`]
module"]
#[doc(alias = "UDS_BODY1")]
pub type UdsBody1 = crate::Reg<uds_body1::UdsBody1Spec>;
#[doc = "."]
pub mod uds_body1;
#[doc = "UDS_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code3`]
module"]
#[doc(alias = "UDS_KEY_CODE3")]
pub type UdsKeyCode3 = crate::Reg<uds_key_code3::UdsKeyCode3Spec>;
#[doc = "."]
pub mod uds_key_code3;
#[doc = "UDS_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body2`]
module"]
#[doc(alias = "UDS_BODY2")]
pub type UdsBody2 = crate::Reg<uds_body2::UdsBody2Spec>;
#[doc = "."]
pub mod uds_body2;
#[doc = "UDS_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code4`]
module"]
#[doc(alias = "UDS_KEY_CODE4")]
pub type UdsKeyCode4 = crate::Reg<uds_key_code4::UdsKeyCode4Spec>;
#[doc = "."]
pub mod uds_key_code4;
#[doc = "UDS_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body3`]
module"]
#[doc(alias = "UDS_BODY3")]
pub type UdsBody3 = crate::Reg<uds_body3::UdsBody3Spec>;
#[doc = "."]
pub mod uds_body3;
#[doc = "UDS_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code5`]
module"]
#[doc(alias = "UDS_KEY_CODE5")]
pub type UdsKeyCode5 = crate::Reg<uds_key_code5::UdsKeyCode5Spec>;
#[doc = "."]
pub mod uds_key_code5;
#[doc = "UDS_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body4`]
module"]
#[doc(alias = "UDS_BODY4")]
pub type UdsBody4 = crate::Reg<uds_body4::UdsBody4Spec>;
#[doc = "."]
pub mod uds_body4;
#[doc = "UDS_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code6`]
module"]
#[doc(alias = "UDS_KEY_CODE6")]
pub type UdsKeyCode6 = crate::Reg<uds_key_code6::UdsKeyCode6Spec>;
#[doc = "."]
pub mod uds_key_code6;
#[doc = "UDS_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body5`]
module"]
#[doc(alias = "UDS_BODY5")]
pub type UdsBody5 = crate::Reg<uds_body5::UdsBody5Spec>;
#[doc = "."]
pub mod uds_body5;
#[doc = "UDS_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code7`]
module"]
#[doc(alias = "UDS_KEY_CODE7")]
pub type UdsKeyCode7 = crate::Reg<uds_key_code7::UdsKeyCode7Spec>;
#[doc = "."]
pub mod uds_key_code7;
#[doc = "UDS_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body6`]
module"]
#[doc(alias = "UDS_BODY6")]
pub type UdsBody6 = crate::Reg<uds_body6::UdsBody6Spec>;
#[doc = "."]
pub mod uds_body6;
#[doc = "UDS_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code8`]
module"]
#[doc(alias = "UDS_KEY_CODE8")]
pub type UdsKeyCode8 = crate::Reg<uds_key_code8::UdsKeyCode8Spec>;
#[doc = "."]
pub mod uds_key_code8;
#[doc = "UDS_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body7`]
module"]
#[doc(alias = "UDS_BODY7")]
pub type UdsBody7 = crate::Reg<uds_body7::UdsBody7Spec>;
#[doc = "."]
pub mod uds_body7;
#[doc = "UDS_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code9`]
module"]
#[doc(alias = "UDS_KEY_CODE9")]
pub type UdsKeyCode9 = crate::Reg<uds_key_code9::UdsKeyCode9Spec>;
#[doc = "."]
pub mod uds_key_code9;
#[doc = "UDS_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body8`]
module"]
#[doc(alias = "UDS_BODY8")]
pub type UdsBody8 = crate::Reg<uds_body8::UdsBody8Spec>;
#[doc = "."]
pub mod uds_body8;
#[doc = "UDS_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code10`]
module"]
#[doc(alias = "UDS_KEY_CODE10")]
pub type UdsKeyCode10 = crate::Reg<uds_key_code10::UdsKeyCode10Spec>;
#[doc = "."]
pub mod uds_key_code10;
#[doc = "UDS_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body9`]
module"]
#[doc(alias = "UDS_BODY9")]
pub type UdsBody9 = crate::Reg<uds_body9::UdsBody9Spec>;
#[doc = "."]
pub mod uds_body9;
#[doc = "UDS_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code11`]
module"]
#[doc(alias = "UDS_KEY_CODE11")]
pub type UdsKeyCode11 = crate::Reg<uds_key_code11::UdsKeyCode11Spec>;
#[doc = "."]
pub mod uds_key_code11;
#[doc = "UDS_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body10`]
module"]
#[doc(alias = "UDS_BODY10")]
pub type UdsBody10 = crate::Reg<uds_body10::UdsBody10Spec>;
#[doc = "."]
pub mod uds_body10;
#[doc = "UDS_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code12`]
module"]
#[doc(alias = "UDS_KEY_CODE12")]
pub type UdsKeyCode12 = crate::Reg<uds_key_code12::UdsKeyCode12Spec>;
#[doc = "."]
pub mod uds_key_code12;
#[doc = "UDS_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_body11`]
module"]
#[doc(alias = "UDS_BODY11")]
pub type UdsBody11 = crate::Reg<uds_body11::UdsBody11Spec>;
#[doc = "."]
pub mod uds_body11;
#[doc = "UDS_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`uds_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uds_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uds_key_code13`]
module"]
#[doc(alias = "UDS_KEY_CODE13")]
pub type UdsKeyCode13 = crate::Reg<uds_key_code13::UdsKeyCode13Spec>;
#[doc = "."]
pub mod uds_key_code13;
#[doc = "PRINCE_REGION0_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_header0`]
module"]
#[doc(alias = "PRINCE_REGION0_HEADER0")]
pub type PrinceRegion0Header0 =
    crate::Reg<prince_region0_header0::PrinceRegion0Header0Spec>;
#[doc = "."]
pub mod prince_region0_header0;
#[doc = "PRINCE_REGION0_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code0`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE0")]
pub type PrinceRegion0KeyCode0 =
    crate::Reg<prince_region0_key_code0::PrinceRegion0KeyCode0Spec>;
#[doc = "."]
pub mod prince_region0_key_code0;
#[doc = "PRINCE_REGION0_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_header1`]
module"]
#[doc(alias = "PRINCE_REGION0_HEADER1")]
pub type PrinceRegion0Header1 =
    crate::Reg<prince_region0_header1::PrinceRegion0Header1Spec>;
#[doc = "."]
pub mod prince_region0_header1;
#[doc = "PRINCE_REGION0_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code1`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE1")]
pub type PrinceRegion0KeyCode1 =
    crate::Reg<prince_region0_key_code1::PrinceRegion0KeyCode1Spec>;
#[doc = "."]
pub mod prince_region0_key_code1;
#[doc = "PRINCE_REGION0_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body0`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY0")]
pub type PrinceRegion0Body0 = crate::Reg<prince_region0_body0::PrinceRegion0Body0Spec>;
#[doc = "."]
pub mod prince_region0_body0;
#[doc = "PRINCE_REGION0_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code2`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE2")]
pub type PrinceRegion0KeyCode2 =
    crate::Reg<prince_region0_key_code2::PrinceRegion0KeyCode2Spec>;
#[doc = "."]
pub mod prince_region0_key_code2;
#[doc = "PRINCE_REGION0_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body1`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY1")]
pub type PrinceRegion0Body1 = crate::Reg<prince_region0_body1::PrinceRegion0Body1Spec>;
#[doc = "."]
pub mod prince_region0_body1;
#[doc = "PRINCE_REGION0_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code3`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE3")]
pub type PrinceRegion0KeyCode3 =
    crate::Reg<prince_region0_key_code3::PrinceRegion0KeyCode3Spec>;
#[doc = "."]
pub mod prince_region0_key_code3;
#[doc = "PRINCE_REGION0_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body2`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY2")]
pub type PrinceRegion0Body2 = crate::Reg<prince_region0_body2::PrinceRegion0Body2Spec>;
#[doc = "."]
pub mod prince_region0_body2;
#[doc = "PRINCE_REGION0_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code4`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE4")]
pub type PrinceRegion0KeyCode4 =
    crate::Reg<prince_region0_key_code4::PrinceRegion0KeyCode4Spec>;
#[doc = "."]
pub mod prince_region0_key_code4;
#[doc = "PRINCE_REGION0_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body3`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY3")]
pub type PrinceRegion0Body3 = crate::Reg<prince_region0_body3::PrinceRegion0Body3Spec>;
#[doc = "."]
pub mod prince_region0_body3;
#[doc = "PRINCE_REGION0_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code5`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE5")]
pub type PrinceRegion0KeyCode5 =
    crate::Reg<prince_region0_key_code5::PrinceRegion0KeyCode5Spec>;
#[doc = "."]
pub mod prince_region0_key_code5;
#[doc = "PRINCE_REGION0_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body4`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY4")]
pub type PrinceRegion0Body4 = crate::Reg<prince_region0_body4::PrinceRegion0Body4Spec>;
#[doc = "."]
pub mod prince_region0_body4;
#[doc = "PRINCE_REGION0_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code6`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE6")]
pub type PrinceRegion0KeyCode6 =
    crate::Reg<prince_region0_key_code6::PrinceRegion0KeyCode6Spec>;
#[doc = "."]
pub mod prince_region0_key_code6;
#[doc = "PRINCE_REGION0_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body5`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY5")]
pub type PrinceRegion0Body5 = crate::Reg<prince_region0_body5::PrinceRegion0Body5Spec>;
#[doc = "."]
pub mod prince_region0_body5;
#[doc = "PRINCE_REGION0_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code7`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE7")]
pub type PrinceRegion0KeyCode7 =
    crate::Reg<prince_region0_key_code7::PrinceRegion0KeyCode7Spec>;
#[doc = "."]
pub mod prince_region0_key_code7;
#[doc = "PRINCE_REGION0_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body6`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY6")]
pub type PrinceRegion0Body6 = crate::Reg<prince_region0_body6::PrinceRegion0Body6Spec>;
#[doc = "."]
pub mod prince_region0_body6;
#[doc = "PRINCE_REGION0_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code8`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE8")]
pub type PrinceRegion0KeyCode8 =
    crate::Reg<prince_region0_key_code8::PrinceRegion0KeyCode8Spec>;
#[doc = "."]
pub mod prince_region0_key_code8;
#[doc = "PRINCE_REGION0_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body7`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY7")]
pub type PrinceRegion0Body7 = crate::Reg<prince_region0_body7::PrinceRegion0Body7Spec>;
#[doc = "."]
pub mod prince_region0_body7;
#[doc = "PRINCE_REGION0_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code9`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE9")]
pub type PrinceRegion0KeyCode9 =
    crate::Reg<prince_region0_key_code9::PrinceRegion0KeyCode9Spec>;
#[doc = "."]
pub mod prince_region0_key_code9;
#[doc = "PRINCE_REGION0_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body8`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY8")]
pub type PrinceRegion0Body8 = crate::Reg<prince_region0_body8::PrinceRegion0Body8Spec>;
#[doc = "."]
pub mod prince_region0_body8;
#[doc = "PRINCE_REGION0_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code10`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE10")]
pub type PrinceRegion0KeyCode10 =
    crate::Reg<prince_region0_key_code10::PrinceRegion0KeyCode10Spec>;
#[doc = "."]
pub mod prince_region0_key_code10;
#[doc = "PRINCE_REGION0_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body9`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY9")]
pub type PrinceRegion0Body9 = crate::Reg<prince_region0_body9::PrinceRegion0Body9Spec>;
#[doc = "."]
pub mod prince_region0_body9;
#[doc = "PRINCE_REGION0_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code11`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE11")]
pub type PrinceRegion0KeyCode11 =
    crate::Reg<prince_region0_key_code11::PrinceRegion0KeyCode11Spec>;
#[doc = "."]
pub mod prince_region0_key_code11;
#[doc = "PRINCE_REGION0_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body10`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY10")]
pub type PrinceRegion0Body10 =
    crate::Reg<prince_region0_body10::PrinceRegion0Body10Spec>;
#[doc = "."]
pub mod prince_region0_body10;
#[doc = "PRINCE_REGION0_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code12`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE12")]
pub type PrinceRegion0KeyCode12 =
    crate::Reg<prince_region0_key_code12::PrinceRegion0KeyCode12Spec>;
#[doc = "."]
pub mod prince_region0_key_code12;
#[doc = "PRINCE_REGION0_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_body11`]
module"]
#[doc(alias = "PRINCE_REGION0_BODY11")]
pub type PrinceRegion0Body11 =
    crate::Reg<prince_region0_body11::PrinceRegion0Body11Spec>;
#[doc = "."]
pub mod prince_region0_body11;
#[doc = "PRINCE_REGION0_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region0_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region0_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region0_key_code13`]
module"]
#[doc(alias = "PRINCE_REGION0_KEY_CODE13")]
pub type PrinceRegion0KeyCode13 =
    crate::Reg<prince_region0_key_code13::PrinceRegion0KeyCode13Spec>;
#[doc = "."]
pub mod prince_region0_key_code13;
#[doc = "PRINCE_REGION1_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_header0`]
module"]
#[doc(alias = "PRINCE_REGION1_HEADER0")]
pub type PrinceRegion1Header0 =
    crate::Reg<prince_region1_header0::PrinceRegion1Header0Spec>;
#[doc = "."]
pub mod prince_region1_header0;
#[doc = "PRINCE_REGION1_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code0`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE0")]
pub type PrinceRegion1KeyCode0 =
    crate::Reg<prince_region1_key_code0::PrinceRegion1KeyCode0Spec>;
#[doc = "."]
pub mod prince_region1_key_code0;
#[doc = "PRINCE_REGION1_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_header1`]
module"]
#[doc(alias = "PRINCE_REGION1_HEADER1")]
pub type PrinceRegion1Header1 =
    crate::Reg<prince_region1_header1::PrinceRegion1Header1Spec>;
#[doc = "."]
pub mod prince_region1_header1;
#[doc = "PRINCE_REGION1_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code1`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE1")]
pub type PrinceRegion1KeyCode1 =
    crate::Reg<prince_region1_key_code1::PrinceRegion1KeyCode1Spec>;
#[doc = "."]
pub mod prince_region1_key_code1;
#[doc = "PRINCE_REGION1_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body0`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY0")]
pub type PrinceRegion1Body0 = crate::Reg<prince_region1_body0::PrinceRegion1Body0Spec>;
#[doc = "."]
pub mod prince_region1_body0;
#[doc = "PRINCE_REGION1_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code2`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE2")]
pub type PrinceRegion1KeyCode2 =
    crate::Reg<prince_region1_key_code2::PrinceRegion1KeyCode2Spec>;
#[doc = "."]
pub mod prince_region1_key_code2;
#[doc = "PRINCE_REGION1_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body1`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY1")]
pub type PrinceRegion1Body1 = crate::Reg<prince_region1_body1::PrinceRegion1Body1Spec>;
#[doc = "."]
pub mod prince_region1_body1;
#[doc = "PRINCE_REGION1_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code3`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE3")]
pub type PrinceRegion1KeyCode3 =
    crate::Reg<prince_region1_key_code3::PrinceRegion1KeyCode3Spec>;
#[doc = "."]
pub mod prince_region1_key_code3;
#[doc = "PRINCE_REGION1_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body2`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY2")]
pub type PrinceRegion1Body2 = crate::Reg<prince_region1_body2::PrinceRegion1Body2Spec>;
#[doc = "."]
pub mod prince_region1_body2;
#[doc = "PRINCE_REGION1_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code4`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE4")]
pub type PrinceRegion1KeyCode4 =
    crate::Reg<prince_region1_key_code4::PrinceRegion1KeyCode4Spec>;
#[doc = "."]
pub mod prince_region1_key_code4;
#[doc = "PRINCE_REGION1_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body3`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY3")]
pub type PrinceRegion1Body3 = crate::Reg<prince_region1_body3::PrinceRegion1Body3Spec>;
#[doc = "."]
pub mod prince_region1_body3;
#[doc = "PRINCE_REGION1_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code5`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE5")]
pub type PrinceRegion1KeyCode5 =
    crate::Reg<prince_region1_key_code5::PrinceRegion1KeyCode5Spec>;
#[doc = "."]
pub mod prince_region1_key_code5;
#[doc = "PRINCE_REGION1_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body4`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY4")]
pub type PrinceRegion1Body4 = crate::Reg<prince_region1_body4::PrinceRegion1Body4Spec>;
#[doc = "."]
pub mod prince_region1_body4;
#[doc = "PRINCE_REGION1_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code6`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE6")]
pub type PrinceRegion1KeyCode6 =
    crate::Reg<prince_region1_key_code6::PrinceRegion1KeyCode6Spec>;
#[doc = "."]
pub mod prince_region1_key_code6;
#[doc = "PRINCE_REGION1_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body5`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY5")]
pub type PrinceRegion1Body5 = crate::Reg<prince_region1_body5::PrinceRegion1Body5Spec>;
#[doc = "."]
pub mod prince_region1_body5;
#[doc = "PRINCE_REGION1_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code7`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE7")]
pub type PrinceRegion1KeyCode7 =
    crate::Reg<prince_region1_key_code7::PrinceRegion1KeyCode7Spec>;
#[doc = "."]
pub mod prince_region1_key_code7;
#[doc = "PRINCE_REGION1_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body6`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY6")]
pub type PrinceRegion1Body6 = crate::Reg<prince_region1_body6::PrinceRegion1Body6Spec>;
#[doc = "."]
pub mod prince_region1_body6;
#[doc = "PRINCE_REGION1_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code8`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE8")]
pub type PrinceRegion1KeyCode8 =
    crate::Reg<prince_region1_key_code8::PrinceRegion1KeyCode8Spec>;
#[doc = "."]
pub mod prince_region1_key_code8;
#[doc = "PRINCE_REGION1_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body7`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY7")]
pub type PrinceRegion1Body7 = crate::Reg<prince_region1_body7::PrinceRegion1Body7Spec>;
#[doc = "."]
pub mod prince_region1_body7;
#[doc = "PRINCE_REGION1_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code9`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE9")]
pub type PrinceRegion1KeyCode9 =
    crate::Reg<prince_region1_key_code9::PrinceRegion1KeyCode9Spec>;
#[doc = "."]
pub mod prince_region1_key_code9;
#[doc = "PRINCE_REGION1_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body8`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY8")]
pub type PrinceRegion1Body8 = crate::Reg<prince_region1_body8::PrinceRegion1Body8Spec>;
#[doc = "."]
pub mod prince_region1_body8;
#[doc = "PRINCE_REGION1_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code10`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE10")]
pub type PrinceRegion1KeyCode10 =
    crate::Reg<prince_region1_key_code10::PrinceRegion1KeyCode10Spec>;
#[doc = "."]
pub mod prince_region1_key_code10;
#[doc = "PRINCE_REGION1_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body9`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY9")]
pub type PrinceRegion1Body9 = crate::Reg<prince_region1_body9::PrinceRegion1Body9Spec>;
#[doc = "."]
pub mod prince_region1_body9;
#[doc = "PRINCE_REGION1_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code11`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE11")]
pub type PrinceRegion1KeyCode11 =
    crate::Reg<prince_region1_key_code11::PrinceRegion1KeyCode11Spec>;
#[doc = "."]
pub mod prince_region1_key_code11;
#[doc = "PRINCE_REGION1_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body10`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY10")]
pub type PrinceRegion1Body10 =
    crate::Reg<prince_region1_body10::PrinceRegion1Body10Spec>;
#[doc = "."]
pub mod prince_region1_body10;
#[doc = "PRINCE_REGION1_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code12`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE12")]
pub type PrinceRegion1KeyCode12 =
    crate::Reg<prince_region1_key_code12::PrinceRegion1KeyCode12Spec>;
#[doc = "."]
pub mod prince_region1_key_code12;
#[doc = "PRINCE_REGION1_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_body11`]
module"]
#[doc(alias = "PRINCE_REGION1_BODY11")]
pub type PrinceRegion1Body11 =
    crate::Reg<prince_region1_body11::PrinceRegion1Body11Spec>;
#[doc = "."]
pub mod prince_region1_body11;
#[doc = "PRINCE_REGION1_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region1_key_code13`]
module"]
#[doc(alias = "PRINCE_REGION1_KEY_CODE13")]
pub type PrinceRegion1KeyCode13 =
    crate::Reg<prince_region1_key_code13::PrinceRegion1KeyCode13Spec>;
#[doc = "."]
pub mod prince_region1_key_code13;
#[doc = "PRINCE_REGION2_HEADER0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_header0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_header0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_header0`]
module"]
#[doc(alias = "PRINCE_REGION2_HEADER0")]
pub type PrinceRegion2Header0 =
    crate::Reg<prince_region2_header0::PrinceRegion2Header0Spec>;
#[doc = "."]
pub mod prince_region2_header0;
#[doc = "PRINCE_REGION2_KEY_CODE0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code0`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE0")]
pub type PrinceRegion2KeyCode0 =
    crate::Reg<prince_region2_key_code0::PrinceRegion2KeyCode0Spec>;
#[doc = "."]
pub mod prince_region2_key_code0;
#[doc = "PRINCE_REGION2_HEADER1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_header1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_header1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_header1`]
module"]
#[doc(alias = "PRINCE_REGION2_HEADER1")]
pub type PrinceRegion2Header1 =
    crate::Reg<prince_region2_header1::PrinceRegion2Header1Spec>;
#[doc = "."]
pub mod prince_region2_header1;
#[doc = "PRINCE_REGION2_KEY_CODE1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code1`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE1")]
pub type PrinceRegion2KeyCode1 =
    crate::Reg<prince_region2_key_code1::PrinceRegion2KeyCode1Spec>;
#[doc = "."]
pub mod prince_region2_key_code1;
#[doc = "PRINCE_REGION2_BODY0 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body0`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY0")]
pub type PrinceRegion2Body0 = crate::Reg<prince_region2_body0::PrinceRegion2Body0Spec>;
#[doc = "."]
pub mod prince_region2_body0;
#[doc = "PRINCE_REGION2_KEY_CODE2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code2`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE2")]
pub type PrinceRegion2KeyCode2 =
    crate::Reg<prince_region2_key_code2::PrinceRegion2KeyCode2Spec>;
#[doc = "."]
pub mod prince_region2_key_code2;
#[doc = "PRINCE_REGION2_BODY1 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body1`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY1")]
pub type PrinceRegion2Body1 = crate::Reg<prince_region2_body1::PrinceRegion2Body1Spec>;
#[doc = "."]
pub mod prince_region2_body1;
#[doc = "PRINCE_REGION2_KEY_CODE3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code3`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE3")]
pub type PrinceRegion2KeyCode3 =
    crate::Reg<prince_region2_key_code3::PrinceRegion2KeyCode3Spec>;
#[doc = "."]
pub mod prince_region2_key_code3;
#[doc = "PRINCE_REGION2_BODY2 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body2`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY2")]
pub type PrinceRegion2Body2 = crate::Reg<prince_region2_body2::PrinceRegion2Body2Spec>;
#[doc = "."]
pub mod prince_region2_body2;
#[doc = "PRINCE_REGION2_KEY_CODE4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code4`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE4")]
pub type PrinceRegion2KeyCode4 =
    crate::Reg<prince_region2_key_code4::PrinceRegion2KeyCode4Spec>;
#[doc = "."]
pub mod prince_region2_key_code4;
#[doc = "PRINCE_REGION2_BODY3 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body3`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY3")]
pub type PrinceRegion2Body3 = crate::Reg<prince_region2_body3::PrinceRegion2Body3Spec>;
#[doc = "."]
pub mod prince_region2_body3;
#[doc = "PRINCE_REGION2_KEY_CODE5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code5`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE5")]
pub type PrinceRegion2KeyCode5 =
    crate::Reg<prince_region2_key_code5::PrinceRegion2KeyCode5Spec>;
#[doc = "."]
pub mod prince_region2_key_code5;
#[doc = "PRINCE_REGION2_BODY4 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body4`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY4")]
pub type PrinceRegion2Body4 = crate::Reg<prince_region2_body4::PrinceRegion2Body4Spec>;
#[doc = "."]
pub mod prince_region2_body4;
#[doc = "PRINCE_REGION2_KEY_CODE6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code6`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE6")]
pub type PrinceRegion2KeyCode6 =
    crate::Reg<prince_region2_key_code6::PrinceRegion2KeyCode6Spec>;
#[doc = "."]
pub mod prince_region2_key_code6;
#[doc = "PRINCE_REGION2_BODY5 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body5`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY5")]
pub type PrinceRegion2Body5 = crate::Reg<prince_region2_body5::PrinceRegion2Body5Spec>;
#[doc = "."]
pub mod prince_region2_body5;
#[doc = "PRINCE_REGION2_KEY_CODE7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code7`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE7")]
pub type PrinceRegion2KeyCode7 =
    crate::Reg<prince_region2_key_code7::PrinceRegion2KeyCode7Spec>;
#[doc = "."]
pub mod prince_region2_key_code7;
#[doc = "PRINCE_REGION2_BODY6 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body6`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY6")]
pub type PrinceRegion2Body6 = crate::Reg<prince_region2_body6::PrinceRegion2Body6Spec>;
#[doc = "."]
pub mod prince_region2_body6;
#[doc = "PRINCE_REGION2_KEY_CODE8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code8`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE8")]
pub type PrinceRegion2KeyCode8 =
    crate::Reg<prince_region2_key_code8::PrinceRegion2KeyCode8Spec>;
#[doc = "."]
pub mod prince_region2_key_code8;
#[doc = "PRINCE_REGION2_BODY7 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body7`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY7")]
pub type PrinceRegion2Body7 = crate::Reg<prince_region2_body7::PrinceRegion2Body7Spec>;
#[doc = "."]
pub mod prince_region2_body7;
#[doc = "PRINCE_REGION2_KEY_CODE9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code9`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE9")]
pub type PrinceRegion2KeyCode9 =
    crate::Reg<prince_region2_key_code9::PrinceRegion2KeyCode9Spec>;
#[doc = "."]
pub mod prince_region2_key_code9;
#[doc = "PRINCE_REGION2_BODY8 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body8`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY8")]
pub type PrinceRegion2Body8 = crate::Reg<prince_region2_body8::PrinceRegion2Body8Spec>;
#[doc = "."]
pub mod prince_region2_body8;
#[doc = "PRINCE_REGION2_KEY_CODE10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code10`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE10")]
pub type PrinceRegion2KeyCode10 =
    crate::Reg<prince_region2_key_code10::PrinceRegion2KeyCode10Spec>;
#[doc = "."]
pub mod prince_region2_key_code10;
#[doc = "PRINCE_REGION2_BODY9 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body9`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY9")]
pub type PrinceRegion2Body9 = crate::Reg<prince_region2_body9::PrinceRegion2Body9Spec>;
#[doc = "."]
pub mod prince_region2_body9;
#[doc = "PRINCE_REGION2_KEY_CODE11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code11`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE11")]
pub type PrinceRegion2KeyCode11 =
    crate::Reg<prince_region2_key_code11::PrinceRegion2KeyCode11Spec>;
#[doc = "."]
pub mod prince_region2_key_code11;
#[doc = "PRINCE_REGION2_BODY10 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body10`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY10")]
pub type PrinceRegion2Body10 =
    crate::Reg<prince_region2_body10::PrinceRegion2Body10Spec>;
#[doc = "."]
pub mod prince_region2_body10;
#[doc = "PRINCE_REGION2_KEY_CODE12 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code12`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE12")]
pub type PrinceRegion2KeyCode12 =
    crate::Reg<prince_region2_key_code12::PrinceRegion2KeyCode12Spec>;
#[doc = "."]
pub mod prince_region2_key_code12;
#[doc = "PRINCE_REGION2_BODY11 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_body11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_body11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_body11`]
module"]
#[doc(alias = "PRINCE_REGION2_BODY11")]
pub type PrinceRegion2Body11 =
    crate::Reg<prince_region2_body11::PrinceRegion2Body11Spec>;
#[doc = "."]
pub mod prince_region2_body11;
#[doc = "PRINCE_REGION2_KEY_CODE13 (rw) register accessor: .\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prince_region2_key_code13`]
module"]
#[doc(alias = "PRINCE_REGION2_KEY_CODE13")]
pub type PrinceRegion2KeyCode13 =
    crate::Reg<prince_region2_key_code13::PrinceRegion2KeyCode13Spec>;
#[doc = "."]
pub mod prince_region2_key_code13;
