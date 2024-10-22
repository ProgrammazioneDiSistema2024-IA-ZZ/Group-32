use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn start_mouse_tracking() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mouse Input")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    if state == ElementState::Pressed && button == MouseButton::Left {
                        println!("Mouse click detected");
                    }
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let (x, y) = delta;
                    println!("Mouse moved by: ({}, {})", x, y);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
