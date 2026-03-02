/// Box-drawing character sets.
#[derive(Debug)]
pub struct BoxChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub arrow_down: char,
    pub arrow_up: char,
    pub arrow_right: char,
    pub arrow_left: char,
}

impl BoxChars {
    pub fn unicode() -> Self {
        Self {
            top_left: '\u{250C}',     // ┌
            top_right: '\u{2510}',    // ┐
            bottom_left: '\u{2514}',  // └
            bottom_right: '\u{2518}', // ┘
            horizontal: '\u{2500}',   // ─
            vertical: '\u{2502}',     // │
            arrow_down: 'v',
            arrow_up: '^',
            arrow_right: '>',
            arrow_left: '<',
        }
    }

    pub fn ascii() -> Self {
        Self {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            arrow_down: 'v',
            arrow_up: '^',
            arrow_right: '>',
            arrow_left: '<',
        }
    }
}
