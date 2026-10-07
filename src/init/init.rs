use std::error::Error;

use calloop::LoopHandle;

use smithay::{
    input::{
        keyboard::FilterResult,
        pointer::MotionEvent,
    },
    reexports::input::event::{
        keyboard::KeyboardEventTrait,
        pointer::PointerEventTrait,
    },
    backend::{
        drm::{
            DrmDeviceNotifier,
            DrmEvent,
        },
        session::Event as SessionEvent,
        session::libseat::LibSeatSessionNotifier,
        udev::UdevBackend,
        input::{
            AbsolutePositionEvent,
            InputEvent,
            ButtonState,
            KeyboardKeyEvent,
            PointerAxisEvent,
            PointerButtonEvent,
            PointerMotionEvent,
            Axis,
        },
    },
    utils::{
        Point,
        SERIAL_COUNTER,
    },
};

use crate::{
    backend::{
        session::SessionBackend,
        input::LibinputBackend,
        drm::DrmBackend,
        renderer::RendererBackend,
        drm_compositor::DrmCompositor,
    },
    desktop::{
        Desktop,
        Cursor,
    },
    wayland::State,
};

use sios_wm_api::MonitorConfig;

pub struct SIOSBackend {
    pub session: SessionBackend,
    pub drm: DrmBackend,
    pub compositor: DrmCompositor,
}

impl SIOSBackend {

    fn init_session() -> Result<SessionBackend, Box<dyn Error>> {
        let session = SessionBackend::new()?;

        eprintln!(
            "SIOS: Session initialized: seat={}, active={}",
            session.seat(),
            session.is_active(),
        );

        Ok(session)
    }

    fn init_udev(
        session: &SessionBackend
    ) -> Result<UdevBackend, Box<dyn Error>> {
        let udev = UdevBackend::new(session.seat())?;

        eprintln!("SIOS: Udev initialized.");

        Ok(udev)
    }

    fn init_input(
        session: &SessionBackend
    ) -> Result<LibinputBackend, Box<dyn Error>> {
        let input = LibinputBackend::new(
            &session.session
        )?;

        eprintln!("SIOS: Libinput initialized.");

        Ok(input)
    }

    fn init_drm(
        session: &mut SessionBackend,
    ) -> Result<(DrmBackend, DrmDeviceNotifier), Box<dyn Error>> {
        let (drm, notifier) = 
            DrmBackend::new(&mut session.session)?;

        Ok((
            drm,
            notifier,
        ))
    }

    fn init_renderer(
        drm: &DrmBackend
    ) -> Result<RendererBackend, Box<dyn Error>> {
        let renderer = 
            RendererBackend::new(drm.fd.clone())?;

        eprintln!("SIOS: Renderer initialized.");

        Ok(renderer)
    }

    fn register_udev(
        handle: &LoopHandle<'static, State>,
        udev: UdevBackend,
    ) -> Result<(), Box<dyn Error>> {
        handle.insert_source(
            udev,
            |event, _, _state| {
                eprintln!(
                    "SIOS: Udev event: {:?}",
                    event,
                );
            },
        )?;

        eprintln!("SIOS: Udev registered.");

        Ok(())
    }

