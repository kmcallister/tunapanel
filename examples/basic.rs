#[macro_use] extern crate serde_derive;
#[macro_use] extern crate tunapanel;

use tunapanel::widget::Button;

tunapanel! {
    #[title = "My awesome panel"]
    #[derive(Debug)]
    struct Panel {
        #[label = "A float"]
        x: f32 = 0.0,

        #[label = "A string"]
        y: String = String::new(),

        #[label = "A bool"]
        b: bool = true,

        #[label = "A button"]
        but1: Button = Button::new(),

        #[label = "Another button"]
        but2: Button = Button::new(),
    }
}

fn main() {
    tunapanel::serve(|p: Panel| {
        println!("Panel update: {:?}", p);
    }).unwrap();
}
