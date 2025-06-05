use spacetimedb::{table, Identity, ScheduleAt};

use crate::players::update_players_windows;

#[table(name = players_windows, public)]
#[derive(Clone, Copy)]
pub struct PlayerWindow {
    #[primary_key]
    pub id: Identity,
    pub lr_bl_x: f32,
    pub lr_bl_z: f32,
    pub lr_tr_x: f32,
    pub lr_tr_z: f32,
    pub lr_size: f32,
    pub hr_bl_x: f32,
    pub hr_bl_z: f32,
    pub hr_tr_x: f32,
    pub hr_tr_z: f32,
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
            lr_bl_x: x - half_lr_size,
            lr_bl_z: y - half_lr_size,
            lr_tr_x: x + half_lr_size,
            lr_tr_z: y + half_lr_size,
            lr_size,
            hr_bl_x: x - half_hr_size,
            hr_bl_z: y - half_hr_size,
            hr_tr_x: x + half_hr_size,
            hr_tr_z: y + half_hr_size,
            hr_size,
            recompute_distance: recompute_threshold * recompute_threshold,
        }
    }

    // Recompute the window boundaries based on the new position
    pub fn recompute(&mut self, x: f32, z: f32) {
        let half_lr_size = self.lr_size / 2.0;
        let half_hr_size = self.hr_size / 2.0;

        self.lr_bl_x = x - half_lr_size;
        self.lr_bl_z = z - half_lr_size;
        self.lr_tr_x = x + half_lr_size;
        self.lr_tr_z = z + half_lr_size;

        self.hr_bl_x = x - half_hr_size;
        self.hr_bl_z = z - half_hr_size;
        self.hr_tr_x = x + half_hr_size;
        self.hr_tr_z = z + half_hr_size;
    }
}

#[table(name = players_window_updates, private, scheduled(update_players_windows))]
pub struct PlayerWindowUpdate {
    #[primary_key]
    pub id: u64,
    pub scheduled_at: ScheduleAt,
}
