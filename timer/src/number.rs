#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Number
{
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}


impl Number
{
    pub fn convert_to_lights(&self) -> [[u8; 5]; 5]
    {
        match self
        {
            Number::Zero => 
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Number::One =>
            {
                [
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                ]
            },
            Number::Two =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Number::Three =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                ]
            },
            Number::Four =>
            {
                [
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                ]
            },
            Number::Five =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Number::Six =>
            {
                [
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Number::Seven =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 0, 0, 0],
                ]
            },
            Number::Eight =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Number::Nine =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                ]
            },
        }
    }   


    pub fn next(&mut self)
    {
        match self
        {
            Number::Zero => *self = Number::One,
            Number::One => *self = Number::Two,
            Number::Two => *self = Number::Three,
            Number::Three => *self = Number::Four,
            Number::Four => *self = Number::Five,
            Number::Five => *self = Number::Six,
            Number::Six => *self = Number::Seven,
            Number::Seven => *self = Number::Eight,
            Number::Eight => *self = Number::Nine,
            Number::Nine => *self = Number::Zero,
        }
    }


    pub fn previous(&mut self)
    {
        match self
        {
            Number::Zero => *self = Number::Nine,
            Number::One => *self = Number::Zero,
            Number::Two => *self = Number::One,
            Number::Three => *self = Number::Two,
            Number::Four => *self = Number::Three,
            Number::Five => *self = Number::Four,
            Number::Six => *self = Number::Five,
            Number::Seven => *self = Number::Six,
            Number::Eight => *self = Number::Seven,
            Number::Nine => *self = Number::Eight,
        }
    }
}
