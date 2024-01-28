#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use microbit::
{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};


mod serial_setup;
use serial_setup::UartePort;


#[entry]
fn main() -> ! 
{
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = 
    {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    
    write!(serial, "Write text here:\n").unwrap();
    nb::block!(serial.flush()).unwrap();

    loop 
    {
        if let Ok(byte) = nb::block!(serial.read())
        {
            rprintln!("The char typed by client: {}", char::from(byte));
            nb::block!(serial.write(byte)).unwrap();
            nb::block!(serial.flush()).unwrap();
        }
    }
}