    fn register_input(
        handle: &LoopHandle<'static, State>,
        input: LibinputBackend,
    ) -> Result<(), Box<dyn Error>> {
        handle.insert_source(
            input.backend,
            |event, _, state| {
                match event {
                    InputEvent::Keyboard { event } => {
                        eprintln!(
                            "SIOS: Keyboard: key={:?}, state={:?}",
                            event.key_code(),
                            event.state(),
                        );

                        if let Some(keyboard) =
                            state.seat.get_keyboard()
                        {
                            let serial = 
                                SERIAL_COUNTER.next_serial();

                            let time =
                                event.time() as u32;

                            keyboard.input(
                                state,
                                event.key_code(),
                                event.state(),
                                serial,
                                time,
                                |_, _, _| {
                                    FilterResult::<()>::Forward
                                }
                            );
                        }
                    }

                    InputEvent::PointerMotion { event } => {
                        let delta = event.delta();

                        state.cursor.motion((
                            delta.x,
                            delta.y,
                        ));

                        let position = state.cursor.position();

                        let motion = MotionEvent {
                            location: position,
                            serial: SERIAL_COUNTER.next_serial(),
                            time: event.time() as u32,
                        };

                        state.handle_pointer_motion(
                            motion,
                        );

                        state.redraw_requested = true;
                    }

                    InputEvent::PointerMotionAbsolute { event } => {
                        let raw_position = 
                            event.position();

                        state.cursor.set_position(
                            Point::from((
                                raw_position.x,
                                raw_position.y,
                            )),
                        );

                        let position = state.cursor.position();

                        let motion = MotionEvent {
                            location: position,
                            serial: SERIAL_COUNTER.next_serial(),
                            time: event.time() as u32,
                        };

                        state.handle_pointer_motion(
                            motion,
                        );

                        state.redraw_requested = true;
                    }

                    InputEvent::PointerButton { event } => {
                        eprintln!(
                            "SIOS: Mouse Button: button={}, state={:?}",
                            event.button_code(),
                            event.state(),
                        );
                    }

                    InputEvent::PointerAxis { event } => {
                        eprintln!(
                            "SIOS: Mouse wheel: {:?}",
                            event.amount(Axis::Vertical),
                        );
                    }

                    InputEvent::DeviceAdded { device } => {
                        eprintln!(
                            "SIOS: Input device added: {}",
                            device.name(),
                        );
                    }

                    InputEvent::DeviceRemoved { device } => {
                        eprintln!(
                            "SIOS: Input device removed: {}",
                            device.name()
                        )
                    }
                    _ => {}
                }
            },
        )?;

        eprintln!("SIOS: Input initialized.");

        Ok(())
    }

    fn register_session(
        handle: &LoopHandle<'static, State>,
        notifier: LibSeatSessionNotifier,
    ) -> Result<(), Box<dyn Error>> {
        handle.insert_source(
            notifier,
            |event, _, _state| {
                match event {
                    SessionEvent::ActivateSession => {
                        eprintln!(
                            "SIOS: Session activated.",
                        );
                    }

                    SessionEvent::PauseSession => {
                        eprintln!(
                            "SIOS: Session paused.",
                        );
                    }
                }
            },
        )?;

        eprintln!("SIOS: Session notifier required.");

        Ok(())
    }

    fn register_drm(
        handle: &LoopHandle<'static, State>,
        notifier: DrmDeviceNotifier,
    ) -> Result<(), Box<dyn Error>> {
        handle.insert_source(
            notifier,
            |event, _, state| {
                match event {
                    DrmEvent::VBlank(crtc) => {
                        state.drm_vblank.push(crtc);
                    }

                    DrmEvent::Error(error) => {
                        eprintln!(
                            "SIOS: DRM Error: {:?}",
                            error,
                        );
                    }
                }
            },

        )?;

        eprintln!(
            "SIOS: DRM notifier required.",
        );

        Ok(())
    }

    pub fn init_all(
        handle: &LoopHandle<'static, State>,
        monitor: &MonitorConfig,
    ) -> Result<(Self, Desktop), Box<dyn Error>> {
        let mut session = Self::init_session()?;

        let udev =
            Self::init_udev(&session)?;

        Self::register_udev(
            handle,
            udev,
        )?;

        let input = 
            Self::init_input(&session)?;

        Self::register_input(
            handle,
            input,
        )?;

        let (mut drm, drm_notifier) = 
            Self::init_drm(
                &mut session,
            )?;

        let outputs = drm.find_outputs(monitor)?;

        let surfaces = drm.create_surfaces(&outputs)?;

        let mut desktop = Desktop::new();

        desktop.add_monitors(&outputs);

        Self::register_drm(
            handle,
            drm_notifier,
        )?;

        let notifier = session.take_notifier()
            .expect("SIOS: Session notifier already taken.");

        Self::register_session(
            handle,
            notifier,
        )?;

        let renderer = 
            Self::init_renderer(&drm)?;

        let compositor =
            DrmCompositor::new(
                &drm,
                surfaces,
                &outputs,
                renderer,
            )?;

        eprintln!(
            "SIOS: Backend initialized",
        );

        Ok((
            Self {
                session,
                drm,
                compositor,
            },
            desktop,
        ))
    }
}