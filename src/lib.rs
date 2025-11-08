#![no_std]

use core::arch::asm;
use cortex_m::interrupt::{disable, enable};

#[unsafe(no_mangle)]
fn udiv(a: u32, b: u32) -> u32 {
    a / b
}

#[unsafe(no_mangle)]
fn prot_udiv(a: u32, b: u32) -> u32 {
    cortex_m::interrupt::free(|_| a / b)
}

#[inline(always)]
fn udiv_asm(a: u32, b: u32) -> u32 {
    let r: u32;
    unsafe {
        asm!(
            "udiv {r}, {a}, {b}",
            a = in(reg) a,
            b = in(reg) b,
            r = lateout(reg) r,
        );
    }
    r
}

#[unsafe(no_mangle)]
fn nested_udiv(a: u32, b: u32) -> u32 {
    mem_free(|| working_udiv(a, b))
}

#[unsafe(no_mangle)]
fn working_udiv(a: u32, b: u32) -> u32 {
    mem_free(|| udiv_asm(a, b))
}

static mut PRIMASK: bool = false;

#[inline]
pub fn mem_free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    disable();
    let old_primask = unsafe { PRIMASK };
    unsafe { PRIMASK = true };

    let r = f();

    if !old_primask {
        unsafe {
            PRIMASK = old_primask;
            enable()
        }
    }

    r
}
