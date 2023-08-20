use pixels::{Pixels, SurfaceTexture};
use serde::{Deserialize, Serialize};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::memory_mapping::{MappingTable, MemoryMapping};
use crate::NAME;
use crate::render_device::{RenderDevice, RenderType, SoftwareRenderMethod};

const BUFFER_SIZE: usize = 0x40000;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

#[derive(Serialize, Deserialize)]
pub struct VGABuf {
    event_loop: EventLoop<()>,
    input: WinitInputHelper,
    mapping_table: MappingTable,
    pixels: Pixels
}
impl RenderDevice for VGABuf {
    fn new() -> Self {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT + 10);
            WindowBuilder::new()
                .with_title(NAME)
                .with_inner_size(size)
                .with_resizable(false)
                .build(&event_loop)?
        };

        let mut pixels: Pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT + 10, surface_texture)?
        };

        Self {
            event_loop,
            input,
            pixels
        }
    }

    fn render_type() -> RenderType {
        RenderType::Software(SoftwareRenderMethod::Buffer(640 * 480))
    }

    fn event_loop(self) -> EventLoop<()> {
        self.event_loop
    }

    fn input(self) -> WinitInputHelper {
        self.input
    }
}
impl MemoryMapping for VGABuf {
    fn initialize(start: usize) -> usize {
        todo!()
    }

    fn table() -> MappingTable {
        todo!()
    }
}