#![recursion_limit = "1024"]

mod app;
mod components;
mod constants;
mod modules;
mod router;
mod utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;

fn main() {
    set_panic_hook();

    yew::start_app::<app::App>();
}
