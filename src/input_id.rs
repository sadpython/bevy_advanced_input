static mut NEXT_INPUT_INDEX: u8 = 0;

#[derive(Debug, PartialEq, Eq)]
pub struct InputId {
    pub id: u8,
}

impl InputId {
    pub fn new(id: u8) -> Self {
        Self { id }
    }
}

impl Default for InputId {
    fn default() -> Self {
        unsafe {
            let component = Self::new(NEXT_INPUT_INDEX);
            NEXT_INPUT_INDEX += 1;
            component
        }
    }
}
