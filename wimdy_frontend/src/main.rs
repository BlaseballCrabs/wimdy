use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        template! {
            p { "crab swin" }
        }
    });
}
