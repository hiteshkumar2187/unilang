// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

use crate::scanner::Scanner;
use crate::token::TokenKind;

/// The quote style used for a string literal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuoteStyle {
    Single,
    Double,
}

/// Result from scanning a string — may be a regular string or the start of an f-string.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StringScanResult {
    /// A complete string literal.
    Complete(TokenKind),
    /// An unterminated string (error).
    Unterminated,
}

/// Scan a string literal. The scanner should be positioned at the opening quote character.
/// Handles single-quoted, double-quoted, and triple-quoted strings.
pub fn scan_string(scanner: &mut Scanner, quote: char) -> StringScanResult {
    // Check for triple-quoted string
    if scanner.peek_nth(0) == Some(quote) && scanner.peek_nth(1) == Some(quote) {
        return scan_triple_string(scanner, quote);
    }

    // Single-line string: advance past the opening quote (already consumed by caller).
    // Scan until matching closing quote or end of line.
    loop {
        match scanner.peek() {
            None | Some('\n') | Some('\r') => {
                return StringScanResult::Unterminated;
            }
            Some('\\') => {
                // Escape sequence — skip the backslash and the next character.
                scanner.advance();
                scanner.advance();
            }
            Some(c) if c == quote => {
                scanner.advance(); // consume closing quote
                return StringScanResult::Complete(TokenKind::StringLiteral);
            }
            _ => {
                scanner.advance();
            }
        }
    }
}

/// Scan a triple-quoted string (""" or ''').
/// The scanner is positioned at the second quote character (first already consumed by caller).
fn scan_triple_string(scanner: &mut Scanner, quote: char) -> StringScanResult {
    // Consume the remaining two quotes of the opening triple
    scanner.advance(); // second quote
    scanner.advance(); // third quote

    let mut consecutive_quotes = 0;

    loop {
        match scanner.peek() {
            None => return StringScanResult::Unterminated,
            Some('\\') => {
                scanner.advance();
                scanner.advance();
                consecutive_quotes = 0;
            }
            Some(c) if c == quote => {
                scanner.advance();
                consecutive_quotes += 1;
                if consecutive_quotes >= 3 {
                    return StringScanResult::Complete(TokenKind::TripleStringLiteral);
                }
            }
            _ => {
                scanner.advance();
                consecutive_quotes = 0;
            }
        }
    }
}

/// Scan an f-string. The 'f' prefix has already been consumed.
/// The scanner is positioned at the opening quote character.
/// Returns a list of token kinds and their byte ranges within the f-string.
///
/// For the initial implementation, we treat the entire f-string as a single
/// StringLiteral token. A more advanced implementation would emit
/// FStringStart / FStringMiddle / FStringEnd tokens with interpolated expressions.
pub fn scan_fstring(scanner: &mut Scanner, quote: char) -> StringScanResult {
    // For now, treat f-strings like regular strings.
    // The parser will handle the interpolation semantics.
    // This skips `{...}` content but doesn't lex the expressions inside.
    let mut brace_depth = 0;

    loop {
        match scanner.peek() {
            None | Some('\n') | Some('\r') => {
                return StringScanResult::Unterminated;
            }
            Some('\\') => {
                scanner.advance();
                scanner.advance();
            }
            Some('{') => {
                scanner.advance();
                if scanner.peek() == Some('{') {
                    scanner.advance(); // escaped {{ → literal {
                } else {
                    brace_depth += 1;
                    // Scan until matching }
                    while brace_depth > 0 {
                        match scanner.peek() {
                            None | Some('\n') | Some('\r') => {
                                return StringScanResult::Unterminated;
                            }
                            Some('{') => {
                                scanner.advance();
                                brace_depth += 1;
                            }
                            Some('}') => {
                                scanner.advance();
                                brace_depth -= 1;
                            }
                            Some('\\') => {
                                scanner.advance();
                                scanner.advance();
                            }
                            Some(q) if q == quote => {
                                // A quote inside the interpolation — part of a nested string
                                scanner.advance();
                            }
                            _ => {
                                scanner.advance();
                            }
                        }
                    }
                }
            }
            Some('}') => {
                scanner.advance();
                if scanner.peek() == Some('}') {
                    scanner.advance(); // escaped }} → literal }
                }
            }
            Some(c) if c == quote => {
                scanner.advance();
                return StringScanResult::Complete(TokenKind::StringLiteral);
            }
            _ => {
                scanner.advance();
            }
        }
    }
}

/// Scan a raw string (r"..." or r'...'). The 'r' prefix has been consumed.
/// Scanner is positioned at the opening quote.
pub fn scan_raw_string(scanner: &mut Scanner, quote: char) -> StringScanResult {
    loop {
        match scanner.peek() {
            None | Some('\n') | Some('\r') => {
                return StringScanResult::Unterminated;
            }
            Some(c) if c == quote => {
                scanner.advance();
                return StringScanResult::Complete(TokenKind::StringLiteral);
            }
            _ => {
                scanner.advance();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_double_string() {
        let mut scanner = Scanner::new("hello world\"rest");
        let result = scan_string(&mut scanner, '"');
        assert_eq!(result, StringScanResult::Complete(TokenKind::StringLiteral));
        assert_eq!(scanner.pos(), 12); // up to and including the closing quote
    }

    #[test]
    fn test_escaped_quote() {
        let mut scanner = Scanner::new(r#"hello \"world"rest"#);
        let result = scan_string(&mut scanner, '"');
        assert_eq!(result, StringScanResult::Complete(TokenKind::StringLiteral));
    }

    #[test]
    fn test_unterminated() {
        let mut scanner = Scanner::new("hello world\n");
        let result = scan_string(&mut scanner, '"');
        assert_eq!(result, StringScanResult::Unterminated);
    }

    #[test]
    fn test_triple_string() {
        // Scanner starts at the second quote (first consumed by caller)
        let mut scanner = Scanner::new("\"\"hello\nworld\"\"\"rest");
        let result = scan_string(&mut scanner, '"');
        assert_eq!(
            result,
            StringScanResult::Complete(TokenKind::TripleStringLiteral)
        );
    }

    #[test]
    fn test_fstring() {
        let mut scanner = Scanner::new("hello {name}!\"rest");
        let result = scan_fstring(&mut scanner, '"');
        assert_eq!(result, StringScanResult::Complete(TokenKind::StringLiteral));
    }
}
