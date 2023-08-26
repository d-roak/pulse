

pub fn xor_distance(id1: &[u8; 20], id2: &[u8; 20]) -> [u8; 20] {
    let mut result = [0u8; 20];
    for i in 0..20 {
        result[i] = id1[i] ^ id2[i];
    }
    result
}
