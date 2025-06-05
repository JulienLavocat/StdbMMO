use spacetimedb::{client_visibility_filter, Filter};

#[client_visibility_filter]
const ONLINE_PLAYERS: Filter = Filter::Sql("SELECT * FROM players WHERE online = true");

#[client_visibility_filter]
// Only show players within the high-resolution window of the sender on players_positions
const PLAYERS_POSITIONS_FILTER: Filter = Filter::Sql(
    "SELECT p.* FROM players_positions p
        JOIN players_windows pw
        WHERE pw.id = :sender
            AND p.x >= pw.hr_bl_x AND p.x <= pw.hr_tr_x
            AND p.z >= pw.hr_bl_z AND p.z <= pw.hr_tr_z
        ",
);

#[client_visibility_filter]
// Only show players in the LR window but outside the HR window
const PLAYERS_POSITIONS_LR_FILTER: Filter = Filter::Sql(
    "SELECT p.* FROM players_positions_lr p
        JOIN players_windows pw
        WHERE
            pw.id = :sender
            AND p.x >= pw.lr_bl_x AND p.x <= pw.lr_tr_x
            AND p.z >= pw.lr_bl_z AND p.z <= pw.lr_tr_z
            AND (
                p.x < pw.hr_bl_x OR p.x > pw.hr_tr_x OR
                p.z < pw.hr_bl_z OR p.z > pw.hr_tr_z
        )
    ",
);
