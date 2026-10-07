use std::{
    sync::Arc,
    thread,
    time::{
        Duration,
        Instant,
    },
};

use calloop::{
    EventLoop,
    LoopHandle,
};

use smithay::{
    input::SeatState,
    utils::Point,
    reexports::{
        wayland_server::{
            Display,
            protocol::{
                wl_shm,
                wl_surface::WlSurface,
            },
        },
    },
    wayland::{
        shell::wlr_layer::{
            WlrLayerShellState,
        },
        compositor::{
            CompositorClientState,
            CompositorState,
            with_surface_tree_downward,
            SurfaceAttributes,
            TraversalAction,
        },
        socket::ListeningSocketSource,
        shell::xdg::{
            XdgShellState,
            decoration::XdgDecorationState,
        },
        shm::ShmState,
        viewporter::ViewporterState,
        output::OutputManagerState,
    },
};

use sios_wm::{
    config::{
        build_window_rules,
        Config,
        ConfigCompiler,
        ConfigLoader,
        MonitorConfigLoader,
        WindowConfigLoader,
        CursorConfigLoader,
        ShaderConfigLoader,
    },
    desktop::{
        Desktop,
        Cursor,
        render::{
            Shader,
            ShaderCompositor,
            CursorRenderer,
        },
    },
    init::SIOSBackend,
    wayland::{
        ClientState,
        State,
    },
};


fn start_safety_timeout() {
    thread::spawn(|| {
        thread::sleep(
            Duration::from_secs(30),
        );

        eprintln!(
            "SIOS: 10-seconds safety timeout.",
        );

        std::process::exit(0);
    });
}

fn load_config() ->
    Result<Config, Box<dyn std::error::Error>>
{
    let compiler = 
        ConfigCompiler::new();

    let library =
        compiler.compile()?;

    let loader =
        ConfigLoader::load(
            &library
        )?;

    let mut config = Config::new();

    MonitorConfigLoader::load(
        &loader,
        &mut config.monitor,
    );
    WindowConfigLoader::load(
        &loader,
        &mut config.window,
    );
    CursorConfigLoader::load(
        &loader,
        &mut config.cursor,
    );
    ShaderConfigLoader::load(
        &loader,
        &mut config.shader,
    );

    Ok(config)
}

fn create_shaders(
    config: &Config
) -> Result<
    (Shader, Shader),
    Box<dyn std::error::Error>,
> {
    let rounded_border = 
        config
            .shader
            .rounded_border
            .as_ref()
            .map(Shader::from_api)
            .ok_or(
                "SIOS: Rounded border is not configured.",
            )?;

    let rounded_surface =
        config
            .shader
            .rounded_surface
            .as_ref()
            .map(Shader::from_api)
            .ok_or(
                "SIOS: Rounded surface is not configured.",
            )?;

    Ok((
        rounded_border,
        rounded_surface,
    ))
}

fn create_shader_compositor(
    backend: &mut SIOSBackend,
    rounded_border: &Shader,
    rounded_surface: &Shader,
) -> Result<
    ShaderCompositor,
    Box<dyn std::error::Error>,
> {
    ShaderCompositor::new(
        backend.compositor.renderer_mut(),
        rounded_border,
        rounded_surface,
    )
}

fn create_state(
    display: &Display<State>,
    desktop: Desktop,
    config: &Config,
) -> Result<State, Box<dyn std::error::Error>> {
    let display_handle = display.handle();

    let compositor_state =
        CompositorState::new::<State>(
            &display_handle,
        );

    let layer_shell_state = 
        WlrLayerShellState::new::<State>(
            &display_handle,
        );

    let viewporter_state =
        ViewporterState::new::<State>(
            &display_handle,
        );

    let output_manager_state = 
        OutputManagerState::new();

    let xdg_shell_state =
        XdgShellState::new::<State>(
            &display_handle,
        );

    let xdg_decoration_state =
        XdgDecorationState::new::<State>(
            &display_handle,
        );

    let mut seat_state =
        SeatState::<State>::new();

    let seat =
        seat_state.new_wl_seat(
            &display.handle(),
            "seat0",
        );

    let shm_state =
        ShmState::new::<State>(
            &display_handle,
            vec![
                wl_shm::Format::Argb8888,
                wl_shm::Format::Xrgb8888,
            ],
        );

    let mut state = State {
        display_handle,
        compositor_state,
        xdg_shell_state,
        xdg_decoration_state,
        seat_state,
        shm_state,
        seat,
        layer_shell_state,
        viewporter_state,
        output_manager_state,
        desktop,
        cursor: Cursor::new(&config.cursor),
        cursor_renderer: CursorRenderer::new(),
        drm_vblank: Vec::new(),
        frame_in_flight: false,
        redraw_requested: true,
    };

    state.seat.add_pointer();

    state.seat.add_keyboard(
        Default::default(),
        600,
        25,
    )?;

    Ok(state)
}

