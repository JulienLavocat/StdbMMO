use std::thread;

use bindings::{DbConnection, move_player};
use rand::random_range;
use spacetimedb_sdk::DbContext;

const MODULE_NAME: &str = "ariaonline";
const STDB_URI: &str = "https://stdb.jlavocat.eu";
const MOVE_SPEED: f32 = 0.25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num_bots: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);

    let mut bots = Vec::new();
    println!("Starting {} bot threads", num_bots);

    for i in 0..num_bots {
        bots.push(thread::spawn(move || {
            run_bot(i);
        }));
        thread::sleep(std::time::Duration::from_millis(250));
    }

    for bots in bots {
        bots.join().unwrap();
    }
}

fn run_bot(id: usize) {
    println!("[{}] Starting bot thread", id);
    let conn = DbConnection::builder()
        .with_module_name(MODULE_NAME)
        .on_connect(move |ctx, _id, _c| {
            println!("[{}] Connected to SpacetimeDB as {}", id, ctx.identity());
            ctx.subscription_builder().subscribe_to_all_tables();
        })
        .with_uri(STDB_URI)
        .build()
        .expect("Failed to create SpacetimeDB connection");

    let mut current_position: (f32, f32) = (0.0, 0.0);
    let mut goal = (random_range(-40.0..40.0), random_range(-40.0..40.0));
    let mut last_goal_update = std::time::Instant::now();
    loop {
        conn.frame_tick().unwrap();

        if last_goal_update.elapsed().as_secs_f32() > 5.0 {
            last_goal_update = std::time::Instant::now();
            goal = (random_range(-40.0..40.0), random_range(-40.0..40.0));
        }

        let dx = goal.0 - current_position.0;
        let dz = goal.1 - current_position.1;
        let dist: f32 = (dx.powi(2) + dz.powi(2)).sqrt();

        let dir_x = dx / dist;
        let dir_z = dz / dist;

        let pos_x = current_position.0 + dir_x * MOVE_SPEED;
        let pos_z = current_position.1 + dir_z * MOVE_SPEED;

        current_position = (pos_x, pos_z);
        conn.reducers.move_player(pos_x, 1.0, pos_z).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
