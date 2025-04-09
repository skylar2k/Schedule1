use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, TryFromPrimitive)]
pub enum Rank {
    StreetRat,
    Hoodlum,
    Peddler,
    Hustler,
    Bagman,
    Enforcer,
    ShotCaller,
    BlockBoss,
    Underlord,
    Baron,
    Kingpin,
}

impl Rank {
    // Increases by 25% for each rank
    pub fn order_multiplier(self) -> f32 {
        (25.0 * self as u8 as f32) / 100.0 + 1.0
    }
}