#![no_std]
#![no_main]

use panic_halt as _;
use support::*;

#[entry]
fn func() -> ! {
    // let error: Result<(), ()> = Err(());
    // error.unwrap();

    let (mut user_led, mut delay) = support::init();

    loop {
        delay.delay_ms(50u16);
        user_led.toggle();
    }
}
