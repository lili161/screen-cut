extern crate pretty_env_logger;
#[macro_use] extern crate log;
use image::io::Reader as ImageReader;
use std::path::PathBuf;
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder},
};
fn main() {
    pretty_env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_decorations(false) // This line makes the window borderless
        .with_skip_taskbar(true)
        .build(&event_loop)
        .unwrap();
    info!("begin");
    // let image_path = PathBuf::from("path/to/image.png");
    // let image = ImageReader::open(image_path).unwrap().decode().unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let size = window.inner_size();
                // let image = image.resize(size.width, size.height, image::imageops::FilterType::Triangle);
                // Draw the image to the window here
            }
            Event::DeviceEvent {
                device_id: _,
                event,
            } => {
                // Handle keyboard input here
                match event {
                    DeviceEvent::Key(keyboard_input) => match keyboard_input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => {
                            info!("END!");
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            _ => {
                warn!("Unhandled event: {:?}", event)
            }
        }
    });
}
