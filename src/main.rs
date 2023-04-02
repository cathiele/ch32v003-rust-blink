#![no_std]
#![no_main]

use riscv_rt::entry;
use panic_halt as _;
use riscv as _;

use ch32v0::ch32v003 as pac;

#[no_mangle]

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;

    rcc.apb2pcenr.modify(|_, w| w.iopcen().set_bit());

    let gpioc = peripherals.GPIOC;

    unsafe {
        gpioc
            .cfglr
            .modify(|_, w| w.cnf4().bits(0b00).mode4().bits(0b11))
    };

    let cycle = 8_000_000 / 4;
    loop {
        gpioc.outdr.modify(|_, w| w.odr4().set_bit());
        for _ in 0..cycle {
            unsafe {
                riscv::asm::nop();
            }
        }

        gpioc.outdr.modify(|_, w| w.odr4().clear_bit());
        for _ in 0..cycle {
            unsafe {
                riscv::asm::nop();
            }
        }
    }
}

