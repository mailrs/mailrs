#[derive(Clone)]
pub struct Tag {
    inner: String,
}

impl std::fmt::Debug for Tag {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <String as std::fmt::Debug>::fmt(&self.inner, f)
    }
}

impl Tag {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl std::fmt::Display for Tag {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <String as std::fmt::Display>::fmt(&self.inner, f)
    }
}
