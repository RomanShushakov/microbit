#![deny(unsafe_code)]
#![no_main]
#![no_std]


use core::ops::Range;
use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::
{
    board::Board, display::blocking::Display, hal::Timer,
};


const SIZE: usize = 5;


#[derive(Clone, Copy)]
enum Edge
{
    First,
    Second,
    Third,
    Fourth,
}


fn path(edge: Edge) -> (Range<usize>, Range<usize>)
{
    match edge
    {
        Edge::First =>
        {
            return (0..1, 1..SIZE);
        },
        Edge::Second =>
        {
            return (1..SIZE, SIZE - 1..SIZE);
        },
        Edge::Third =>
        {
            return (SIZE - 1..SIZE, 0..SIZE - 1);
        },
        Edge::Fourth =>
        {
            return (0..SIZE - 1, 0..1);
        },
    }
}



#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut lights = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut edge = Edge::First;

    loop 
    {
        let (rng_i, rng_j) = path(edge);

        let mut blink = |i: usize, j: usize| 
        {
            lights[i][j] = 1;
            display.show(&mut timer, lights, 200);
            lights[i][j] = 0;
        };

        match edge
        {
            Edge::First => 
            { 
                for i in rng_i
                {
                    for j in rng_j.clone()
                    {
                        blink(i, j);
                    }
                }
                edge = Edge::Second;
            },
            Edge::Second => 
            {
                for i in rng_i
                {
                    for j in rng_j.clone()
                    {
                        blink(i, j);
                    }
                }
                edge = Edge::Third;
            },
            Edge::Third => 
            {
                for i in rng_i
                {
                    for j in rng_j.clone().rev()
                    {
                        blink(i, j);
                    }
                }
                edge = Edge::Fourth;          
            },
            Edge::Fourth => 
            {
                for i in rng_i.rev()
                {
                    for j in rng_j.clone()
                    {
                        blink(i, j);
                    }
                }
                edge = Edge::First;  
            },
        }
    }
}
