use smithay::{
    delegate_output,
    wayland::output::OutputHandler,
};

use super::State;

impl OutputHandler for State {
}

delegate_output!(State);