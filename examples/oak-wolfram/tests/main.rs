#[test]
fn ready() {
    println!("oak-wat tests ready!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // 基础功能测试
        assert!(true);
    }
}
