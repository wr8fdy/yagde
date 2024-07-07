use std::path::Path;
use yagde::gd::char;

#[test]
fn read_vanilla_female_hc() {
    const PATH: &str = "./tests/save/_TestVanillaFemaleHC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestVanillaFemaleHC");
    assert_eq!(current_char.header.hardcore, 1);
    assert_eq!(current_char.header.sex as u8, 0);
    assert_eq!(current_char.header.sex, char::Sex::Female);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::Vanilla
    );
    assert_eq!(current_char.header.level, 2);
}

#[test]
fn read_vanilla_male_sc() {
    const PATH: &str = "./tests/save/_TestVanillaMaleSC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestVanillaMaleSC");
    assert_eq!(current_char.header.hardcore, 0);
    assert_eq!(current_char.header.sex as u8, 1);
    assert_eq!(current_char.header.sex, char::Sex::Male);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::Vanilla
    );
    assert_eq!(current_char.header.level, 2);
}

#[test]
fn read_aom_female_hc() {
    const PATH: &str = "./tests/save/_TestAoMFemaleHC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestAoMFemaleHC");
    assert_eq!(current_char.header.hardcore, 1);
    assert_eq!(current_char.header.sex as u8, 0);
    assert_eq!(current_char.header.sex, char::Sex::Female);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::AshesOfMalmouth
    );
    assert_eq!(current_char.header.level, 2);
}

#[test]
fn read_aom_male_sc() {
    const PATH: &str = "./tests/save/_TestAoMMaleSC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestAoMMaleSC");
    assert_eq!(current_char.header.hardcore, 0);
    assert_eq!(current_char.header.sex as u8, 1);
    assert_eq!(current_char.header.sex, char::Sex::Male);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::AshesOfMalmouth
    );
    assert_eq!(current_char.header.level, 1);
}

#[test]
fn read_fg_female_hc() {
    const PATH: &str = "./tests/save/_TestFGFemaleHC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestFGFemaleHC");
    assert_eq!(current_char.header.hardcore, 1);
    assert_eq!(current_char.header.sex as u8, 0);
    assert_eq!(current_char.header.sex, char::Sex::Female);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::ForgottenGods
    );
    assert_eq!(current_char.header.level, 1);
}

#[test]
fn read_fg_male_sc() {
    const PATH: &str = "./tests/save/_TestFGMaleSC/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestFGMaleSC");
    assert_eq!(current_char.header.hardcore, 0);
    assert_eq!(current_char.header.sex as u8, 1);
    assert_eq!(current_char.header.sex, char::Sex::Male);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::ForgottenGods
    );
    assert_eq!(current_char.header.level, 1);
}

#[test]
fn read_v121() {
    const PATH: &str = "./tests/save/_TestMain121/player.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    assert_eq!(current_char.header.name, "TestOneTwoOne");
    assert_eq!(current_char.header.hardcore, 1);
    assert_eq!(current_char.header.sex as u8, 1);
    assert_eq!(current_char.header.sex, char::Sex::Male);
    assert_eq!(
        current_char.header.expansion_status,
        char::ExpansionStatus::Crucible
    );
    assert_eq!(current_char.header.level, 100);
}
