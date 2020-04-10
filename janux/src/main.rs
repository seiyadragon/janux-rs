extern crate minifb;

pub use crate::core::*;
pub mod core;

struct Game
{

}

fn main()
{
    let app = Application::new(ApplicationConfig::default());
    app.run();
}
