// MATLAB 错误处理 - 占位符实
#[derive(Debug, Clone, PartialEq)]
pub struct MatlabError {
    pub message: &'static str,
}

impl MatlabError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}
