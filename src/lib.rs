#![no_std]

#[unsafe(no_mangle)]
fn udiv(a: u32, b: u32) -> u32 {
    a / b
}

#[unsafe(no_mangle)]
fn prot_udiv(a: u32, b: u32) -> u32 {
    cortex_m::interrupt::free(|_|{a / b})
}
