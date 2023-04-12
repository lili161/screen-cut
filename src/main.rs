use std::path::PathBuf;
use image::io::Reader as ImageReader;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}, platform::windows::WindowBuilderExtWindows,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_decorations(false) // This line makes the window borderless
        .with_skip_taskbar(true)
        .with
        .build(&event_loop)
        .unwrap();

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
            },
            _ => {
                println!("Unhandled event: {:?}", event);
            },
        }
    });
}