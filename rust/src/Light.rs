pub struct Light {
    light_color: String,
}

impl Default for Light {
    fn default() -> Self {
        Light {
            light_color: "off".into(),
        }
    }
}

impl Light {
    // const char* light_color = "?";

    pub fn off(&mut self) {
        self.light_color = "off".into();
    }

    pub fn blue(&mut self) {
        self.light_color = "blue".into();
    }

    pub fn yellow(&mut self) {
        self.light_color = "yellow".into();
    }

    pub fn get_color(&self) -> String {
        return self.light_color.clone();
    }
}
