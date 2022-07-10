use confy;
use serde::{Deserialize, Serialize};
// use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}
impl std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            name: "default name for the config".to_string(),
            comfy: true,
            foo: 42_i64,
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    // now we run this first time it should still load
    // dummy cfg not default cfg
    // let dummy_cfg = MyConfig {
    //     name: "dummy".to_string(),
    //     comfy: true,
    //     foo: 42_i64,
    // };
    // confy::store("app_config", dummy_cfg)?;
    let mut cfg: MyConfig = confy::load("app_config")?;
    println!("currently load cfg -- {:#?}", cfg);
    println!("incremeting foo value");
    cfg.foo += 1_i64;
    confy::store("app_config", cfg)?;
    Ok(())
}
