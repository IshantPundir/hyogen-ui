use hyogen_ui::{hvf, layer::HyogenLayer};
use smithay_client_toolkit::{compositor::CompositorState, shell::{wlr_layer::{Anchor, Layer, LayerShell}, WaylandSurface}, shm::{slot::SlotPool, Shm}};
use wayland_client::{globals::registry_queue_init, Connection};

use std::{env, time::Duration};

fn main() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }
    
    // Retrive hvf file path from CLI.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        tracing::error!("hvf file was not provided!");
        tracing::info!("Usage: {} <path_to_hvf_file>", args[0]);
        return;
    }
    
    tracing::info!("Welcome to Hyogen UI");

    // All wayland clients start by connecting to the compositor (Aurora).
    let conn = Connection::connect_to_env().unwrap();

    // Enumerate the list of glovals to get the protocols the server implements.
    let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
    let qh = event_queue.handle();

    // The compositor allows configuring surfaces to be presented.
    let compositor = CompositorState::bind(&globals, &qh).expect("wl_compositor is not availabel");

    // HyogenUI uses the wlr layer shell, so make sure the compositor supports it. (Aurora does)
    let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");

    // Using wl_shm to allow software rendering to a buffer, shared with the compositor process.
    // TODO: Use GPU for rendering.
    let shm = Shm::bind(&globals, &qh).expect("wl_shm is not available");

    // A layer surface is created from a surface.
    let surface = compositor.create_surface(&qh);
    // And then we create a layer shell
    let layer = layer_shell.create_layer_surface(&qh, surface, Layer::Background, Some("Hyogen_UI"), None);

    // Configure the layer surface, providing things like the anchor on screen, desired size, etc.
    layer.set_anchor(Anchor::TOP);
    layer.set_size(800, 480);

    // In order for layer surface to be mapped, we need to perform an initial commit with no attached buffer
    // The compositor will respond with an initial configure that we can then use to present to the layer
    // surface with the correct options.
    layer.commit();

    // We don't know how large the window will be yet, so lets assume the minimum size we suggested for the initial memory allocation.
    let pool = SlotPool::new(800 * 480 * 4, &shm).expect("failed to create pool");
    
    // Load hvf files;
    let hvf_loader = hvf::loader::HVFLoader::new(&args[1]).unwrap();

    let default_ring = hvf_loader.get("expression", "neutral").unwrap();
    let blink_ring = hvf_loader.get("expression", "blink").unwrap();

    // Initialize hyogen layer
    let mut hyogen_layer = HyogenLayer::new(layer, &globals, &qh, shm, pool, default_ring);

    hyogen_layer.animator.animate(blink_ring, Duration::from_secs_f64(1.4));
    // let blink_animation = Blink::new(blink_ring, &hyogen_layer.animator);
    // blink_animation.start();

    // We don't draw immediately, the configure will notify us when to first draw.
    loop {
        event_queue.blocking_dispatch(&mut hyogen_layer).unwrap();

        if hyogen_layer.exit() {
            tracing::info!("Exiting Hyogen UI");
            break;
        }
    }
}