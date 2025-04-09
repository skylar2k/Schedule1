use schedule1::{LevelManager, Rank};

#[test]
fn test_streetrat_one() {
    let level = LevelManager::new(Rank::StreetRat, 1);
    assert_eq!(level.order_limit_multiplier(), 1.0);
}

#[test]
fn test_peddler_three() {
    let level = LevelManager::new(Rank::Peddler, 3);
    assert_eq!(level.order_limit_multiplier(), 1.625)
}