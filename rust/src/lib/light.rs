pub trait LightSmBase {
    fn turn_off(&mut self);
    fn turn_blue(&mut self);
    fn turn_yellow(&mut self);
    fn get_color(&self) -> String;
}

// State machine variables. Can be used for inputs, outputs, user variables...
#[derive(Default)]
pub struct LightSm {
    state_id: StateId,
    // event_id: EventId,
    // vars
    pub count: isize,
    pub light_string: String,
}

impl LightSmBase for LightSm {
    fn turn_off(&mut self) {
        self.light_string = "off".into();
    }

    fn turn_blue(&mut self) {
        self.light_string = "blue".into();
    }

    fn turn_yellow(&mut self) {
        self.light_string = "yellow".into();
    }

    fn get_color(&self) -> String {
        return self.light_string.clone();
    }
}
