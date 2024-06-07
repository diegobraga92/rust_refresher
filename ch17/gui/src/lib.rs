pub struct Button {
    pub width: u32,
    pub heigh: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}