use serde::{Deserialize, Serialize};

use super::team::Team;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    Planted,
    Exploded,
    Defused,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "phase")]
pub enum Round {
    FreezeTime,
    Over {
        win_team: Option<Team>,
        bomb: Option<BombState>,
    },
    Live {
        bomb: Option<BombState>,
    },
}
