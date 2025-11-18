use num_bigint::BigUint;

pub fn encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let leading_zeros = bytes.iter().rev().take_while(|&&b| b == 0).count();

    if leading_zeros == bytes.len() {
        "0".repeat(bytes.len())
    } else {
        let num = BigUint::from_bytes_le(bytes);
        let mut result = num.to_string();
        if leading_zeros > 0 {
            result.push_str(&"0".repeat(leading_zeros));
        }
        result
    }
}

pub fn decode(s: &str) -> Vec<u8> {
    if s.is_empty() {
        return Vec::new();
    }

    let trailing_zeros = s.chars().rev().take_while(|&c| c == '0').count();

    if trailing_zeros == s.len() {
        vec![0u8; s.len()]
    } else {
        let s_trimmed = &s[..s.len() - trailing_zeros];
        let num = s_trimmed.parse::<BigUint>().expect("Invalid base10 string");
        let mut bytes = num.to_bytes_le();
        bytes.extend(vec![0u8; trailing_zeros]);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(encode(&[]), "");
        assert_eq!(decode(""), Vec::<u8>::new());
    }

    #[test]
    fn test_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_with_leading_zeros() {
        let data = vec![1, 2, 3, 0, 0];
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_all_zeros() {
        let data = vec![0, 0, 0];
        let encoded = encode(&data);
        println!("Encoded '000' as: '{}'", encoded);
        let decoded = decode(&encoded);
        println!("Decoded as: {:?}", decoded);
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_large_data() {
        let data = vec![255; 100];
        let encoded = encode(&data);
        let decoded = decode(&encoded);
        assert_eq!(data, decoded);
    }
}
