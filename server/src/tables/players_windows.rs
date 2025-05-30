use spacetimedb::{table, Identity, ScheduleAt};

use crate::players::update_players_windows;

#[table(name = players_windows, public)]
#[derive(Clone, Copy)]
pub struct PlayerWindow {
    #[primary_key]
    pub id: Identity,
    pub lr_br_x: f32,
    pub lr_br_y: f32,
    pub lr_tl_x: f32,
    pub lr_tl_y: f32,
    pub lr_size: f32,
    pub hr_br_x: f32,
    pub hr_br_y: f32,
    pub hr_tl_x: f32,
    pub hr_tl_y: f32,
    pub hr_size: f32,
    pub recompute_distance: f32,
}

impl PlayerWindow {
    pub fn new(id: Identity, x: f32, y: f32, lr_size: f32, hr_size: f32) -> Self {
        let half_lr_size = lr_size / 2.0;
        let half_hr_size = hr_size / 2.0;

        let recompute_threshold = hr_size / 8.0;
        Self {
            id,
            lr_br_x: x - half_lr_size,
            lr_br_y: y - half_lr_size,
            lr_tl_x: x + half_lr_size,
            lr_tl_y: y + half_lr_size,
            lr_size,
            hr_br_x: x - half_hr_size,
            hr_br_y: y - half_hr_size,
            hr_tl_x: x + half_hr_size,
            hr_tl_y: y + half_hr_size,
            hr_size,
            recompute_distance: recompute_threshold * recompute_threshold,
        }
    }

    // Recompute the window boundaries based on the new position
    pub fn recompute(&mut self, x: f32, y: f32) {
        let half_lr_size = self.lr_size / 2.0;
        let half_hr_size = self.hr_size / 2.0;

        self.lr_br_x = x - half_lr_size;
        self.lr_br_y = y - half_lr_size;
        self.lr_tl_x = x + half_lr_size;
        self.lr_tl_y = y + half_lr_size;

        self.hr_br_x = x - half_hr_size;
        self.hr_br_y = y - half_hr_size;
        self.hr_tl_x = x + half_hr_size;
        self.hr_tl_y = y + half_hr_size;
    }
}

#[table(name = players_window_updates, private, scheduled(update_players_windows))]
pub struct PlayerWindowUpdate {
    #[primary_key]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}
