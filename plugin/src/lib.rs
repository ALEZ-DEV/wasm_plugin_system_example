use api::{Greeter, Message};

struct Someone;

impl Greeter for Someone {
    fn new() -> Self {
        Self
    }

    fn greet(&self, msg: Message) {
        println!("Oh! hello {}, my name is {}", msg.content, msg.name);
    }
}

api::register_plugin!(Someone);
