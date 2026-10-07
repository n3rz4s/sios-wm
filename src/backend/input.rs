use smithay::{
    backend::{
        libinput::{
            LibinputInputBackend,
            LibinputSessionInterface,
        },
        session::{
            libseat::LibSeatSession,
            Session,
        },
    },
    reexports::input::Libinput,
};

pub struct LibinputBackend {
    pub backend: LibinputInputBackend,
}

impl LibinputBackend {
    pub fn new(
        session: &LibSeatSession,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let seat_name = session.seat();

        let mut libinput = Libinput::new_with_udev(
            LibinputSessionInterface::from(session.clone()),
        );

        libinput
            .udev_assign_seat(&seat_name)
            .map_err(|_| "SIOS: Failed to assign libinput seat")?;

        let backend = LibinputInputBackend::new(libinput);

        Ok(
            Self { backend }
        )
    }
}