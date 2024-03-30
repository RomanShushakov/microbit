// // #![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::
{
    board::Board, display::blocking::Display, hal::{prelude::*, timer::{Instance, Periodic}, Timer},
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


fn run_countdown<T>(
    is_timer_running: &mut bool,
    timer: &mut Timer<T, Periodic>,
    ticks_per_second: u32,
    sec: &mut Number,
    whistle: &mut u32,
)
    where T: Instance
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
    let speaker = board.speaker_pin;

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
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle);
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
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle);

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
                run_countdown(&mut is_timer_running, &mut timer, ticks_per_second, &mut sec, &mut whistle);
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





// #![no_main]
// #![no_std]

// use panic_rtt_target as _;
// use rtt_target::{rprintln, rtt_init_print};

// use core::cell::RefCell;
// use cortex_m::interrupt::Mutex;

// use libm::round;

// use cortex_m_rt::entry;
// use microbit::{
//     hal::prelude::*,
//     hal::{
//         clocks::Clocks,
//         gpio::{self, Output, Pin, PushPull},
//         pwm,
//         rtc::{Rtc, RtcInterrupt},
//         time::Hertz,
//         Timer,
//     },
//     pac::{self, interrupt},
//     Board,
// };

// const BASE_INTERVAL: u32 = 128;
// const DUTY_SPEAKER: u16 = 8;
// const DUTY_JACK: u16 = 4096;

// static OUT_PIN: Mutex<RefCell<Option<Pin<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
// static RTC: Mutex<RefCell<Option<Rtc<pac::RTC0>>>> = Mutex::new(RefCell::new(None));
// static SPEAKER: Mutex<RefCell<Option<pwm::Pwm<pac::PWM0>>>> = Mutex::new(RefCell::new(None));
// static INTERVAL: Mutex<RefCell<f64>> = Mutex::new(RefCell::new(BASE_INTERVAL as f64));

// // Used to control volume and tone depending on the output device
// static DUTY_DIVISOR: Mutex<RefCell<u16>> = Mutex::new(RefCell::new(DUTY_JACK));

// fn to_bpm(interval: f64) -> f64 {
//     round(BASE_INTERVAL as f64 / interval * 60.0)
// }

// fn to_interval(bpm: f64) -> f64 {
//     BASE_INTERVAL as f64 / bpm * 60.0
// }

// #[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     rprintln!("Start");

//     if let Some(mut board) = Board::take() {
//         let button_a = board.buttons.button_a.into_pullup_input();
//         let button_b = board.buttons.button_b.into_pullup_input();
//         let _clocks = Clocks::new(board.CLOCK)
//             .enable_ext_hfosc()
//             .set_lfclk_src_synth()
//             .start_lfclk();

//         let mut timer = Timer::new(board.TIMER0);

//         // Interrupt every 1/128 s  (32768 / 128 Hz) - 1 = 255
//         let prescaler = 255; // 128 Hz
//         let mut rtc = Rtc::new(board.RTC0, prescaler).unwrap();
//         rtc.enable_counter();
//         rtc.enable_interrupt(RtcInterrupt::Tick, Some(&mut board.NVIC));
//         rtc.enable_event(RtcInterrupt::Tick);

//         // For jack output use board.pins.p0_02 and large pin 0 on the board
//         let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::Low);
//         let mut jack_pin = board.pins.p0_02.into_push_pull_output(gpio::Level::Low);

//         // Use the PWM peripheral to generate a waveform for the speaker
//         let speaker = pwm::Pwm::new(board.PWM0);
//         speaker
//             .set_output_pin(pwm::Channel::C0, jack_pin.degrade())
//             .set_prescaler(pwm::Prescaler::Div2)
//             .set_period(Hertz(440u32))
//             .set_counter_mode(pwm::CounterMode::UpAndDown)
//             .set_max_duty(32767)
//             .enable();

//         speaker
//             .set_seq_refresh(pwm::Seq::Seq0, 0)
//             .set_seq_end_delay(pwm::Seq::Seq0, 0);

