use internment::Intern;
use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    src: Intern<String>,
    range: (usize, usize),
}

impl Span {
    pub fn new(src: Intern<String>, range: Range<usize>) -> Self {
        assert!(range.start <= range.end);
        Self {
            src,
            range: (range.start, range.end),
        }
    }

    pub fn src(&self) -> Intern<String> {
        self.src
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.range.0..self.range.1
    }

    pub fn start(&self) -> usize {
        self.range.0
    }
    pub fn end(&self) -> usize {
        self.range.1
    }

    pub fn union(&self, other: &Self) -> Self {
        assert!(self.src == other.src);
        Self {
            src: self.src,
            range: (self.start().min(other.start()), self.end().max(other.end())),
        }
    }
}

impl chumsky::Span for Span {
    type Context = Intern<String>;
    type Offset = usize;

    fn new(src: Intern<String>, range: Range<usize>) -> Self {
        assert!(range.start <= range.end);
        Self {
            src,
            range: (range.start, range.end),
        }
    }

    fn context(&self) -> Self::Context {
        self.src
    }
    fn start(&self) -> Self::Offset {
        self.range.0
    }
    fn end(&self) -> Self::Offset {
        self.range.1
    }
}

impl ariadne::Span for Span {
    type SourceId = Intern<String>;

    fn source(&self) -> &Self::SourceId {
        &self.src
    }

    fn start(&self) -> usize {
        self.range.0
    }

    fn end(&self) -> usize {
        self.range.1
    }
}
