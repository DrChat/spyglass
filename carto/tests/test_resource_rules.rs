use carto::models::ResourceRule;
use rusqlite::Connection;

#[test]
fn test_init() {
    let db = Connection::open_in_memory().unwrap();
    ResourceRule::init_table(&db);
}

#[test]
fn test_insert() {
    let db = Connection::open_in_memory().unwrap();
    ResourceRule::init_table(&db);

    let allow = Vec::new();
    let disallow = Vec::new();

    let res = ResourceRule::insert(&db, "oldschool.runescape.wiki", "/", false, true);
    assert!(res.is_ok());

    let rules = ResourceRule::find(&db, "oldschool.runescape.wiki");
    assert_eq!(rules.len(), 1);
}
