//! Base conversion operations sanity tests

#[cfg(test)]
mod tests {
    use rsb::math::base::*;

    #[test]
    fn test_to_hex() {
        assert_eq!(to_hex(255), "ff");
        assert_eq!(to_hex(16), "10");
        assert_eq!(to_hex(0), "0");
        assert_eq!(to_hex(15), "f");

        assert_eq!(to_hex_upper(255), "FF");
        assert_eq!(to_hex_upper(16), "10");
        assert_eq!(to_hex_upper(15), "F");
    }

    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary(8), "1000");
        assert_eq!(to_binary(15), "1111");
        assert_eq!(to_binary(0), "0");
        assert_eq!(to_binary(1), "1");
        assert_eq!(to_binary(255), "11111111");
    }

    #[test]
    fn test_to_octal() {
        assert_eq!(to_octal(8), "10");
        assert_eq!(to_octal(64), "100");
        assert_eq!(to_octal(0), "0");
        assert_eq!(to_octal(7), "7");
        assert_eq!(to_octal(255), "377");
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(from_hex("ff").unwrap(), 255);
        assert_eq!(from_hex("FF").unwrap(), 255);
        assert_eq!(from_hex("0").unwrap(), 0);
        assert_eq!(from_hex("10").unwrap(), 16);

        // With prefixes
        assert_eq!(from_hex("0xff").unwrap(), 255);
        assert_eq!(from_hex("0xFF").unwrap(), 255);

        // Error cases
        assert!(from_hex("gg").is_err());
        assert!(from_hex("").is_err());
    }

    #[test]
    fn test_from_binary() {
        assert_eq!(from_binary("1000").unwrap(), 8);
        assert_eq!(from_binary("1111").unwrap(), 15);
        assert_eq!(from_binary("0").unwrap(), 0);
        assert_eq!(from_binary("11111111").unwrap(), 255);

        // With prefixes
        assert_eq!(from_binary("0b1000").unwrap(), 8);
        assert_eq!(from_binary("0B1111").unwrap(), 15);

        // Error cases
        assert!(from_binary("102").is_err()); // invalid binary digit
        assert!(from_binary("").is_err());
    }

    #[test]
    fn test_from_octal() {
        assert_eq!(from_octal("10").unwrap(), 8);
        assert_eq!(from_octal("100").unwrap(), 64);
        assert_eq!(from_octal("0").unwrap(), 0);
        assert_eq!(from_octal("377").unwrap(), 255);

        // With prefixes
        assert_eq!(from_octal("0o10").unwrap(), 8);
        assert_eq!(from_octal("0O100").unwrap(), 64);

        // Error cases
        assert!(from_octal("8").is_err()); // invalid octal digit
        assert!(from_octal("").is_err());
    }

    #[test]
    fn test_to_base() {
        // Base 2 (binary)
        assert_eq!(to_base(8, 2).unwrap(), "1000");
        assert_eq!(to_base(0, 2).unwrap(), "0");

        // Base 8 (octal)
        assert_eq!(to_base(64, 8).unwrap(), "100");

        // Base 16 (hex)
        assert_eq!(to_base(255, 16).unwrap(), "ff");

        // Base 36 (max)
        assert_eq!(to_base(35, 36).unwrap(), "z");
        assert_eq!(to_base(36, 36).unwrap(), "10");

        // Negative numbers
        assert_eq!(to_base(-8, 2).unwrap(), "-1000");

        // Error cases
        assert!(to_base(10, 1).is_err()); // base too small
        assert!(to_base(10, 37).is_err()); // base too large
    }

    #[test]
    fn test_from_base() {
        // Base 2
        assert_eq!(from_base("1000", 2).unwrap(), 8);
        assert_eq!(from_base("1111", 2).unwrap(), 15);

        // Base 8
        assert_eq!(from_base("100", 8).unwrap(), 64);

        // Base 16
        assert_eq!(from_base("ff", 16).unwrap(), 255);
        assert_eq!(from_base("FF", 16).unwrap(), 255);

        // Base 36
        assert_eq!(from_base("z", 36).unwrap(), 35);
        assert_eq!(from_base("10", 36).unwrap(), 36);

        // Error cases
        assert!(from_base("10", 1).is_err()); // base too small
        assert!(from_base("10", 37).is_err()); // base too large
        assert!(from_base("2", 2).is_err()); // invalid digit for base
    }

    #[test]
    fn test_base_convert() {
        // Binary to hex
        assert_eq!(base_convert("1111", 2, 16).unwrap(), "f");

        // Hex to binary
        assert_eq!(base_convert("ff", 16, 2).unwrap(), "11111111");

        // Decimal to octal
        assert_eq!(base_convert("64", 10, 8).unwrap(), "100");

        // Octal to decimal
        assert_eq!(base_convert("100", 8, 10).unwrap(), "64");

        // Same base (identity)
        assert_eq!(base_convert("123", 10, 10).unwrap(), "123");

        // Error propagation
        assert!(base_convert("invalid", 2, 10).is_err());
        assert!(base_convert("10", 1, 10).is_err());
    }

    #[test]
    fn test_round_trip_conversions() {
        let test_values = vec![0, 1, 15, 16, 255, 256, 1023, 1024];

        for &value in &test_values {
            // Decimal -> Hex -> Decimal
            let hex = to_hex(value);
            assert_eq!(from_hex(&hex).unwrap(), value);

            // Decimal -> Binary -> Decimal
            let binary = to_binary(value);
            assert_eq!(from_binary(&binary).unwrap(), value);

            // Decimal -> Octal -> Decimal
            let octal = to_octal(value);
            assert_eq!(from_octal(&octal).unwrap(), value);
        }
    }
}
