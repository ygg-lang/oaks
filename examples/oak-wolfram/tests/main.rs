#![feature(new_range_api)]

mod lexer;
mod parser;

#[test]
fn ready() {
    println!("oak-wolfram tests ready!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_functionality() {
        // 基础功能测试
        assert!(true);
    }
}
