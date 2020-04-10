#![feature(new_uninit)]

use minifb::*;
use crate::core::*;

pub struct ApplicationConfig
{
    pub name: &'static str,
    pub width: usize,
    pub height: usize,
    pub fullscreen: bool,
    pub target_tps: f64,
}

impl ApplicationConfig // Static
{
    pub fn default() -> Self
    {
        Self
        {
            name: "Janux",
            width: 1280,
            height: 720,
            fullscreen: false,
            target_tps: 60.0,
        }
    }
}

pub struct Application
{
    pub config: ApplicationConfig,
    pub window: Window,
    pub clock: Clock,
}

impl Application // Static
{
    pub fn new(config: ApplicationConfig) -> Self
    {
        let mut options = WindowOptions::default();
        options.resize = true;

        if config.fullscreen
        {
            options.borderless = true;
            options.resize = false;
        }

        let window = Window::new(config.name, config.width, config.height, options).unwrap();

        let clock = Clock::new();

        Self
        {
            config: config,
            window: window,
            clock: clock,
        }
    }
}

impl Application // Member
{
    pub fn run(mut self)
    {
        let mut last = self.clock.now();
        let mut lag = 0.0;

        while self.window.is_open()
        {
            let now = self.clock.now();
            let elapsed = now.nanos() - last.nanos();
            last = now;
            lag += elapsed as f64;

            while lag >= self.config.target_tps
            {
                lag -= self.config.target_tps;
            }

            self.window.update();
        }
    }
}