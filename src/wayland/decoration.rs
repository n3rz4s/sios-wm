use smithay::{
    delegate_xdg_decoration,
    reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::Mode,
    wayland::shell::xdg::{
        decoration::XdgDecorationHandler,
        ToplevelSurface,
    },
};

use super::State;


impl XdgDecorationHandler for State {
    fn new_decoration(&mut self, toplevel: ToplevelSurface) {
        eprintln!(
            "SIOS: New XDG Decoration",
        );

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });

        toplevel.send_configure();

        eprintln!(
            "SIOS: XDG Decoration mode = ServerSide",
        );
    }

    fn request_mode(
        &mut self,
        toplevel: ToplevelSurface,
        mode: Mode,
    ) {
        eprintln!(
            "SIOS: Client requested XDG decoration mode: {:?}",
            mode,
        );

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });

        toplevel.send_configure();

        eprintln!(
            "SIOS: XDG Decoration forced to ServerSide",
        );
    }

    fn unset_mode(&mut self, toplevel: ToplevelSurface) {
        eprintln!(
            "SIOS: Client unset XDG decoration mode",
        );

        toplevel.with_pending_state(|state| {
            state.decoration_mode = Some(Mode::ServerSide);
        });

        toplevel.send_configure();

        eprintln!(
            "SIOS: XDG Decoration remains ServerSide",
        );
    }
}

delegate_xdg_decoration!(State);