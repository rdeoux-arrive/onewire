#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

mod capture;
mod graphics;
mod sample;
mod settings;
mod view;
mod voltage;

use std::{fs::File, io, path::PathBuf, process::exit, sync::Arc};

use clap::Parser;
use indicatif::ProgressBar;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{ElementState, Modifiers, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::ModifiersKeyState,
    window::{Window, WindowId},
};

use crate::{
    capture::{Capture, Direction, Edge, SlotKind},
    graphics::{Color, Draw, Line, Point, Rect, Text, gruvbox::dark as gruvbox},
    sample::Sample,
    settings::Settings,
    view::{HEIGHT, View, WIDTH},
    voltage::Voltage,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Winit event loop: {0}")]
    EventLoop(#[from] EventLoopError),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid capture: {0}")]
    Capture(#[from] capture::Error),
}

impl Error {
    #[must_use]
    pub fn raw_os_error(&self) -> Option<i32> {
        match self {
            Self::EventLoop(EventLoopError::ExitFailure(code)) => Some(*code),
            Self::Io(error) => error.raw_os_error(),
            Self::Capture(capture::Error::Csv(error)) => {
                if let csv::ErrorKind::Io(error) = error.kind() {
                    error.raw_os_error()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Path to the CSV data
    pub filepath: PathBuf,
}

struct App {
    filepath: PathBuf,
    capture: Capture,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    settings: Settings,
    modifiers: Modifiers,
    cursor_position: Option<u32>,
    drag_start: Option<(u32, i32)>,
}

impl App {
    pub fn new() -> Result<Self, Error> {
        let Args { filepath } = Args::parse();

        // Open the CSV file.
        let file = File::open(&filepath)
            .inspect_err(|err| error!("Unable to open {}: {err}", filepath.display()))?;

        // Create a progress bar.
        let metadata = file
            .metadata()
            .inspect_err(|err| error!("Unable to get file metadata: {err}"))?;
        let bar = ProgressBar::new(metadata.len());

        // Load the data.
        let capture = Capture::from_reader(bar.wrap_read(file))
            .inspect_err(|err| error!("Invalid content: {err}"))?;

        Ok(Self {
            filepath,
            capture,
            window: None,
            pixels: None,
            settings: Settings::default(),
            modifiers: Modifiers::default(),
            cursor_position: None,
            drag_start: None,
        })
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
            .with_resizable(false)
            .with_title(self.filepath.to_string_lossy());
        let window = event_loop
            .create_window(window_attributes)
            .inspect_err(|err| error!("Unable to create the window: {err}"));

        if let Ok(window) = window {
            let window = Arc::new(window);
            let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window.clone());
            self.pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)
                .inspect_err(|err| error!("Unable to create a pixels buffer: {err}"))
                .ok();
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ModifiersChanged(modifiers) => self.modifiers = modifiers,
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as u32;
                if let Some(x) = self.cursor_position.replace(x)
                    && let Some((drag_start, offset)) = self.drag_start
                {
                    let view = View::new(&self.settings, &self.capture);
                    let x0 = view.x_to_timestamp(drag_start);
                    let x1 = view.x_to_timestamp(x);
                    self.settings.offset = offset + x0 - x1;
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let shift = self.modifiers.lshift_state() == ModifiersKeyState::Pressed
                    || self.modifiers.rshift_state() == ModifiersKeyState::Pressed;
                let delta_inc = match delta {
                    MouseScrollDelta::LineDelta(hdelta, vdelta) => hdelta > 0.0 || vdelta > 0.0,
                    MouseScrollDelta::PixelDelta(physical_position) => {
                        physical_position.x >= 0.0 || physical_position.y >= 0.0
                    }
                };
                if delta_inc {
                    if shift {
                        self.settings.dec_voltage();
                    } else {
                        self.settings.dec_period();
                    }
                } else if shift {
                    self.settings.inc_voltage();
                } else {
                    self.settings.inc_period();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::MouseInput { state, .. } => {
                if let Some(x) = self.cursor_position {
                    self.drag_start = if state == ElementState::Pressed {
                        Some((x, self.settings.offset))
                    } else {
                        None
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    Rect::new()
                        .with_width(WIDTH)
                        .with_height(HEIGHT)
                        .with_color(gruvbox::BG1)
                        .draw(frame, WIDTH);

                    let view = View::new(&self.settings, &self.capture);
                    view.draw(frame, WIDTH);

                    // Cursor
                    if let Some(x) = self.cursor_position {
                        let text = format!("X = {} us", view.x_to_timestamp(x));
                        Text::new(&text).with_color(gruvbox::FG0).draw(frame, WIDTH);
                    }

                    if let Err(err) = pixels.render() {
                        error!("Unable to render: {err}");
                    }
                }
            }
            _ => {}
        }
    }
}

fn onewire_main() -> Result<(), Error> {
    let mut app = App::new()?;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app).map_err(Error::from)
}

fn main() {
    if let Err(err) = onewire_main() {
        eprintln!("{err}");
        exit(err.raw_os_error().unwrap_or(1))
    }
}
