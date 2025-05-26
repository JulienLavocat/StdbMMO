use spacetimedb::{client_visibility_filter, Filter};

#[client_visibility_filter]
const ONLINE_PLAYERS: Filter = Filter::Sql("SELECT * FROM players WHERE online = true");

#[client_visibility_filter]
// Only show players within the high-resolution window of the sender on players_positions
const PLAYERS_POSITIONS_FILTER: Filter = Filter::Sql(
    "SELECT p.* FROM players_positions p
        JOIN players_windows pw
        WHERE pw.id = :sender
        AND (
            p.x >= pw.hr_br_x AND p.x <= pw.hr_tl_x AND
            p.y >= pw.hr_br_y AND p.y <= pw.hr_tl_y
        )
        ",
);

#[client_visibility_filter]
// Only show players in the LR window but outside the HR window
const PLAYERS_POSITIONS_LR_FILTER: Filter = Filter::Sql(
    "SELECT p.* FROM players_positions_lr p
        JOIN players_windows pw
        WHERE
            pw.id = :sender 
            AND p.x >= pw.lr_br_x AND p.x <= pw.lr_tl_x
            AND p.y >= pw.lr_br_y AND p.y <= pw.lr_tl_y
            AND (
                p.x < pw.hr_br_x OR p.x > pw.hr_tl_x
                OR p.y < pw.hr_br_y OR p.y > pw.hr_tl_y
            )
    ",
);
