pub fn xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1.iter().zip(b2.iter()).map(|(a, b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_to_bytes::hex_to_bytes;

    #[test]
    fn it_works() {
        let inp1 = "1c0111001f010100061a024b53535009181c";
        let inp2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let result = xor(&hex_to_bytes(inp1), &hex_to_bytes(inp2));
        assert_eq!(hex_to_bytes(expected), result);

        for c in result.iter().map(|c| format!("{:x}", c)) {
            print!("{}", c);
        }
        print!("\n");
    }
}
