use screen::{Button, Draw, Screen};

struct SelectBox {
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Checkbox with options:");
        for option in self.options.iter() {
            println!("[] {}", option);
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 20,
                label: String::from("Submit"),
            }),
        ],
    };

    screen.run();
}
