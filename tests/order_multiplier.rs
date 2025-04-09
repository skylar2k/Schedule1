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

#[test]
fn test_kingpin_one() {
    let level = LevelManager::new(Rank::Kingpin, 1);
    assert_eq!(level.order_limit_multiplier(), 3.5)
}

#[test]
fn test_kingpin_seventeen() {
    let level = LevelManager::new(Rank::Kingpin, 17);
    assert_eq!(level.order_limit_multiplier(), 5.1)

}