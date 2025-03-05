use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Over,
    Live,
    Bomb,
    Defuse,
    Warmup,
    FreezeTime,
    #[serde(rename = "timeout_t")]
    TimeoutT,
    #[serde(rename = "timeout_ct")]
    TimeoutCT,
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhaseCountdowns {
    pub phase: Phase,
    #[serde_as(as = "DisplayFromStr")]
    pub phase_ends_in: f64, // not sure what the max is
}
