use smithay::backend::session::{
    libseat::{
        LibSeatSession,
        LibSeatSessionNotifier,
    },
    Session,
};

pub struct SessionBackend {
    pub session: LibSeatSession,
    pub notifier: Option<LibSeatSessionNotifier>,
}

impl SessionBackend {
    pub fn new() -> Result<
    Self,
    Box<dyn std::error::Error>,
    > {
        let (session, notifier) = LibSeatSession::new()?;

        Ok(
            Self {
                session,
                notifier: Some(notifier),
            }
        )
    }

    pub fn is_active(&self) -> bool {
        self.session.is_active()
    }

    pub fn seat(&self) -> String {
        self.session.seat()
    }

    pub fn take_notifier(&mut self) -> Option<LibSeatSessionNotifier> {
        self.notifier.take()
    }
}