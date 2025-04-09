use core::convert::TryFrom;

use crate::{leveling::rank::Rank, math::lerp};

pub struct LevelManager {
    xp: u32,
    rank: Rank,
    tier: u32,
}

impl LevelManager {
    pub fn new(rank: Rank, tier: u32) -> Self {
        Self { xp: 0, rank, tier }
    }
    // Total XP needed to rank up.
    pub fn order_limit_multiplier(&self) -> f32 {
        if self.rank < Rank::Kingpin {
            if let Ok(next_rank) = Rank::try_from(self.rank as u8 + 1u8) {
                let t = (self.tier as f32 - 1.0) / 4.0;
                return lerp(
                    self.rank.order_multiplier(),
                    next_rank.order_multiplier(),
                    t,
                );
            }
        }
        (self.order_limit_multiplier() + 0.1 * (self.tier as f32 - 1.0)).clamp(1.0, 10.0)
    }
}
