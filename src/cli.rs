pub struct Cli {
    pub args: i32,
}

impl Cli {
    pub fn say(&self) {
        println!("args {}", self.args)
    }
}
