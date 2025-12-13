use internment::Intern;
use std::{
    cmp::{self, Ordering},
    fmt::Display,
    ops::{Deref, DerefMut, Range},
};

use crate::src::Src;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub src: Intern<Src>,

    // tuple instead of Range<usize> because Range<usize> doesn't implement copy
    // private to ensure start <= end
    range: (usize, usize),
}

impl Span {
    pub fn new(src: Intern<Src>, range: Range<usize>) -> Self {
        assert!(range.start <= range.end);
        Self {
            src,
            range: (range.start, range.end),
        }
    }

    pub(crate) fn spanned<T>(self, val: T) -> Spanned<T> {
        Spanned::new(val, self)
    }

    pub fn range(&self) -> Range<usize> {
        self.range.0..self.range.1
    }

    pub fn start(&self) -> usize {
        self.range.0
    }

    pub fn end(&self) -> usize {
        self.range.1
    }

    // None if no intersection
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.src != other.src {
            // no intersection since they are in seperate files
            return None;
        }

        let start = cmp::max(self.start(), other.start());
        let end = cmp::min(self.end(), other.end());

        if start <= end {
            Some(Self::new(self.src, start..end))
        } else {
            None
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }

    pub fn union(&self, other: &Self) -> Self {
        assert!(self.src == other.src);
        Self {
            src: self.src,
            range: (self.start().min(other.start()), self.end().max(other.end())),
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}..{}", self.src, self.start(), self.end())
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.src == other.src {
            self.start().partial_cmp(&other.start())
        } else {
            None
        }
    }
}

impl chumsky::span::Span for Span {
    type Context = Intern<Src>;
    type Offset = usize;

    fn new(src: Self::Context, range: Range<Self::Offset>) -> Self {
        Span::new(src, range)
    }

    fn context(&self) -> Self::Context {
        self.src
    }

    fn start(&self) -> Self::Offset {
        self.start()
    }

    fn end(&self) -> Self::Offset {
        self.end()
    }
}

impl ariadne::Span for Span {
    type SourceId = Intern<Src>;

    fn source(&self) -> &Self::SourceId {
        &self.src
    }

    fn start(&self) -> usize {
        self.start()
    }

    fn end(&self) -> usize {
        self.end()
    }
}

// just a struct to hold a span and a value for error messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Spanned<T> {
    pub(crate) val: T,
    pub(crate) span: Span,
}

impl<T> Spanned<T> {
    pub(crate) fn new(val: T, span: Span) -> Self {
        Self { val, span }
    }

    // converts &Spanned<T> to Spanned<&T>
    pub(crate) fn as_ref(&self) -> Spanned<&T> {
        Spanned::new(&self.val, self.span)
    }

    pub(crate) fn as_mut_ref(&mut self) -> Spanned<&mut T> {
        Spanned::new(&mut self.val, self.span)
    }

    pub(crate) fn span_to<V>(&self, to: V) -> Spanned<V> {
        Spanned::new(to, self.span)
    }
}

impl<T: Copy> Spanned<&T> {
    pub(crate) fn copied(&self) -> Spanned<T> {
        Spanned::new(*self.val, self.span)
    }
}

impl<T> Spanned<&mut T> {
    pub(crate) fn to_borrow(&self) -> Spanned<&T> {
        Spanned::new(self.val, self.span)
    }
}

// just for simplicity (i.e. removes ".val" everywhere)
impl<T> Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl<'a, T> Spanned<&'a Option<T>> {
    // generic over any value with a spanned result
    // but mostly for generator, because the expression result during generation should always be Some
    pub(crate) fn unwrap(&self) -> Spanned<&'a T> {
        let val = self.val.as_ref().unwrap_or_else(|| {
            panic!(
                "Option at {} was None while attempting to unwrap",
                self.span
            )
        });
        self.span_to(val)
    }
}
