// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

use crate::scanner::Scanner;
use crate::token::TokenKind;

/// Scan a numeric literal starting at the current position.
/// The scanner must be positioned at the first digit (or '0' for prefixed literals).
/// Returns the token kind (IntLiteral or FloatLiteral).
pub fn scan_number(scanner: &mut Scanner) -> TokenKind {
    let first = scanner.peek().unwrap();

    if first == '0' {
        scanner.advance();
        match scanner.peek() {
            Some('x' | 'X') => {
                scanner.advance();
                eat_hex_digits(scanner);
                return TokenKind::IntLiteral;
            }
            Some('o' | 'O') => {
                scanner.advance();
                eat_octal_digits(scanner);
                return TokenKind::IntLiteral;
            }
            Some('b' | 'B') => {
                scanner.advance();
                eat_binary_digits(scanner);
                return TokenKind::IntLiteral;
            }
            _ => {
                // Could be 0, 0.5, 0e10, etc.
            }
        }
    } else {
        eat_decimal_digits(scanner);
    }

    // Check for float: decimal point or exponent
    let mut is_float = false;

    if scanner.peek() == Some('.') {
        // Make sure it's not `..` (range) or `.method()`
        if scanner.peek_nth(1) != Some('.') && !is_ident_start(scanner.peek_nth(1)) {
            scanner.advance(); // eat '.'
            eat_decimal_digits(scanner);
            is_float = true;
        }
    }

    if matches!(scanner.peek(), Some('e' | 'E')) {
        scanner.advance();
        scanner.eat_if(|c| c == '+' || c == '-');
        eat_decimal_digits(scanner);
        is_float = true;
    }

    // Optional long suffix for integers
    if !is_float {
        scanner.eat_if(|c| c == 'l' || c == 'L');
    }

    if is_float {
        TokenKind::FloatLiteral
    } else {
        TokenKind::IntLiteral
    }
}

fn eat_decimal_digits(scanner: &mut Scanner) {
    scanner.eat_while(|c| c.is_ascii_digit() || c == '_');
}

fn eat_hex_digits(scanner: &mut Scanner) {
    scanner.eat_while(|c| c.is_ascii_hexdigit() || c == '_');
}

fn eat_octal_digits(scanner: &mut Scanner) {
    scanner.eat_while(|c| matches!(c, '0'..='7' | '_'));
}

fn eat_binary_digits(scanner: &mut Scanner) {
    scanner.eat_while(|c| c == '0' || c == '1' || c == '_');
}

fn is_ident_start(ch: Option<char>) -> bool {
    matches!(ch, Some(c) if c.is_alphabetic() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(input: &str) -> (TokenKind, usize) {
        let mut scanner = Scanner::new(input);
        let kind = scan_number(&mut scanner);
        (kind, scanner.pos())
    }

    #[test]
    fn test_integer() {
        assert_eq!(scan("42"), (TokenKind::IntLiteral, 2));
        assert_eq!(scan("0"), (TokenKind::IntLiteral, 1));
        assert_eq!(scan("1_000_000"), (TokenKind::IntLiteral, 9));
    }

    #[test]
    fn test_hex() {
        assert_eq!(scan("0xFF"), (TokenKind::IntLiteral, 4));
        assert_eq!(scan("0x1A_2B"), (TokenKind::IntLiteral, 7));
    }

    #[test]
    fn test_octal() {
        assert_eq!(scan("0o77"), (TokenKind::IntLiteral, 4));
    }

    #[test]
    fn test_binary() {
        assert_eq!(scan("0b1010"), (TokenKind::IntLiteral, 6));
    }

    #[test]
    fn test_float() {
        assert_eq!(scan("3.14"), (TokenKind::FloatLiteral, 4));
        assert_eq!(scan("1e10"), (TokenKind::FloatLiteral, 4));
        assert_eq!(scan("2.5e-3"), (TokenKind::FloatLiteral, 6));
        assert_eq!(scan("0.5"), (TokenKind::FloatLiteral, 3));
    }

    #[test]
    fn test_long_suffix() {
        assert_eq!(scan("42L"), (TokenKind::IntLiteral, 3));
        assert_eq!(scan("100l"), (TokenKind::IntLiteral, 4));
    }
}
