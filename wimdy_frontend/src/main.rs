use sycamore::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    sycamore::render(|| {
        template! {
            p { "crab swin" }
        }
    });
}
