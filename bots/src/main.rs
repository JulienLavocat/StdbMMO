use std::time::{Duration, Instant};

use bindings::{DbConnection, move_player};
use clap::Parser;
use rand::random_range;
use spacetimedb_sdk::DbContext;
use tokio::time::{interval, sleep};

const MODULE_NAME: &str = "ariaonline";
const STDB_URI: &str = "https://stdb.jlavocat.eu";
// const STDB_URI: &str = "https://maincloud.spacetimedb.com";
const MOVE_SPEED: f32 = 0.4;
const BOUNDS: f32 = 128.0; // Movement bounds for the bots

#[derive(Parser, Debug, Clone, Copy)]
struct Args {
    /// Number of bot threads to run
    num_bots: usize,
    #[clap(short, default_value = "false")]
    /// If true, bots will not subscribe to the database
    no_subscribe: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let num_bots = args.num_bots;

    println!("Running with arguments: {:?}", args);

    let mut handles = vec![];

    for id in 0..num_bots {
        let handle = tokio::spawn(run_bot(id, args));
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }
}

async fn run_bot(id: usize, args: Args) {
    let delay = id * 25;
    sleep(Duration::from_millis(delay as u64)).await;

    let conn = DbConnection::builder()
        .with_module_name(MODULE_NAME)
        .on_connect(move |ctx, _id, _c| {
            println!("[{}] Connected to SpacetimeDB as {}", id, ctx.identity());
            if args.no_subscribe {
                return;
            }

            ctx.subscription_builder()
                .on_error(move |_, err| {
                    panic!("[{}] Subscription error: {}", id, err);
                })
                .subscribe([
                    "SELECT * FROM players",
                    "SELECT * FROM players_positions",
                    "SELECT * FROM players_positions_lr",
                ]);
        })
        .with_uri(STDB_URI)
        .build()
        .expect("Failed to create SpacetimeDB connection");

    let mut current_position: (f32, f32) = (0.0, 0.0);
    let mut goal = (random_range(-BOUNDS..BOUNDS), random_range(-BOUNDS..BOUNDS));
    let mut last_goal_update = Instant::now();
    let mut move_interval = interval(Duration::from_millis(100));

    loop {
        tokio::select! {
            result = conn.advance_one_message_async() => {
                if let Err(e) = result {
                    eprintln!("[{}] Error advancing message: {:?}", id, e);
                    return;
                }
            }

            _ = move_interval.tick() => {
                if last_goal_update.elapsed().as_secs_f32() > 5.0 {
                    last_goal_update = Instant::now();
                    goal = (random_range(-BOUNDS..BOUNDS), random_range(-BOUNDS..BOUNDS));
                }

                let dx = goal.0 - current_position.0;
                let dz = goal.1 - current_position.1;
                let dist = (dx.powi(2) + dz.powi(2)).sqrt();

                // Avoid divide-by-zero
                if dist > 0.01 {
                    let dir_x = dx / dist;
                    let dir_z = dz / dist;

                    let pos_x = current_position.0 + dir_x * MOVE_SPEED;
                    let pos_z = current_position.1 + dir_z * MOVE_SPEED;

                    current_position = (pos_x, pos_z);
                    if let Err(e) = conn.reducers.move_player(pos_x, 1.0, pos_z) {
                        eprintln!("[{}] Error moving player: {:?}", id, e);
                        return;
                    }
                }
            }
        }
    }
}
