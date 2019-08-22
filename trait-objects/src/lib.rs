pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Where the magic happens - components must be a Vec of items that implement the trait "Draw"
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "{} by {} button with label {}",
            self.width, self.height, self.label
        );
    }
}
