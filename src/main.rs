mod render_device;
mod memory_mapping;
mod peripherals;

use std::error::Error;
use pixels::{Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::peripherals::vga_buf::VGABuf;
use crate::render_device::RenderDevice;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const NAME: &str = "ETCa_emu";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let vga = VGABuf::new();
    let event_loop = vga.event_loop();
    let mut input = vga.input();

    event_loop.run(move |event, _, control_flow| {
        pixels.clear_color(Color::BLACK);
        let frame = pixels.frame_mut();
        frame[1228799] = 0xFF;
        frame[1228798] = 0x00;
        frame[1228797] = 0xFF;
        frame[1228796] = 0x00;
        pixels.render().expect("ERROR RENDERING");

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}
