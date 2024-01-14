#![deny(unsafe_code)]
#![no_main]
#![no_std]


use core::ops::Range;
use core::iter::Rev;
use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::
{
    board::Board, display::blocking::Display, hal::Timer,
};


const SIZE: usize = 5;


enum Edge
{
    First,
    Second,
    Third,
    Fourth,
}


enum Path
{
    RowFwdColFwd(Range<usize>, Range<usize>),
    RowRevColFwd(Rev<Range<usize>>, Range<usize>),
    RowFwdColRev(Range<usize>, Rev<Range<usize>>),
}


fn get_path(edge: &Edge) -> Path
{
    match edge
    {
        Edge::First => Path::RowFwdColFwd(0..1, 1..SIZE),
        Edge::Second => Path::RowFwdColFwd(1..SIZE, SIZE - 1..SIZE),
        Edge::Third => Path::RowFwdColRev(SIZE - 1..SIZE, (0..SIZE - 1).rev()),
        Edge::Fourth => Path::RowRevColFwd((0..SIZE - 1).rev(), 0..1),
    }
}


#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut lights = [[0; SIZE]; SIZE];

    let mut edge = Edge::First;

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
            Path::RowFwdColFwd(rng_i, rng_j) => 
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
            Path::RowFwdColRev(rng_i, rng_j)  =>
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
            Path::RowRevColFwd(rng_i, rng_j) =>
                rng_i.for_each(|i| { rng_j.clone().for_each(|j| blink(i, j)) }),
        }

        match edge
        {
            Edge::First => edge = Edge::Second,
            Edge::Second => edge = Edge::Third,
            Edge::Third => edge = Edge::Fourth,
            Edge::Fourth => edge = Edge::First,
        }
    }
}
