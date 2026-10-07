use smithay::{
    delegate_xdg_shell,
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::protocol::wl_seat,
    },
    utils::{
        Serial,
        SERIAL_COUNTER,
    },
    wayland::shell::xdg::{
        PopupSurface,
        PositionerState,
        ToplevelSurface,
        XdgShellHandler,
        XdgShellState,
    },
};

use super::State;

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        eprintln!("SIOS: New TopLevel");

        surface.with_pending_state(|state| {
            state.size = Some((1050, 650).into());
        });

        surface.send_configure();

        let id = 
            self.desktop.add_window(surface);

        if self.desktop.focused_window()
            == Some(id)
        {
            let serial = 
                SERIAL_COUNTER.next_serial();

            self.sync_xdg_focus();

            self.sync_keyboard_focus(
                Some(id),
                serial,
            );
        }

        eprintln!(
            "SIOS: Window count = {}",
            self.desktop.windows.len(),
        );
    }

    fn toplevel_destroyed(
        &mut self,
        surface: ToplevelSurface,
    ) {
        let id =
            self.desktop.windows
                .iter()
                .find(|window| {
                    *window.surface().wl_surface()
                        == *surface.wl_surface()
                })
                .map(|window| window.id());

        let Some(id) = id else {
            return;
        };

        let was_focused =
            self.desktop.focused_window()
                == Some(id);

        self.desktop.remove_window(id);

        if !was_focused {
            return;
        }

        let next =
            self.desktop.window_at(
                self.cursor.position()
            );

        match next {
            Some(id) => {
                let _ =
                    self.desktop
                        .focus_window(id);

                self.sync_xdg_focus();

                self.sync_keyboard_focus(
                    Some(id),
                    SERIAL_COUNTER.next_serial(),

                );
            }

            None => {
                self.desktop.unfocus();

                self.sync_xdg_focus();

                self.sync_keyboard_focus(
                    None,
                    SERIAL_COUNTER.next_serial(),
                );
            }
        }
    }

    fn new_popup(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState
    ) {
        eprintln!("SIOS: New Popup");
    }

    fn grab(
        &mut self,
        _surface: PopupSurface,
        _seat: wl_seat::WlSeat,
        _serial: Serial,
    ) {
        eprintln!("SIOS: Popup Grab");
    }

    fn reposition_request(
        &mut self,
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
        eprintln!("SIOS: Popup Reposition");
    }
}

delegate_xdg_shell!(State);