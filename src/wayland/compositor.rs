use smithay::{
    delegate_compositor,
    delegate_layer_shell,
    backend::renderer::utils::{
        on_commit_buffer_handler,
        with_renderer_surface_state,
    },
    input::{
        Seat,
        SeatState,
        pointer::MotionEvent,
    },
    utils::{
        Logical,
        Point,
        Serial,
    },
    reexports::wayland_server::{
        backend::ClientData,
        protocol::wl_surface::WlSurface,
        protocol::wl_output::WlOutput,
        Client,
        DisplayHandle
    },
    reexports::drm::control::crtc::Handle as CrtcHandle,
    reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
    wayland::{
        shell::wlr_layer::{
            Layer,
            LayerSurface,
            WlrLayerShellHandler,
            WlrLayerShellState,
        },
        compositor::{
            CompositorClientState,
            CompositorHandler,
            CompositorState,
        },
        shell::xdg::{
            XdgShellState,
            decoration::XdgDecorationState,
        },
        shm::ShmState,
        viewporter::ViewporterState,
        output::OutputManagerState,
    },
};

use crate::desktop::{
    WindowId,
    Desktop,
    Cursor,
    render::CursorRenderer,
};


pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}


pub struct State {
    pub display_handle: DisplayHandle,

    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub xdg_decoration_state: XdgDecorationState,
    pub seat_state: SeatState<Self>,
    pub seat: Seat<Self>,
    pub shm_state: ShmState,
    pub layer_shell_state: WlrLayerShellState,
    pub viewporter_state: ViewporterState,
    pub output_manager_state: OutputManagerState,

    pub desktop: Desktop,

    pub cursor: Cursor,
    pub cursor_renderer: CursorRenderer,
    pub drm_vblank: Vec<CrtcHandle>,

    pub frame_in_flight: bool,
    pub redraw_requested: bool,
}

impl CompositorHandler for State {
    
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a Client,
    ) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .unwrap()
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {

        on_commit_buffer_handler::<Self>(surface);

        let surface_size =
            with_renderer_surface_state(
                surface,
                |state| state.surface_size(),
            );

        if let Some(Some(size)) = surface_size
        {

            for window in &mut self.desktop.windows {
                if *window.surface().wl_surface() == *surface {
                    window.set_size(size);

                    eprintln!(
                        "SIOS: Window {:?} geometry updated: position={:?}, size={:?}",
                        window.id(),
                        window.position(),
                        window.size(),
                    );

                    break;
                }
            }
        } else {
            eprintln!(
                "SIOS: Surface {:?}: No renderer view",
                surface,
            );
        }

        self.redraw_requested = true;
    }
}

impl WlrLayerShellHandler for State {
    fn shell_state(
        &mut self,
    ) -> &mut WlrLayerShellState {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        _output: Option<WlOutput>,
        layer: Layer,
        namespace: String,
    ) {
        eprintln!(
            "SIOS: New layer surface: layer={:?}, namespace={}",
            layer,
            namespace,
        );

        surface.send_configure();

        self.desktop.add_layer(surface);

        self.redraw_requested = true;
    }
}

impl State {
    pub fn sync_keyboard_focus(
        &mut self,
        id: Option<WindowId>,
        serial: Serial,
    ) {
        let focus = id.and_then(|id| {
            self.desktop
                .window(id)
                .map(|window| {
                    window
                        .surface()
                        .wl_surface()
                        .clone()
                })
        });

        let Some(keyboard) = self.seat.get_keyboard() else {
            return;
        };

        if keyboard.current_focus() == focus {
            return;
        }

        keyboard.set_focus(
            self,
            focus,
            serial,
        );
    }

    pub fn sync_xdg_focus(
        &mut self
    ) {
        for window in &self.desktop.windows {
            let activated = window.is_focused();

            window.surface().with_pending_state(
                |state|
                {
                    if activated {
                        state.states.set(
                            xdg_toplevel::State::Activated,
                        );
                    } else {
                        state.states.unset(
                            xdg_toplevel::State::Activated,
                        );
                    }
                },
            );

            let _ = window
                .surface()
                .send_pending_configure();
        }
    }

    pub fn handle_pointer_motion(
        &mut self,
        event: MotionEvent,
    ) {
        let new_focus =
            self.desktop.window_at(event.location);

        let old_focus =
            self.desktop.focused_window();

        if old_focus != new_focus {
            match new_focus {
                Some(id) => {
                    if self.desktop.focus_window(id) {
                        self.sync_xdg_focus();

                        self.sync_keyboard_focus(
                            Some(id),
                            event.serial,
                        );
                    }
                }

                None => {
                    self.desktop.unfocus();

                    self.sync_xdg_focus();

                    self.sync_keyboard_focus(
                        None,
                        event.serial,
                    );
                }
            }
        }

        let pointer_focus =
            new_focus.and_then(|id| {
                self.desktop
                    .window(id)
                    .map(|window| {
                        (
                            window
                                .surface()
                                .wl_surface()
                                .clone(),

                            Point::from((
                                window.position().x as f64,
                                window.position().y as f64,
                            )),
                        )
                    })
            });

        let Some(pointer) =
            self.seat.get_pointer()
        else {
            return;
        };

        pointer.motion(
            self,
            pointer_focus,
            &event,
        );

        pointer.frame(self);
    }
}

delegate_compositor!(State);
delegate_layer_shell!(State);

