#![allow(unused_imports)]

use aarch64_cpu::registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0};
use int_ratio::Ratio;
use tock_registers::interfaces::{Readable, Writeable};

static mut CNTPCT_TO_NANOS_RATIO: Ratio = Ratio::zero();
static mut NANOS_TO_CNTPCT_RATIO: Ratio = Ratio::zero();
/// RTC wall time offset in nanoseconds at monotonic time base.
static mut RTC_EPOCHOFFSET_NANOS: u64 = 0;

/// Returns the current clock time in hardware ticks.
#[inline]
pub fn current_ticks() -> u64 {
    CNTPCT_EL0.get()
}

/// Converts hardware ticks to nanoseconds.
#[inline]
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    unsafe { CNTPCT_TO_NANOS_RATIO.mul_trunc(ticks) }
}

/// Converts nanoseconds to hardware ticks.
#[inline]
pub fn nanos_to_ticks(nanos: u64) -> u64 {
    unsafe { NANOS_TO_CNTPCT_RATIO.mul_trunc(nanos) }
}

/// Return epoch offset in nanoseconds (wall time offset to monotonic clock start).
pub fn epochoffset_nanos() -> u64 {
    unsafe { RTC_EPOCHOFFSET_NANOS }
}

/// Set a one-shot timer.
///
/// A timer interrupt will be triggered at the given deadline (in nanoseconds).
#[cfg(all(feature = "irq", not(feature = "hv")))]
pub fn set_oneshot_timer(deadline_ns: u64) {
    let cnptct = CNTPCT_EL0.get();
    let cnptct_deadline = nanos_to_ticks(deadline_ns);
    if cnptct < cnptct_deadline {
        let interval = cnptct_deadline - cnptct;
        debug_assert!(interval <= u32::MAX as u64);
        CNTP_TVAL_EL0.set(interval);
    } else {
        CNTP_TVAL_EL0.set(0);
    }
}

#[cfg(all(feature = "irq", feature = "hv"))]
pub fn set_oneshot_timer(deadline_ns: u64) {
    let cnptct = CNTPCT_EL0.get();
    let cnptct_deadline = nanos_to_ticks(deadline_ns);
    if cnptct < cnptct_deadline {
        let interval = cnptct_deadline - cnptct;
        debug_assert!(interval <= u32::MAX as u64);
        unsafe {
            core::arch::asm!("msr CNTHP_TVAL_EL2, {}", in(reg) interval);
        }
    } else {
        unsafe {
            core::arch::asm!("msr CNTHP_TVAL_EL2, {0:x}", in(reg) 0);
        }
    }
}

/// Early stage initialization: stores the timer frequency.
pub(crate) fn init_early() {
    let freq = CNTFRQ_EL0.get();
    unsafe {
        CNTPCT_TO_NANOS_RATIO = Ratio::new(crate::time::NANOS_PER_SEC as u32, freq as u32);
        NANOS_TO_CNTPCT_RATIO = CNTPCT_TO_NANOS_RATIO.inverse();
    }

    // Make sure `RTC_PADDR` is valid in platform config file.
    #[cfg(feature = "rtc")]
    if axconfig::RTC_PADDR != 0 {
        use crate::mem::phys_to_virt;
        use arm_pl031::Rtc;
        use memory_addr::PhysAddr;

        const PL031_BASE: PhysAddr = pa!(axconfig::RTC_PADDR);

        let rtc = unsafe { Rtc::new(phys_to_virt(PL031_BASE).as_usize() as _) };
        // Get the current time in microseconds since the epoch (1970-01-01) from the aarch64 pl031 RTC.
        // Subtract the timer ticks to get the actual time when ArceOS was booted.
        let epoch_time_nanos = rtc.get_unix_timestamp() as u64 * 1_000_000_000;

        unsafe {
            RTC_EPOCHOFFSET_NANOS = epoch_time_nanos - ticks_to_nanos(current_ticks());
        }
    }
}

pub(crate) fn init_percpu() {
    #[cfg(all(feature = "irq", not(feature = "hv")))]
    {
        CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
        CNTP_TVAL_EL0.set(0);
    }
    #[cfg(all(feature = "irq", feature = "hv"))]
    {
        // ENABLE, bit [0]， Enables the timer.
        let ctl = 1;
        let tval = 0;
        unsafe {
            core::arch::asm!("msr CNTHP_CTL_EL2, {0:x}", in(reg) ctl);
            core::arch::asm!("msr CNTHP_TVAL_EL2, {0:x}", in(reg) tval);
        }
    }
    #[cfg(feature = "irq")]
    crate::platform::irq::set_enable(crate::platform::irq::TIMER_IRQ_NUM, true);
}
