use std::ops::Range;
use internment::Intern;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    src: Intern<String>,
    range: (usize, usize) 
}

impl Span {
    pub fn src(&self) -> Intern<String> {
        return self.src;
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        return self.range.0..self.range.1;
    }

    pub fn start(&self) -> usize { self.range.0 }
    pub fn end(&self) -> usize { self.range.1 }
}

impl chumsky::Span for Span {
    type Context = Intern<String>;
    type Offset = usize;

    fn new(src: Intern<String>, range: Range<usize>) -> Self {
        assert!(range.start <= range.end);
        Self { src, range: (range.start, range.end) }
    }

    fn context(&self) -> Intern<String> { self.src }
    fn start(&self) -> Self::Offset { self.range.0 }
    fn end(&self) -> Self::Offset { self.range.1 }
}
 
impl ariadne::Span for Span {
    type SourceId = Intern<String>;

    fn source(&self) -> &Intern<String> { &self.src }

    fn start(&self) -> usize { self.range.0 }
    fn end(&self) -> usize { self.range.1 }
}
