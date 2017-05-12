#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate tunapanel;

use tunapanel::ServerConfig;

tunapanel! {
    #[derive(Debug)]
    struct Panel {
        #[label = "A float"]
        x: f32 = 0.0,

        #[label = "A string"]
        y: String = String::new(),
    }
}

fn main() {
    let server_config: ServerConfig = Default::default();

    println!("Listening on {}", &server_config.listen_on);
    tunapanel::serve::<Panel, _>(server_config, |p| {
        println!("Panel update: {:?}", p);
    }).unwrap();
}
