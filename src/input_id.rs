static mut NEXT_INPUT_INDEX: u8 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct InputID {
    pub id: u8,
}

impl InputID {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Default for InputID {
    fn default() -> Self {
        unsafe {
            let component = Self::new(NEXT_INPUT_INDEX);
            NEXT_INPUT_INDEX += 1;
            component
        }
    }
}
