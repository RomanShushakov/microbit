use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::{board::Buttons, hal::gpiote::Gpiote, pac::{self, interrupt}};


static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static PRESSED: Mutex<RefCell<Pressed>> = Mutex::new(RefCell::new(Pressed::None));


#[derive(Clone, Copy)]
pub enum Pressed
{
    ButtonA,
    ButtonB,
    None,
}


pub fn init_buttons(board_gpiote: pac::GPIOTE, board_buttons: Buttons) 
{
    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board_buttons.button_a.degrade())
        .hi_to_lo()
        .lo_to_hi()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board_buttons.button_b.degrade())
        .hi_to_lo()
        .lo_to_hi()
        .enable_interrupt();
    channel1.reset_events();

    free(move |cs| 
    {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe 
        {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });
}


pub fn get_pressed(reset: bool) -> Pressed
{
    free(|cs| 
    {
        let pressed = *PRESSED.borrow(cs).borrow();
        if reset 
        {
            *PRESSED.borrow(cs).borrow_mut() = Pressed::None
        }
        pressed
    })
}


#[pac::interrupt]
fn GPIOTE() 
{
    free(|cs| 
    {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() 
        {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let pressed = match (a_pressed, b_pressed) 
            {
                (true, false) => Pressed::ButtonA,
                (false, true) => Pressed::ButtonB,
                _ => Pressed::None,
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            *PRESSED.borrow(cs).borrow_mut() = pressed;
        }
    });
}
