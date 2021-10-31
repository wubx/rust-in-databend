struct Test {
    a: String,
    b: *const String, // 改成指针
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn b(&self) -> &String {
        unsafe {&*(self.b)}
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    test1.init();

    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a, test1.b());

    // 使用swap()函数交换两者, 这里发生了move
    std::mem::swap(&mut test1, &mut test2);

    test1.a = "xxxxxxx".to_string();

    println!("a: {}, b: {}", test2.a, test2.b());
}