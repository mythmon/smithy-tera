extern crate smithy;
extern crate smithy_tera;

use smithy::Smithy;
use smithy_tera::SmithyTera;

fn main() {
    match Smithy::builder("input", "output")
        .add_plugin(SmithyTera)
        .build() {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err),
        };
}
