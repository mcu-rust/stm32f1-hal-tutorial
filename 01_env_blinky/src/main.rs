#![no_std]
#![no_main]

use core::panic::PanicInfo;
use heap1::{Heap, Inline};
use stm32f1_hal::{
    Mcu,
    cortex_m::{self, asm},
    cortex_m_rt::entry,
    gpio::PinState,
    os_trait::Timeout,
    pac,
    prelude::*,
    raw_os::RawOs as OS,
    rcc,
};

#[global_allocator]
static HEAP: Heap<Inline<10_000>> = Heap::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    asm::bkpt();
    loop {}
}

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // clock --------------------------------------------------------

    let cfg = rcc::Config::hse(8.MHz()).sysclk(72.MHz());
    let mut rcc = dp.RCC.init().freeze(cfg, &mut dp.FLASH.init().acr);

    // prepare ------------------------------------------------------

    let afio = dp.AFIO.init(&mut rcc);
    let mut mcu = Mcu::new(rcc, afio, cp.SCB.init(), cp.NVIC.init(), dp.EXTI);
    // sys_tick timer
    cp.SYST.counter_hz(&mcu).start(20.Hz()).unwrap();

    // LED ----------------------------------------------------------

    let gpiob = dp.GPIOB.split(&mut mcu.rcc);
    let mut led = gpiob.pb0.into_open_drain_output_with_state(PinState::High);

    let mut interval = Timeout::<OS>::millis(500);
    loop {
        if interval.timeout() {
            led.toggle();
        }
    }
}