//         cortex_m::interrupt::free(move |cs| {
//             *RTC.borrow(cs).borrow_mut() = Some(rtc);
//             *SPEAKER.borrow(cs).borrow_mut() = Some(speaker);
//             *OUT_PIN.borrow(cs).borrow_mut() = Some(speaker_pin.degrade());

//             // Configure RTC interrupt
//             unsafe {
//                 pac::NVIC::unmask(pac::Interrupt::RTC0);
//             }
//             pac::NVIC::unpend(pac::Interrupt::RTC0);
//         });

//         loop {
//             // Switch the output between internal speaker and GPIO pin 02 on two buttons pressed
//             if let (Ok(true), Ok(true)) = (button_a.is_low(), button_b.is_low()) {
//                 rprintln!("Switch output");
//                 cortex_m::interrupt::free(|cs| {
//                     if let Some(speaker) = SPEAKER.borrow(cs).borrow_mut().as_mut() {
//                         let inner_pin = OUT_PIN.borrow(cs).borrow_mut().take();
//                         if let Some(new_pin) = inner_pin {
//                             if let Some(old_pin) = speaker.clear_output_pin(pwm::Channel::C0) {
//                                 speaker.disable(pwm::Channel::C0);

//                                 OUT_PIN.borrow(cs).borrow_mut().replace(old_pin);
//                                 let mut divisor = DUTY_DIVISOR.borrow(cs).borrow_mut();

//                                 if *divisor == DUTY_JACK {
//                                     rprintln!("Switching to speaker");
//                                     *divisor = DUTY_SPEAKER;
//                                     speaker.set_prescaler(pwm::Prescaler::Div16);
//                                 } else {
//                                     rprintln!("Switching to jack");
//                                     *divisor = DUTY_JACK;
//                                     speaker.set_prescaler(pwm::Prescaler::Div2);
//                                 }
//                             }
//                             speaker
//                                 .set_output_pin(pwm::Channel::C0, new_pin)
//                                 .enable_channel(pwm::Channel::C0);
//                         }
//                     }
//                 });
//                 timer.delay_ms(500_u32);
//                 continue;
//             }
//             if let Ok(true) = button_a.is_low() {
//                 cortex_m::interrupt::free(move |cs| {
//                     let mut interval = INTERVAL.borrow(cs).borrow_mut();
//                     let bpm = to_bpm(*interval) - 1.0;
//                     if bpm > 0.0 {
//                         *interval = to_interval(bpm);
//                     }
//                     rprintln!("----BPM: {}, interval: {}", bpm, interval);
//                 });
//                 timer.delay_ms(100_u32);
//             };

//             if let Ok(true) = button_b.is_low() {
//                 cortex_m::interrupt::free(move |cs| {
//                     let mut interval = INTERVAL.borrow(cs).borrow_mut();
//                     if *interval > 0.0 {
//                         let bpm = to_bpm(*interval) + 1.0;
//                         *interval = to_interval(bpm);
//                         rprintln!("----BPM: {}, interval: {}", bpm, interval);
//                     };
//                 });
//                 timer.delay_ms(100_u32);
//             };
//         }
//     }

//     loop {
//         continue;
//     }
// }

// // RTC interrupt, executed for each RTC tick
// #[interrupt]
// fn RTC0() {
//     static mut SLEEP_COUNTER: u32 = 0;

//     /* Enter critical section */
//     cortex_m::interrupt::free(|cs| {
//         if let (Some(speaker), Some(rtc)) = (
//             SPEAKER.borrow(cs).borrow().as_ref(),
//             RTC.borrow(cs).borrow().as_ref(),
//         ) {
//             let interval = INTERVAL.borrow(cs).borrow();
//             let duty_divisor = DUTY_DIVISOR.borrow(cs).borrow();

//             if *SLEEP_COUNTER as f64 >= *interval {
//                 let max_duty = speaker.max_duty();
//                 speaker.set_duty_on_common(max_duty / *duty_divisor);
//                 *SLEEP_COUNTER = 0;
//             } else {
//                 speaker.disable();
//             }

//             // Clear the RTC interrupt
//             rtc.reset_event(RtcInterrupt::Tick);
//         }
//     });
//     *SLEEP_COUNTER += 1;
// }