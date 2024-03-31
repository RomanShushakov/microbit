// // #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::interrupt::free;
use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::
{
    board::Board, display::blocking::Display, 
    hal::{gpio, prelude::*, pwm::{self, Pwm}, time::Hertz, timer::{self, Periodic}, Timer},
};

mod number;
use number::Number;

mod buttons;
use buttons::{Pressed, init_buttons, get_pressed};


const ROUND_DURATION: u32 = 180_000_000;    // 3 min in microseconds
const REST_DURATION: u32 = 60_000_000;      // 1 min in microseconds
const WHISTLE: u32 = 10_000_000;            // 10 secs in microseconds


#[derive(PartialEq, Eq)]
enum State
{
    WarmingUp,
    Round,
    RoundWhistle,
    Rest,
    RestWhistle,
    Stop,
}


fn reset(round: &mut Number, sec: &mut Number, state: &mut State, is_timer_running: &mut bool, whistle: &mut u32)
{
    *round = Number::Zero;
    *sec = Number::Zero;
    *state = State::Stop;
    *is_timer_running = false;
    *whistle = WHISTLE;
}


fn run_countdown<T, V>(
    is_timer_running: &mut bool,
    timer: &mut Timer<T, Periodic>,
    ticks_per_second: u32,
    sec: &mut Number,
    whistle: &mut u32,
    speaker: &mut Pwm<V>,
)
    where T: timer::Instance,
          V: pwm::Instance,
{
    if !*is_timer_running
    {
        rprintln!("Start timer");
        timer.start(ticks_per_second);
        *is_timer_running = true;
    }
    else
    {
        if let Ok(_) = timer.wait()
        {
            free(|cs| 
            {
                if *whistle == WHISTLE
                {
                    speaker.set_prescaler(pwm::Prescaler::Div128);
                    speaker.set_duty_on_common(32000);
                } 
                else if *whistle <= 3_000_000 && *whistle > 1_000_000
                {
                    speaker.set_prescaler(pwm::Prescaler::Div1);
                    speaker.set_duty_on_common(32000);
                }
                else
                {
                    speaker.stop();
                }
            });

            sec.previous();
            *whistle -= ticks_per_second;
            rprintln!("Running timer: {}", whistle);
            if *whistle == 0
            {
                rprintln!("Stop timer");
                *is_timer_running = false;
                *whistle = WHISTLE;
            }
        }
    }
}


#[entry]
fn main() -> ! 
{
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0).into_periodic();
    let mut timer_1 = Timer::new(board.TIMER1);
    let mut display = Display::new(board.display_pins);

    let speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
    let mut speaker = pwm::Pwm::new(board.PWM0);

    speaker
        // output the waveform on the speaker pin
        .set_output_pin(pwm::Channel::C0, speaker_pin.degrade())
        // Initial frequency
        .set_period(Hertz(1))
        // Configure for up and down counter mode
        .set_counter_mode(pwm::CounterMode::UpAndDown)
        // Set maximum duty cycle
        .set_max_duty(32767)
        // enable PWM
        .enable();

    speaker
        .set_seq_refresh(pwm::Seq::Seq0, 0)
        .set_seq_end_delay(pwm::Seq::Seq0, 0);

    init_buttons(board.GPIOTE, board.buttons);

    let mut state = State::Stop;
    
    let mut round = Number::Zero;
    let mut sec = Number::Zero;
    let mut is_timer_running = false;

    let mut whistle = WHISTLE;
    let ticks_per_second: u32 = 1_000_000;

    loop 
    {

        match state
        {
            State::Stop => 
            {
                rprintln!("Timer stopped");
                display.show(&mut timer_1, round.convert_to_lights(), 10);
            },
            State::WarmingUp =>
            {
                rprintln!("Warming up");
                display.show(&mut timer_1, sec.convert_to_lights(), 10);
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle, &mut speaker);
                if !is_timer_running
                {
                    state = State::Round;
                }
            },
            State::Round =>
            {
                rprintln!("Round {} start", round as u32);
                display.show(&mut timer_1, round.convert_to_lights(), 10);
                state = State::RoundWhistle;
            },
            State::RoundWhistle =>
            {
                rprintln!("Round {} end", round as u32);
                display.show(&mut timer_1, sec.convert_to_lights(), 10);
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle, &mut speaker);

                if !is_timer_running
                {
                    round.previous();
                }

                if round as u32 == 0
                {
                    reset(&mut round, &mut sec, &mut state, &mut is_timer_running, &mut whistle);
                    state = State::Stop;
                }
                else
                {
                    if !is_timer_running
                    {
                        state = State::Rest;
                    }
                }
            },
            State::Rest =>
            {
                rprintln!("Rest before round {} start", round as u32);
                display.show(&mut timer_1, round.convert_to_lights(), 10);
                state = State::RestWhistle;
            },
            State::RestWhistle =>
            {
                rprintln!("Rest before round {} end", round as u32);
                display.show(&mut timer_1, sec.convert_to_lights(), 10);
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle, &mut speaker);
                if !is_timer_running
                {
                    state = State::Round;
                }
            },
        }


        match get_pressed(true)
        {
            Pressed::ButtonA => 
            {
                rprintln!("Button A clicked!");
                if state != State::Stop
                {
                    reset(&mut round, &mut sec, &mut state, &mut is_timer_running, &mut whistle);
                }
                else
                {
                    state = State::WarmingUp;
                }
            },
            Pressed::ButtonB => 
            {
                rprintln!("Button B clicked!");
                if state == State::Stop
                {
                    display.clear();
                    round.next();
                }
            },
            _ => ()
        }
    }
}
