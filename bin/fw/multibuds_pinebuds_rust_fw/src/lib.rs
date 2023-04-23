//! Main executable for the `MultiBuds` firmware (PineBuds)
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
extern crate panic_halt;

#[cfg(not(debug_assertions))]
extern crate panic_abort;

#[cortex_m_rt::entry]
fn main() -> ! {
    cortex_m::asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
