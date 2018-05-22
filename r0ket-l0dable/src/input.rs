use table::table;

pub struct InputState(pub u8);

macro_rules! def_getter {
    ($name: ident, $mask: expr) => (
        pub fn $name(&self) -> bool {
            self.0 & $mask != 0
        }
    )
}


impl InputState {
    def_getter!(up, 1);
    def_getter!(down, 2);
    def_getter!(left, 4);
    def_getter!(right, 8);
    def_getter!(enter, 0x10);
}

pub fn get_input() -> InputState {
    InputState((table().getInput)())
}

pub fn get_input_raw() -> InputState {
    InputState((table().getInputRaw)())
}
