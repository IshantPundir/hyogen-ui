use smithay_client_toolkit::{
    compositor::CompositorHandler, delegate_compositor, delegate_layer, delegate_output, delegate_registry, delegate_shm, output::{OutputHandler, OutputState}, registry::{ProvidesRegistryState, RegistryState}, registry_handlers, seat::SeatState, shell::{wlr_layer::{LayerShellHandler, LayerSurface}, WaylandSurface}, shm::{slot::SlotPool, Shm, ShmHandler}
};
use wayland_client::{globals::GlobalList, protocol::{wl_pointer, wl_shm}, QueueHandle};

pub struct HyogenLayer {
    registry_state: RegistryState,
    pub seat_state: SeatState,
    output_state: OutputState,
    shm: Shm,
    pool: SlotPool,

    pub layer: LayerSurface,
    pub pointer: Option<wl_pointer::WlPointer>,

    width: u32,
    height: u32,
    shift: Option<u32>,
    first_configure: bool,
    exit: bool
}

impl OutputHandler for HyogenLayer {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn update_output(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }

    fn output_destroyed(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _output: wayland_client::protocol::wl_output::WlOutput,
        ) {
        
    }
}
delegate_output!(HyogenLayer);

impl LayerShellHandler for HyogenLayer {
    fn closed(&mut self, _conn: &wayland_client::Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        self.exit = true;
    }

    fn configure(
            &mut self,
            _conn: &wayland_client::Connection,
            qh: &QueueHandle<Self>,
            _layer: &LayerSurface,
            configure: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
            _serial: u32,
        ) {
        if configure.new_size.0 == 0 || configure.new_size.1 == 0 {
            self.width = 800;
            self.height = 480;
        } else {
            self.width = configure.new_size.0;
            self.height = configure.new_size.1;
        }

        // Initiate the first draw.
        if self.first_configure {
            self.first_configure = false;
            self.draw(qh);
        }
    }
}
delegate_layer!(HyogenLayer);

impl CompositorHandler for HyogenLayer {
    fn scale_factor_changed(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wayland_client::protocol::wl_surface::WlSurface,
            _new_factor: i32,
        ) { }

    fn transform_changed(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wayland_client::protocol::wl_surface::WlSurface,
            _new_transform: wayland_client::protocol::wl_output::Transform,
        ) { }

    fn surface_enter(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wayland_client::protocol::wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) { }

    fn surface_leave(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _surface: &wayland_client::protocol::wl_surface::WlSurface,
            _output: &wayland_client::protocol::wl_output::WlOutput,
        ) { }

    fn frame(
            &mut self,
            _conn: &wayland_client::Connection,
            qh: &QueueHandle<Self>,
            _surface: &wayland_client::protocol::wl_surface::WlSurface,
            _time: u32,
        ) {
        self.draw(qh);
    }
}
delegate_compositor!(HyogenLayer);

impl ShmHandler for HyogenLayer {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
delegate_shm!(HyogenLayer);

impl ProvidesRegistryState for HyogenLayer {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers![OutputState, SeatState];
}
delegate_registry!(HyogenLayer);

impl HyogenLayer {
    pub fn new(layer: LayerSurface, globals: &GlobalList, qh: &QueueHandle<HyogenLayer>, shm: Shm, pool: SlotPool) -> Self {
        HyogenLayer {
            registry_state: RegistryState::new(globals),
            seat_state: SeatState::new(globals, qh),
            output_state: OutputState::new(globals, qh),
            shm,
            pool,

            layer,
            pointer: None,
            
            width: 800,
            height: 480,
            shift: None,
            first_configure: true,
            exit: false
        }
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    pub fn draw(&mut self, qh: &QueueHandle<Self>) {
        let width = self.width;
        let height = self.height;
        let stride = self.width as i32 * 4;

        let (buffer, canvas) = self.pool
            .create_buffer(width as i32, height as i32, stride, wl_shm::Format::Argb8888)
            .expect("create buffer");

        // Draw to the window:
        {
            let shift = self.shift.unwrap_or(0);
            canvas.chunks_exact_mut(4).enumerate().for_each(|(index, chunk)| {
                let x = ((index + shift as usize) % width as usize) as u32;
                let y = (index / width as usize) as u32;

                let a = 0xFF;
                let r = u32::min(((width - x) * 0xFF) / width, ((height - y) * 0xFF) / height);
                let g = u32::min((x * 0xFF) / width, ((height - y) * 0xFF) / height);
                let b = u32::min(((width - x) * 0xFF) / width, (y * 0xFF) / height);
                let color = (a << 24) + (r << 16) + (g << 8) + b;

                let array: &mut [u8; 4] = chunk.try_into().unwrap();
                *array = color.to_le_bytes();
            });

            if let Some(shift) = &mut self.shift {
                *shift = (*shift + 1) % width;
            }
        }

        // Damage the entire window
        self.layer.wl_surface().damage_buffer(0, 0, width as i32, height as i32);

        // Request our next frame
        self.layer.wl_surface().frame(qh, self.layer.wl_surface().clone());

        // Attach and commit to present.
        buffer.attach_to(self.layer.wl_surface()).expect("buffer attach");
        self.layer.commit();

        // TODO save and reuse buffer when the window size is unchanged.  This is especially
        // useful if you do damage tracking, since you don't need to redraw the undamaged parts
        // of the canvas.
            
    }
}