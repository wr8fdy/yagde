use yagde::gd::char;
use std::path::Path;

#[test]
fn reset_deaths() {
    const PATH: &str = "./tests/save/_TestMain/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestMain/test_reset_deaths.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    let new_path = &Path::new(&NEW_PATH).to_path_buf();

    new_char.reset_deaths().save_as(new_path).unwrap();
    new_char.read(new_path).unwrap();

    assert_eq!(new_char.stats.deaths, 0);
}

#[test]
fn reset_attributes() {
    const PATH: &str = "./tests/save/_TestMain/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestMain/test_reset_attributes.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    let new_path = &Path::new(&NEW_PATH).to_path_buf();

    new_char.reset_attributes().save_as(new_path).unwrap();

    let mut new_char = char::Char::new();
    new_char.read(new_path).unwrap();

    assert!(new_char.bio.attribute_points >= new_char.header.level);

    assert_eq!(new_char.bio.physique, 50.0);
    assert_eq!(new_char.bio.cunning, 50.0);
    assert_eq!(new_char.bio.spirit, 50.0);
}

#[test]
fn reset_devotions() {
    const PATH: &str = "./tests/save/_TestMain/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestMain/test_reset_devotions.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    let new_path = &Path::new(&NEW_PATH).to_path_buf();
    new_char.reset_devotions().save_as(new_path).unwrap();

    new_char.read(&Path::new(&NEW_PATH).to_path_buf()).unwrap();

    assert_eq!(new_char.bio.devotion_points, new_char.bio.total_devotion);
}

#[test]
fn reset_skills() {
    const PATH: &str = "./tests/save/_TestMain/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestMain/test_reset_skills.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    let new_path = &Path::new(&NEW_PATH).to_path_buf();
    new_char.reset_skills().save_as(new_path).unwrap();

    new_char.read(&Path::new(&NEW_PATH).to_path_buf()).unwrap();

    /*
       3 Skill Points per level from Levels 2 to 50 (147 Skill Points)
       2 Skill Points per level from Levels 51 to 90 (80 Skill Points)
       1 Skill Point per level from Levels 91 to 100 (10 Skill Points)
    */
    let sp = match new_char.header.level {
        0..=1 => 0,
        2..=50 => (new_char.header.level - 1) * 3,
        51..=90 => 147 + (new_char.header.level - 50) * 2,
        91.. => 147 + 80 + new_char.header.level - 90,
    };
    assert!(new_char.bio.skill_points >= sp);
}
