
mod level;

pub fn write(msg: String) {
    println!("{} ", msg);
}



#[cfg(test)]
mod log_tests {
    use crate::write;

    #[test]
    fn log_test() {
        write(String::from("hello world"));
    }

}