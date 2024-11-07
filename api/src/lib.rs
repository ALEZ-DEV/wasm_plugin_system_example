use std::sync::{Mutex, MutexGuard};

use anyhow::Result;

pub trait Greeter: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn greet(&self, msg: Message);
}

wit_bindgen::generate!({
    skip: ["init-plugin"],
    path: "./wit",
    world: "api",
});

static PLUGIN: Mutex<Option<Box<dyn Greeter>>> = Mutex::new(None);

#[macro_export]
macro_rules! register_plugin {
    ($extension_type:ty) => {
        #[export_name = "init-plugin"]
        pub extern "C" fn __init_extension() {
            api::load_plugin(Box::new(<$extension_type as api::Greeter>::new()));
        }
    };
}

pub fn load_plugin(greeter: Box<dyn Greeter>) -> Result<()> {
    let mut binding = PLUGIN.lock();
    let plugin = binding.as_mut();
    if let Ok(p) = plugin {
        **p = Some(greeter);
    } else {
        anyhow::bail!("Fail to load the plugin");
    }

    Ok(())
}

fn get_plugin<'a>() -> Result<MutexGuard<'a, Option<Box<dyn Greeter>>>> {
    let plugin = PLUGIN.lock();
    if let Ok(p) = plugin {
        Ok(p)
    } else {
        anyhow::bail!("Failed to use the plugin");
    }
}

export!(Component);

#[warn(dead_code)]
struct Component;

impl Guest for Component {
    fn greet(msg: Message) {
        let instance = get_plugin();
        if let Ok(p) = instance {
            p.as_ref().unwrap().greet(msg);
        }
    }
}
