#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use core::cell;

use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB3;
use arduino_uno::prelude::*;
use arduino_uno::pwm;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u32 = 1024;
const TIMER_COUNTS: u32 = 125;

const MILLIS_INCREMENT: u32 = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

fn millis_init(tc0: arduino_uno::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

fn millis() -> u32 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

const notes: &[u16] = &[262, 294, 330, 349];

#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        let mut timer2 = pwm::Timer2Pwm::new(peripherals.TC2, pwm::Prescaler::Prescale64);

        let mut pin = pins.d11.into_output(&mut pins.ddr).into_pwm(&mut timer2);

        millis_init(peripherals.TC0);
        unsafe { avr_device::interrupt::enable() };

        pin.enable();

        const second: u32 = 100;
        let mut time = millis();

        let mut noteIt = 0;
        let mut currentNote = notes[noteIt];
        let mut targetTime = second + time;
        let mut rest = time + (second / 4);
        loop {
            time = millis();
            match time {
                // x if x > rest => ufmt::uwriteln!(&mut serial, "rest\r").void_unwrap(),
                x if x > targetTime => {
                    if noteIt == 3 {
                        noteIt = 0;
                    } else {
                        noteIt = noteIt + 1;
                    }
                    currentNote = notes[noteIt];
                    let mut inSec: f64 = 0.0;
                    if (time > 60) {
                        inSec = time as f64 / 60.0;
                    } else {
                        inSec = 0.0;
                    }
                    ufmt::uwrite!(&mut serial, "time {} {:?}\n\r", time, inSec as u32)
                        .void_unwrap();
                    // ufmt::uwrite!(&mut serial, "in seconds {:?}\t", inSec).void_unwrap();
                    // pin.set_duty(currentNote as u8);

                    time = millis();
                    targetTime = second + time;
                    rest = targetTime - ((second / 4) * 3);
                }
                _ => {}
            };
        }
    }
}
