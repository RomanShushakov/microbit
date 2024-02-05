#![deny(unsafe_code)]
#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use microbit::
{
    hal::{twim, Delay}, pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, AccelMode};


#[entry]
fn main() -> !
{
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();

    let mut delay = Delay::new(board.SYST);
    sensor.set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50).unwrap();
    loop 
    {
        if sensor.accel_status().unwrap().xyz_new_data()
        {
            let data = sensor.acceleration().unwrap();
            // RTT instead of normal print
            rprintln!("Acceleration: x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
        }
    }
}