fn setup_wayland(
    handle: &LoopHandle<'_, State>,
) -> Result<(), Box<dyn std::error::Error>> {
    let socket = 
        ListeningSocketSource::new_auto()?;

    eprintln!(
        "SIOS: Wayland socket = {}",
        socket.socket_name().to_string_lossy(),
    );

    std::process::Command::new("kitty")
        .env(
            "WAYLAND_DISPLAY",
            socket.socket_name(),
        )
        .spawn()?;

    handle.insert_source(
        socket,
        |stream, _, state| {
            let client_state =
                ClientState {
                    compositor_state:
                        CompositorClientState::default(),
                };

            let client =
                state
                    .display_handle
                    .insert_client(
                        stream,
                        Arc::new(client_state)
                    )
                    .unwrap();

            eprintln!(
                "SIOS: Wayland client connected: {:?}",
                client,
            );
        },
    )?;

    Ok(())
}

fn send_frames_surface_tree(
    surface: &WlSurface,
    time: u32,
) {
    with_surface_tree_downward(
        surface,
        (),
        |_, _, &()| {
            TraversalAction::DoChildren(())
        },
        |_surface, states, &()| {
            for callback in states
                .cached_state
                .get::<SurfaceAttributes>()
                .current()
                .frame_callbacks
                .drain(..)
            {
                callback.done(time);
            }
        },
        |_, _, &()| true,
    );
}

fn run_event_loop(
    event_loop: &mut EventLoop<State>,
    display: &mut Display<State>,
    state: &mut State,
    backend: &mut SIOSBackend,
    shader_compositor: &ShaderCompositor,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    loop {
        event_loop.dispatch(
            Some(
                Duration::from_millis(60),
            ),
            state,
        )?;

        for crtc in state.drm_vblank.drain(..) {
            backend
                .compositor
                .frame_submitted(crtc);

            state.frame_in_flight = false;

            let time =
                start_time
                    .elapsed()
                    .as_millis()
                    as u32;

            for window
                in state.desktop.mapped_windows()
            {
                send_frames_surface_tree(
                    window
                        .surface()
                        .wl_surface(),
                    time,
                );
            }
        }

        match display.dispatch_clients(
            state,
        ) {
            Ok(_) => {}

            Err(error) => {
                eprintln!(
                    "SIOS: dispatch_cliens ERROR: {:?}",
                    error,
                );
            }
        }

        if state.redraw_requested 
            && !state.frame_in_flight
        {
            let queued = 
                backend
                    .compositor
                    .render_desktop(
                        &state.desktop,
                        &state.cursor,
                        &mut state.cursor_renderer,
                        shader_compositor,
                    )?;

            if queued {
                state.frame_in_flight = true;
                state.redraw_requested = false;
            }
        }

        display.flush_clients()?;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    start_safety_timeout();

    let mut event_loop = EventLoop::<State>::try_new()?;
    
    let handle = event_loop.handle();

    let mut display = Display::<State>::new()?;

    let config =
        load_config()?;

    let (
        mut backend,
        desktop,
    ) = SIOSBackend::init_all(
        &handle,
        &config.monitor,
    )?;

    let window_rules =
        build_window_rules(
            &config.window,
        );

    let (
        rounded_border,
        rounded_surface,
    ) =
        create_shaders(
            &config,
        )?;

    let shader_compositor =
        create_shader_compositor(
            &mut backend,
            &rounded_border,
            &rounded_surface,
        )?;

    let mut state =
        create_state(
            &display,
            desktop,
            &config,
        )?;

    state.desktop
        .apply_monitor_config(
            &config.monitor,
        );

    state.desktop
        .set_window_rules(
            window_rules,
        );

    setup_wayland(
        &handle,
    )?;

    run_event_loop(
        &mut event_loop,
        &mut display,
        &mut state,
        &mut backend,
        &shader_compositor,
    )?;

    Ok(())
}