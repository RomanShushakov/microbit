pub enum Round
{
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}


impl Round
{
    pub fn convert_to_lights(&self) -> [[u8; 5]; 5]
    {
        match self
        {
            Round::Zero => 
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Round::One =>
            {
                [
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                    [0, 0, 1, 0, 0],
                ]
            },
            Round::Two =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Round::Three =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                ]
            },
            Round::Four =>
            {
                [
                    [0, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0],
                ]
            },
            Round::Five =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 0, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Round::Six =>
            {
                [
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Round::Seven =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 0, 1, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 0, 0, 0],
                    [0, 1, 0, 0, 0],
                ]
            },
            Round::Eight =>
            {
                [
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                    [0, 1, 0, 1, 0],
                    [0, 1, 1, 1, 0],
                ]
            },
            Round::Nine =>
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
            Round::Zero => *self = Round::One,
            Round::One => *self = Round::Two,
            Round::Two => *self = Round::Three,
            Round::Three => *self = Round::Four,
            Round::Four => *self = Round::Five,
            Round::Five => *self = Round::Six,
            Round::Six => *self = Round::Seven,
            Round::Seven => *self = Round::Eight,
            Round::Eight => *self = Round::Nine,
            Round::Nine => *self = Round::Zero,
        }
    }
}
