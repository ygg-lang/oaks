use core::marker::PhantomData;

/// Pratt 解析器
pub struct PrattParser<K> {
    token: PhantomData<K>,
}

impl<K> PrattParser<K> {
    /// 创建新的 Pratt 解析器
    pub fn new() -> Self {
        Self {
            token: PhantomData,
        }
    }
}
