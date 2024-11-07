use api::{Greeter, Message};

struct Someone;

impl Greeter for Someone {
    fn new() -> Self {
        Self
    }

    fn greet(&self, msg: Message) {
        println!("Oh! {}, my name is {}", msg.content, msg.name);
        api::cool_print();
    }
}

api::register_plugin!(Someone);
