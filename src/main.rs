extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use image::io::Reader as ImageReader;
use std::path::PathBuf;
use winit::{
    event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder}, dpi::{PhysicalPosition, LogicalPosition},
};
fn main() {
    pretty_env_logger::init();
   let image = ImageReader::open("C:\\Users\\Administrator\\Desktop\\1.png").unwrap().decode().unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(true)
        .with_decorations(false) // This line makes the window borderless
        .with_skip_taskbar(true)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
        .build(&event_loop)
        .unwrap();
        
    window.drag_window();
    let mut cursor_pos:Option<PhysicalPosition<i32>> = None; // Initialize cursor position to None
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

            //重画window
            Event::RedrawRequested(windowId) => {
                let size = window.inner_size();
                let image = image.resize(size.width, size.height, image::imageops::FilterType::Triangle);
                     // Draw the image to the window here
                     let mut pixels = Vec::new();
                     for pixel in image.pixels() {
                         pixels.push(pixel[0]);
                         pixels.push(pixel[1]);
                         pixels.push(pixel[2]);
                     }
                     let frame = window
                         .prepare_frame(&pixels, size.width as usize * 3)
                         .unwrap();
                     frame
                         .set_position(0, 0)
                         .set_size(size.width as i32, size.height as i32)
                         .set_scale_factor(window.scale_factor())
                         .finish()
                         .unwrap();
                
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
                    DeviceEvent::MouseMotion { delta } => {
                        info!("mouse move: {:?}", delta);
                        if let Some( mut pos) = cursor_pos {
                            pos.x = pos.x + (delta.0 * (2.8 as f64)) as i32;
                            pos.y = pos.y + (delta.1* (2.8 as f64)) as i32;
                            cursor_pos = Some(pos);
                            // let new_pos = pos + delta;
                            window.set_outer_position(pos); // Move the window
                        }
                    }

                    DeviceEvent::MouseWheel { delta, .. } => {
                        info!("mouse wheel: {:?}", delta);
                    }
                    _ => {}
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
            //    let scale_factor = window.scale_factor();
            //    info!("sca : {:?}", scale_factor);
            //     let logic_pos:LogicalPosition<f64> =  position.to_logical(scale_factor);
            //     window.set_outer_position(logic_pos); // Move the window to the cursor position
            }


            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if button == winit::event::MouseButton::Left {
                    if state == winit::event::ElementState::Pressed {
                        cursor_pos = Some(window.outer_position().unwrap()); // Set cursor position to window position
                        info!("in")
                    } else {
                        cursor_pos = None; // Reset cursor position to None
                    }
                }
            }

      

            _ => {
                warn!("Unhandled event: {:?}", event)
            }
        }
    });
}
