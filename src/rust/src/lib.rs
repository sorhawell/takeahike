use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[derive(Debug)]
pub struct MyHiker {
    some_array: Vec<Vec<f64>>,
}

/// show case persist memory between calls to rust
/// @export
#[extendr]
impl MyHiker {
    fn new(l: List) -> Self {
        let some_array: Vec<_> = l
            .iter()
            .map(|(_, robj)| {
                robj.as_real_vector()
                    .expect("hey we said numeric vector!!!")
            })
            .collect();
        MyHiker { some_array }
    }

    fn print(&self) {
        rprintln!("MyHiker: {:?}", self);
    }
}

impl Drop for MyHiker {
    fn drop(&mut self) {
        self.print();
        rprintln!("goodbye my dear hiker !");
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl MyHiker;
    fn hello_world;
}
