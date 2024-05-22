pub fn init() {
    println!("jvm initialising...");
}

inventory::submit! {
    super::Component{ init }
}
