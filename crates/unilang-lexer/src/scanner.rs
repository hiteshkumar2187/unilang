// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

/// Low-level character scanner over UTF-8 source text.
pub struct Scanner<'src> {
    source: &'src str,
    bytes: &'src [u8],
    pos: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    /// Current byte offset into the source.
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns `true` when there are no more characters.
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    /// Peek at the current character without advancing.
    #[inline]
    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source[self.pos..].chars().next()
        }
    }

    /// Peek at the character `n` positions ahead (0 = current).
    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }

    /// Peek at the next byte without advancing (fast path for ASCII).
    #[inline]
    pub fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Peek at the byte `n` positions ahead.
    #[inline]
    pub fn peek_byte_at(&self, n: usize) -> Option<u8> {
        self.bytes.get(self.pos + n).copied()
    }

    /// Advance by one character and return it.
    pub fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let ch = self.source[self.pos..].chars().next()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    /// Advance by `n` bytes (caller must ensure this doesn't split a char).
    #[inline]
    pub fn advance_by(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.bytes.len());
    }

    /// Advance if the current character satisfies the predicate.
    pub fn eat_if(&mut self, pred: impl FnOnce(char) -> bool) -> bool {
        match self.peek() {
            Some(ch) if pred(ch) => {
                self.pos += ch.len_utf8();
                true
            }
            _ => false,
        }
    }

    /// Advance while characters satisfy the predicate.
    pub fn eat_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(ch) = self.peek() {
            if pred(ch) {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    /// Advance if the upcoming bytes match the given string exactly.
    pub fn eat_str(&mut self, s: &str) -> bool {
        if self.source[self.pos..].starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    /// Check if upcoming bytes match the given string without advancing.
    pub fn check_str(&self, s: &str) -> bool {
        self.source[self.pos..].starts_with(s)
    }

    /// Get the source slice from `start` to current position.
    pub fn slice_from(&self, start: usize) -> &'src str {
        &self.source[start..self.pos]
    }

    /// Get the remaining source from the current position.
    pub fn remaining(&self) -> &'src str {
        &self.source[self.pos..]
    }
}
