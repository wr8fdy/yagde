use yagde::gd::char;
use std::fs::File;
use std::path::Path;

#[test]
fn save_as() {
    const PATH: &str = "./tests/save/_TestVanillaFemaleHC/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestVanillaFemaleHC/test_clone.gdc";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    new_char
        .save_as(&Path::new(&NEW_PATH).to_path_buf())
        .unwrap();

    let file1 = match File::open(PATH) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let file2 = match File::open(NEW_PATH) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    assert_eq!(
        file1.metadata().unwrap().len(),
        file2.metadata().unwrap().len()
    );
    assert_eq!(new_char, current_char);
}

#[test]
fn rename() {
    const PATH: &str = "./tests/save/_TestVanillaFemaleHC/player.gdc";
    const NEW_PATH: &str = "./tests/save/_TestVanillaFemaleHC/test_rename.gdc";
    const NEW_NAME: &str = "test_rename";

    let mut current_char = char::Char::new();
    current_char.read(&Path::new(&PATH).to_path_buf()).unwrap();

    let mut new_char = current_char.clone();
    let new_path = &Path::new(&NEW_PATH).to_path_buf();

    new_char.rename(NEW_NAME).save_as(new_path).unwrap();

    let mut new_char = char::Char::new();
    new_char.read(new_path).unwrap();

    assert_eq!(new_char.header.name, NEW_NAME);

    new_char.header.name = current_char.header.name.clone();

    assert_eq!(new_char, current_char);
}
