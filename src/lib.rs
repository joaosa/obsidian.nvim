use nvim_oxi::{self as oxi, object, Dictionary, Function, Object, ObjectKind, Result};
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub dir: String,
}

fn setup(preferences: Object) -> Result<()> {
    let config = match preferences.kind() {
        ObjectKind::Nil => Config::default(),
        _ => {
            let deserializer = object::Deserializer::new(preferences);
            Config::deserialize(deserializer)?
        }
    };

    oxi::print!("All good! {config:?}");

    Ok(())
}

#[oxi::module]
fn obsidian() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([(
        "setup",
        Object::from(Function::from_fn(setup)),
    )]))
}
