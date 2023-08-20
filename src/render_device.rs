use serde::{Serialize, Deserialize};
use winit::event_loop::EventLoop;
use winit_input_helper::WinitInputHelper;

#[derive(Serialize, Deserialize)]
pub enum RenderType {
    Software(SoftwareRenderMethod),
    Hardware
}

#[derive(Serialize, Deserialize)]
pub enum SoftwareRenderMethod {
    Indirect,
    Buffer(usize)
}

pub trait RenderDevice {
    fn new() -> Self;
    fn render_type() -> RenderType;
    fn event_loop(self) -> EventLoop<()>;
    fn input(self) -> WinitInputHelper;
}