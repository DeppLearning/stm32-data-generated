#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Virtual 1-channel timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim1ch {}
unsafe impl Sync for Tim1ch {}
impl Tim1ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr1ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
#[doc = "2-channel timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim2ch {}
unsafe impl Sync for Tim2ch {}
impl Tim2ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr22ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr2ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
#[doc = "Basic timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasic {
    ptr: *mut u8,
}
unsafe impl Send for TimBasic {}
unsafe impl Sync for TimBasic {}
impl TimBasic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Basic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "Virtual Basic timers without CR2 register for common part of TIM_BASIC and TIM_1CH_CMP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasicNoCr2 {
    ptr: *mut u8,
}
unsafe impl Send for TimBasicNoCr2 {}
unsafe impl Sync for TimBasicNoCr2 {}
impl TimBasicNoCr2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "Virtual timer for common part of TIM_BASIC and TIM_1CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimCore {
    ptr: *mut u8,
}
unsafe impl Send for TimCore {}
unsafe impl Sync for TimCore {}
impl TimCore {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
#[doc = "General purpose 16-bit timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp16 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp16 {}
unsafe impl Sync for TimGp16 {}
impl TimGp16 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::PscCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::DcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::DmarGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Option register 1 Note: Check Reference Manual to parse this register content"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "auto-reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ArrCore(pub u32);
    impl ArrCore {
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ArrCore {
        #[inline(always)]
        fn default() -> ArrCore {
            ArrCore(0)
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer1ch(pub u32);
    impl Ccer1ch {
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer1ch {
        #[inline(always)]
        fn default() -> Ccer1ch {
            Ccer1ch(0)
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer2ch(pub u32);
    impl Ccer2ch {
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer2ch {
        #[inline(always)]
        fn default() -> Ccer2ch {
            Ccer2ch(0)
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerGp16(pub u32);
    impl CcerGp16 {
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerGp16 {
        #[inline(always)]
        fn default() -> CcerGp16 {
            CcerGp16(0)
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput1ch(pub u32);
    impl CcmrInput1ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput1ch {
        #[inline(always)]
        fn default() -> CcmrInput1ch {
            CcmrInput1ch(0)
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput2ch(pub u32);
    impl CcmrInput2ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput2ch {
        #[inline(always)]
        fn default() -> CcmrInput2ch {
            CcmrInput2ch(0)
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput1ch(pub u32);
    impl CcmrOutput1ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for CcmrOutput1ch {
        #[inline(always)]
        fn default() -> CcmrOutput1ch {
            CcmrOutput1ch(0)
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput2ch(pub u32);
    impl CcmrOutput2ch {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for CcmrOutput2ch {
        #[inline(always)]
        fn default() -> CcmrOutput2ch {
            CcmrOutput2ch(0)
        }
    }
    #[doc = "capture/compare mode register x (x=1-2) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutputGp16(pub u32);
    impl CcmrOutputGp16 {
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Output compare y clear enable"]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y clear enable"]
        #[inline(always)]
        pub fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutputGp16 {
        #[inline(always)]
        fn default() -> CcmrOutputGp16 {
            CcmrOutputGp16(0)
        }
    }
    #[doc = "capture/compare register x (x=1-4,6)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr1ch(pub u32);
    impl Ccr1ch {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr1ch {
        #[inline(always)]
        fn default() -> Ccr1ch {
            Ccr1ch(0)
        }
    }
    #[doc = "counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntCore(pub u32);
    impl CntCore {
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub const fn uifcpy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UIF copy"]
        #[inline(always)]
        pub fn set_uifcpy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CntCore {
        #[inline(always)]
        fn default() -> CntCore {
            CntCore(0)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr11ch(pub u32);
    impl Cr11ch {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Cr11ch {
        #[inline(always)]
        fn default() -> Cr11ch {
            Cr11ch(0)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Core(pub u32);
    impl Cr1Core {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Cr1Core {
        #[inline(always)]
        fn default() -> Cr1Core {
            Cr1Core(0)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Gp16(pub u32);
    impl Cr1Gp16 {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub const fn cms(&self) -> super::vals::Cms {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cms::from_bits(val as u8)
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub fn set_cms(&mut self, val: super::vals::Cms) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub const fn uifremap(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "UIF status bit remapping enable"]
        #[inline(always)]
        pub fn set_uifremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Cr1Gp16 {
        #[inline(always)]
        fn default() -> Cr1Gp16 {
            Cr1Gp16(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr22ch(pub u32);
    impl Cr22ch {
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr22ch {
        #[inline(always)]
        fn default() -> Cr22ch {
            Cr22ch(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Basic(pub u32);
    impl Cr2Basic {
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Cr2Basic {
        #[inline(always)]
        fn default() -> Cr2Basic {
            Cr2Basic(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Gp16(pub u32);
    impl Cr2Gp16 {
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr2Gp16 {
        #[inline(always)]
        fn default() -> Cr2Gp16 {
            Cr2Gp16(0)
        }
    }
    #[doc = "DMA control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcrGp16(pub u32);
    impl DcrGp16 {
        #[doc = "DMA base address"]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address"]
        #[inline(always)]
        pub fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "DMA burst source selection"]
        #[inline(always)]
        pub const fn dbss(&self) -> super::vals::Dbss {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Dbss::from_bits(val as u8)
        }
        #[doc = "DMA burst source selection"]
        #[inline(always)]
        pub fn set_dbss(&mut self, val: super::vals::Dbss) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for DcrGp16 {
        #[inline(always)]
        fn default() -> DcrGp16 {
            DcrGp16(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier1ch(pub u32);
    impl Dier1ch {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier1ch {
        #[inline(always)]
        fn default() -> Dier1ch {
            Dier1ch(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier2ch(pub u32);
    impl Dier2ch {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Dier2ch {
        #[inline(always)]
        fn default() -> Dier2ch {
            Dier2ch(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierBasicNoCr2(pub u32);
    impl DierBasicNoCr2 {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DierBasicNoCr2 {
        #[inline(always)]
        fn default() -> DierBasicNoCr2 {
            DierBasicNoCr2(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierCore(pub u32);
    impl DierCore {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DierCore {
        #[inline(always)]
        fn default() -> DierCore {
            DierCore(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierGp16(pub u32);
    impl DierGp16 {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for DierGp16 {
        #[inline(always)]
        fn default() -> DierGp16 {
            DierGp16(0)
        }
    }
    #[doc = "DMA address for full transfer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmarGp16(pub u32);
    impl DmarGp16 {
        #[doc = "DMA register for burst accesses"]
        #[inline(always)]
        pub const fn dmab(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA register for burst accesses"]
        #[inline(always)]
        pub fn set_dmab(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DmarGp16 {
        #[inline(always)]
        fn default() -> DmarGp16 {
            DmarGp16(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr1ch(pub u32);
    impl Egr1ch {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr1ch {
        #[inline(always)]
        fn default() -> Egr1ch {
            Egr1ch(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr2ch(pub u32);
    impl Egr2ch {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Egr2ch {
        #[inline(always)]
        fn default() -> Egr2ch {
            Egr2ch(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrCore(pub u32);
    impl EgrCore {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EgrCore {
        #[inline(always)]
        fn default() -> EgrCore {
            EgrCore(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrGp16(pub u32);
    impl EgrGp16 {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for EgrGp16 {
        #[inline(always)]
        fn default() -> EgrGp16 {
            EgrGp16(0)
        }
    }
    #[doc = "prescaler"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PscCore(pub u32);
    impl PscCore {
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub const fn psc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub fn set_psc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PscCore {
        #[inline(always)]
        fn default() -> PscCore {
            PscCore(0)
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr2ch(pub u32);
    impl Smcr2ch {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::FilterValue {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub fn set_etf(&mut self, val: super::vals::FilterValue) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Smcr2ch {
        #[inline(always)]
        fn default() -> Smcr2ch {
            Smcr2ch(0)
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcrGp16(pub u32);
    impl SmcrGp16 {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::FilterValue {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub fn set_etf(&mut self, val: super::vals::FilterValue) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SmcrGp16 {
        #[inline(always)]
        fn default() -> SmcrGp16 {
            SmcrGp16(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1ch(pub u32);
    impl Sr1ch {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr1ch {
        #[inline(always)]
        fn default() -> Sr1ch {
            Sr1ch(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2ch(pub u32);
    impl Sr2ch {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr2ch {
        #[inline(always)]
        fn default() -> Sr2ch {
            Sr2ch(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrCore(pub u32);
    impl SrCore {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SrCore {
        #[inline(always)]
        fn default() -> SrCore {
            SrCore(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrGp16(pub u32);
    impl SrGp16 {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrGp16 {
        #[inline(always)]
        fn default() -> SrGp16 {
            SrGp16(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        ONCOMPARE = 0x0,
        #[doc = "CCx DMA request sent when update event occurs"]
        ONUPDATE = 0x01,
    }
    impl Ccds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccds {
        #[inline(always)]
        fn from(val: u8) -> Ccds {
            Ccds::from_bits(val)
        }
    }
    impl From<Ccds> for u8 {
        #[inline(always)]
        fn from(val: Ccds) -> u8 {
            Ccds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrInputCcs {
        _RESERVED_0 = 0x0,
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        TI4 = 0x01,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        TI3 = 0x02,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        TRC = 0x03,
    }
    impl CcmrInputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrInputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrInputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrInputCcs {
            CcmrInputCcs::from_bits(val)
        }
    }
    impl From<CcmrInputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrInputCcs) -> u8 {
            CcmrInputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        OUTPUT = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcmrOutputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrOutputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrOutputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrOutputCcs {
            CcmrOutputCcs::from_bits(val)
        }
    }
    impl From<CcmrOutputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrOutputCcs) -> u8 {
            CcmrOutputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckd {
        #[doc = "t_DTS = t_CK_INT"]
        DIV1 = 0x0,
        #[doc = "t_DTS = 2 × t_CK_INT"]
        DIV2 = 0x01,
        #[doc = "t_DTS = 4 × t_CK_INT"]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckd {
        #[inline(always)]
        fn from(val: u8) -> Ckd {
            Ckd::from_bits(val)
        }
    }
    impl From<Ckd> for u8 {
        #[inline(always)]
        fn from(val: Ckd) -> u8 {
            Ckd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EDGEALIGNED = 0x0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CENTERALIGNED1 = 0x01,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CENTERALIGNED2 = 0x02,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CENTERALIGNED3 = 0x03,
    }
    impl Cms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cms {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cms {
        #[inline(always)]
        fn from(val: u8) -> Cms {
            Cms::from_bits(val)
        }
    }
    impl From<Cms> for u8 {
        #[inline(always)]
        fn from(val: Cms) -> u8 {
            Cms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dbss {
        _RESERVED_0 = 0x0,
        #[doc = "Update"]
        UPDATE = 0x01,
        #[doc = "CC1"]
        CC1 = 0x02,
        #[doc = "CC2"]
        CC2 = 0x03,
        #[doc = "CC3"]
        CC3 = 0x04,
        #[doc = "CC4"]
        CC4 = 0x05,
        #[doc = "COM"]
        COM = 0x06,
        #[doc = "Trigger"]
        TRIGGER = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Dbss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbss {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbss {
        #[inline(always)]
        fn from(val: u8) -> Dbss {
            Dbss::from_bits(val)
        }
    }
    impl From<Dbss> for u8 {
        #[inline(always)]
        fn from(val: Dbss) -> u8 {
            Dbss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        UP = 0x0,
        #[doc = "Counter used as downcounter"]
        DOWN = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etp {
        #[doc = "ETR is noninverted, active at high level or rising edge"]
        NOTINVERTED = 0x0,
        #[doc = "ETR is inverted, active at low level or falling edge"]
        INVERTED = 0x01,
    }
    impl Etp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etp {
        #[inline(always)]
        fn from(val: u8) -> Etp {
            Etp::from_bits(val)
        }
    }
    impl From<Etp> for u8 {
        #[inline(always)]
        fn from(val: Etp) -> u8 {
            Etp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etps {
        #[doc = "Prescaler OFF"]
        DIV1 = 0x0,
        #[doc = "ETRP frequency divided by 2"]
        DIV2 = 0x01,
        #[doc = "ETRP frequency divided by 4"]
        DIV4 = 0x02,
        #[doc = "ETRP frequency divided by 8"]
        DIV8 = 0x03,
    }
    impl Etps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etps {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etps {
        #[inline(always)]
        fn from(val: u8) -> Etps {
            Etps::from_bits(val)
        }
    }
    impl From<Etps> for u8 {
        #[inline(always)]
        fn from(val: Etps) -> u8 {
            Etps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FilterValue {
        #[doc = "No filter, sampling is done at fDTS"]
        NOFILTER = 0x0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FCK_INT_N2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FCK_INT_N4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FCK_INT_N8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FDTS_DIV2_N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FDTS_DIV2_N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FDTS_DIV4_N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FDTS_DIV4_N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FDTS_DIV8_N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FDTS_DIV8_N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FDTS_DIV16_N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FDTS_DIV16_N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FDTS_DIV16_N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FDTS_DIV32_N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FDTS_DIV32_N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FDTS_DIV32_N8 = 0x0f,
    }
    impl FilterValue {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterValue {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterValue {
        #[inline(always)]
        fn from(val: u8) -> FilterValue {
            FilterValue::from_bits(val)
        }
    }
    impl From<FilterValue> for u8 {
        #[inline(always)]
        fn from(val: FilterValue) -> u8 {
            FilterValue::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mms {
        #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
        RESET = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
        ENABLE = 0x01,
        #[doc = "The update event is selected as trigger output"]
        UPDATE = 0x02,
        #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        COMPAREPULSE = 0x03,
        #[doc = "OC1REF signal is used as trigger output"]
        COMPAREOC1 = 0x04,
        #[doc = "OC2REF signal is used as trigger output"]
        COMPAREOC2 = 0x05,
        #[doc = "OC3REF signal is used as trigger output"]
        COMPAREOC3 = 0x06,
        #[doc = "OC4REF signal is used as trigger output"]
        COMPAREOC4 = 0x07,
    }
    impl Mms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms {
        #[inline(always)]
        fn from(val: u8) -> Mms {
            Mms::from_bits(val)
        }
    }
    impl From<Mms> for u8 {
        #[inline(always)]
        fn from(val: Mms) -> u8 {
            Mms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msm {
        #[doc = "No action"]
        NOSYNC = 0x0,
        #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
        SYNC = 0x01,
    }
    impl Msm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msm {
        #[inline(always)]
        fn from(val: u8) -> Msm {
            Msm::from_bits(val)
        }
    }
    impl From<Msm> for u8 {
        #[inline(always)]
        fn from(val: Msm) -> u8 {
            Msm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        FROZEN = 0x0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ACTIVEONMATCH = 0x01,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        INACTIVEONMATCH = 0x02,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        TOGGLE = 0x03,
        #[doc = "OCyREF is forced low"]
        FORCEINACTIVE = 0x04,
        #[doc = "OCyREF is forced high"]
        FORCEACTIVE = 0x05,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PWMMODE1 = 0x06,
        #[doc = "Inversely to PwmMode1"]
        PWMMODE2 = 0x07,
    }
    impl Ocm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocm {
        #[inline(always)]
        fn from(val: u8) -> Ocm {
            Ocm::from_bits(val)
        }
    }
    impl From<Ocm> for u8 {
        #[inline(always)]
        fn from(val: Ocm) -> u8 {
            Ocm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sms {
        #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
        DISABLED = 0x0,
        #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
        ENCODER_MODE_1 = 0x01,
        #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
        ENCODER_MODE_2 = 0x02,
        #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
        ENCODER_MODE_3 = 0x03,
        #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
        RESET_MODE = 0x04,
        #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
        GATED_MODE = 0x05,
        #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
        TRIGGER_MODE = 0x06,
        #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
        EXT_CLOCK_MODE = 0x07,
    }
    impl Sms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sms {
        #[inline(always)]
        fn from(val: u8) -> Sms {
            Sms::from_bits(val)
        }
    }
    impl From<Sms> for u8 {
        #[inline(always)]
        fn from(val: Sms) -> u8 {
            Sms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ti1s {
        #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
        NORMAL = 0x0,
        #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
        XOR = 0x01,
    }
    impl Ti1s {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ti1s {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ti1s {
        #[inline(always)]
        fn from(val: u8) -> Ti1s {
            Ti1s::from_bits(val)
        }
    }
    impl From<Ti1s> for u8 {
        #[inline(always)]
        fn from(val: Ti1s) -> u8 {
            Ti1s::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ts {
        #[doc = "Internal Trigger 0 (ITR0)"]
        ITR0 = 0x0,
        #[doc = "Internal Trigger 1 (ITR1)"]
        ITR1 = 0x01,
        #[doc = "Internal Trigger 2 (ITR2)"]
        ITR2 = 0x02,
        #[doc = "Internal Trigger 3 (ITR3)"]
        ITR3 = 0x03,
        #[doc = "TI1 Edge Detector (TI1F_ED)"]
        TI1F_ED = 0x04,
        #[doc = "Filtered Timer Input 1 (TI1FP1)"]
        TI1FP1 = 0x05,
        #[doc = "Filtered Timer Input 2 (TI2FP2)"]
        TI2FP2 = 0x06,
        #[doc = "External Trigger input (ETRF)"]
        ETRF = 0x07,
    }
    impl Ts {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ts {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ts {
        #[inline(always)]
        fn from(val: u8) -> Ts {
            Ts::from_bits(val)
        }
    }
    impl From<Ts> for u8 {
        #[inline(always)]
        fn from(val: Ts) -> u8 {
            Ts::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        ANYEVENT = 0x0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        COUNTERONLY = 0x01,
    }
    impl Urs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Urs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Urs {
        #[inline(always)]
        fn from(val: u8) -> Urs {
            Urs::from_bits(val)
        }
    }
    impl From<Urs> for u8 {
        #[inline(always)]
        fn from(val: Urs) -> u8 {
            Urs::to_bits(val)
        }
    }
}
