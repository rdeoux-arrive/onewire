#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

mod capture;
mod graphics;
mod sample;
mod settings;
mod view;
mod voltage;

use crate::capture::{Capture, Direction, Edge, SlotKind};
use crate::graphics::gruvbox::dark as gruvbox;
use crate::graphics::{Color, Draw, Line, Point, Rect, Text};
use crate::sample::Sample;
use crate::settings::Settings;
use crate::view::{View, HEIGHT, WIDTH};
use crate::voltage::Voltage;
use indicatif::ProgressBar;
use pixels::{Pixels, SurfaceTexture};
use std::env::args;
use std::fs::File;
use winit::dpi::PhysicalSize;
use winit::event::{
    ElementState, Event, ModifiersState, MouseButton, MouseScrollDelta, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let filepath = args().nth(1).expect("Missing file path");
    let file = File::open(&filepath).unwrap();
    let bar = ProgressBar::new(file.metadata().unwrap().len());
    let capture = Capture::from_reader(bar.wrap_read(file)).unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false)
        .with_title(filepath)
        .build(&event_loop)
        .unwrap();
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    let mut settings = Settings::default();
    let mut modifiers = ModifiersState::default();
    let mut cursor_position: Option<u32> = None;
    let mut drag_start = None;

    event_loop.run(move |event, _window, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ModifiersChanged(mods) => modifiers = mods,
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, delta),
                    ..
                } => {
                    if delta > 0.0 {
                        if modifiers.shift() {
                            settings.dec_voltage();
                        } else {
                            settings.dec_period();
                        }
                    } else if modifiers.shift() {
                        settings.inc_voltage();
                    } else {
                        settings.inc_period();
                    }
                    window.request_redraw();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x as u32;
                    if let Some(x) = cursor_position.replace(x) {
                        if let Some((drag_start, offset)) = drag_start {
                            let view = View::new(&settings, &capture);
                            let x0 = view.x_to_timestamp(drag_start) as i32;
                            let x1 = view.x_to_timestamp(x) as i32;
                            settings.offset = offset + x0 - x1;
                        }
                    }
                    window.request_redraw();
                }
                WindowEvent::MouseInput {
                    state,
                    button: MouseButton::Left,
                    ..
                } => {
                    if let Some(x) = cursor_position {
                        drag_start = if state == ElementState::Pressed {
                            Some((x, settings.offset))
                        } else {
                            None
                        }
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(_window_id) => {
                let frame = pixels.get_frame();

                Rect::new()
                    .with_width(WIDTH)
                    .with_height(HEIGHT)
                    .with_color(gruvbox::BG1)
                    .draw(frame, WIDTH);

                let view = View::new(&settings, &capture);
                view.draw(frame, WIDTH);

                // Cursor
                if let Some(x) = cursor_position {
                    let text = format!("X = {} us", view.x_to_timestamp(x));
                    Text::new(&text).with_color(gruvbox::FG0).draw(frame, WIDTH);
                }

                pixels.render().ok();
            }
            _ => {}
        }
    })
}
