#![deny(unsafe_code)]
#![no_main]
#![no_std]


use core::ops::Range;
use core::iter::Rev;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::
{
    board::Board, display::blocking::Display, hal::{Timer, prelude::InputPin},
};


const SIZE: usize = 5;


enum Edge
{
    FirstFwd,
    SecondFwd,
    ThirdFwd,
    FourthFwd,
    FirstRev,
    SecondRev,
    ThirdRev,
    FourthRev,
}


enum Path
{
    RowConstColFwd(Range<usize>, Range<usize>),
    RowFwdColConst(Range<usize>, Range<usize>),
    RowRevColConst(Rev<Range<usize>>, Range<usize>),
    RowConstColRev(Range<usize>, Rev<Range<usize>>),
}


fn get_path(edge: &Edge) -> Path
{
    match edge
    {
        Edge::FirstFwd => Path::RowConstColFwd(0..1, 1..SIZE),
        Edge::SecondFwd => Path::RowFwdColConst(1..SIZE, SIZE - 1..SIZE),
        Edge::ThirdFwd => Path::RowConstColRev(SIZE - 1..SIZE, (0..SIZE - 1).rev()),
        Edge::FourthFwd => Path::RowRevColConst((0..SIZE - 1).rev(), 0..1),
        Edge::FirstRev => Path::RowConstColRev(0..1, (1..SIZE).rev()),
        Edge::SecondRev => Path::RowRevColConst((1..SIZE).rev(), SIZE - 1..SIZE),
        Edge::ThirdRev => Path::RowConstColFwd(SIZE - 1..SIZE, 0..SIZE - 1),
        Edge::FourthRev => Path::RowFwdColConst(0..SIZE - 1, 0..1),
    }
}


#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;

    let mut lights = [[0; SIZE]; SIZE];

    let mut edge = Edge::FirstFwd;

    loop 
    {
        let mut blink = |i: usize, j: usize| 
        {
            lights[i][j] = 1;
            display.show(&mut timer, lights, 200);
            lights[i][j] = 0;
        };

        let path = get_path(&edge);

        match path
        {
            Path::RowConstColFwd(rng_i, rng_j) => 
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
            Path::RowFwdColConst(rng_i, rng_j) => 
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
            Path::RowConstColRev(rng_i, rng_j)  =>
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
            Path::RowRevColConst(rng_i, rng_j) =>
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
        }

        match edge
        {
            Edge::FirstFwd => edge = Edge::SecondFwd,
            Edge::SecondFwd => edge = Edge::ThirdFwd,
            Edge::ThirdFwd => edge = Edge::FourthFwd,
            Edge::FourthFwd => edge = Edge::FirstFwd,
            Edge::FirstRev => edge = Edge::FourthRev,
            Edge::FourthRev => edge = Edge::ThirdRev,
            Edge::ThirdRev => edge = Edge::SecondRev,
            Edge::SecondRev => edge = Edge::FirstRev,
        }


        if let Ok(is_button_b_clicked) = button_b.is_low()
        {
            if is_button_b_clicked
            {
                edge = Edge::FirstFwd;
            }
        }


        if let Ok(is_button_a_clicked) = button_a.is_low()
        {
            if is_button_a_clicked
            {
                edge = Edge::FirstRev;
            }
        }
    }
}
