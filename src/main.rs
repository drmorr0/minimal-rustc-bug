#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(abi_avr_interrupt)]

use arduino_uno::atmega328p::TC0 as Timer0;
use avr_hal_generic::avr_device;
use heapless::{
    consts::U8,
    Vec,
};

extern crate panic_halt;
static mut TIMER0_CMPB_ITERS: u32 = 0;
static mut HANDLER: Option<Handler> = None;
static mut EXECUTOR: Option<Executor> = None;

struct Executor {
    data: Vec<u8, U8>,
}

impl Executor {
    #[inline(never)]
    fn push(&mut self, value: u8) {
        self.data.push(value);
    }
}

struct Handler {}

impl Handler {
    unsafe fn call(&mut self) {
        if let Some(executor) = EXECUTOR.as_mut() {
            if executor.data.len() == 0 {
                executor.push(123);
            }
        }
    }
}

fn init_timers(t0: &Timer0) {
    t0.tccr0b.write(|w| w.cs0().prescale_64());
    t0.tcnt0.write(|w| unsafe { w.bits(0) });
    t0.timsk0.write(|w| unsafe { w.bits(0x04) });
    t0.ocr0b.write(|w| unsafe { w.bits(0) });
}

#[avr_device::interrupt(atmega328p)]
unsafe fn TIMER0_COMPB() {
    TIMER0_CMPB_ITERS += 1;
    if TIMER0_CMPB_ITERS > 10 {
        if let Some(mut handler) = HANDLER.take() {
            handler.call();
        }
    }
}

#[arduino_uno::entry]
fn main() -> ! {
    let board = arduino_uno::Peripherals::take().unwrap();
    init_timers(&board.TC0);
    unsafe {
        EXECUTOR = Some(Executor { data: Vec::new() });
        HANDLER = Some(Handler {});
        avr_device::interrupt::enable();
    }
    loop {
        unsafe {
            llvm_asm!("ldi r26, 0xBE");
            llvm_asm!("ldi r27, 0xEF");
        }
    }
}
