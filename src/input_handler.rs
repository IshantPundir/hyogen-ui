use smithay_client_toolkit::{
    delegate_pointer, delegate_seat, seat::{pointer::{PointerEventKind, PointerHandler}, Capability, SeatHandler, SeatState}, shell::WaylandSurface};
use wayland_client::{protocol::wl_pointer, QueueHandle};

use crate::layer::HyogenLayer;

impl SeatHandler for HyogenLayer {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _conn: &wayland_client::Connection, _qh: &QueueHandle<Self>, _seat: wayland_client::protocol::wl_seat::WlSeat) {}

    fn remove_seat(&mut self, _conn: &wayland_client::Connection, _qh: &QueueHandle<Self>, _seat: wayland_client::protocol::wl_seat::WlSeat) {}

    fn new_capability(
            &mut self,
            _conn: &wayland_client::Connection,
            qh: &QueueHandle<Self>,
            seat: wayland_client::protocol::wl_seat::WlSeat,
            capability: smithay_client_toolkit::seat::Capability,
        ) {
        if capability == Capability::Pointer && self.pointer.is_none() {
            tracing::info!("Set pointer capability");
            let pointer = self.seat_state.get_pointer(qh, &seat).expect("Failed to create pointer");
            self.pointer = Some(pointer);
        }

        // TODO: Capability::Touch
    }

    fn remove_capability(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _seat: wayland_client::protocol::wl_seat::WlSeat,
            capability: smithay_client_toolkit::seat::Capability,
        ) {
        if capability == Capability::Pointer && self.pointer.is_some() {
            tracing::info!("Unset pointer capability");
            self.pointer.take().unwrap().release();
        }
    }
}
delegate_seat!(HyogenLayer);

impl PointerHandler for HyogenLayer {
    fn pointer_frame(
            &mut self,
            _conn: &wayland_client::Connection,
            _qh: &QueueHandle<Self>,
            _pointer: &wl_pointer::WlPointer,
            events: &[smithay_client_toolkit::seat::pointer::PointerEvent],
        ) {
        use PointerEventKind::*;

        for event in events {
            // Ignore events for other surfaces
            if &event.surface != self.layer.wl_surface() {
                continue;
            }
            
            match event.kind {
                Enter { .. } => {}
                Leave { .. } => {}
                Motion { .. } => {}
                Press { .. } => {}
                Release { .. } => {}
                Axis { .. } => {}
            }
        }
    }
}
delegate_pointer!(HyogenLayer);

// TODO: impl Touch and gestures...