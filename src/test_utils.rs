use std::fs::File;
use std::io::Write;

use rand::Rng;

pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x ^ y).count_ones() as usize)
        .sum()
}

pub fn flip_key_iv(key: [u8; 32], iv: [u8; 16]) -> ([u8; 32], [u8; 16]) {
    let mut rng = rand::thread_rng();

    let mut key2 = key;
    let mut iv2 = iv;

    // Flip a random bit in key2
    let random_byte_index = rng.gen_range(0..32);
    let random_bit_index = rng.gen_range(0..8);
    key2[random_byte_index] ^= 1 << random_bit_index;

    // Flip a random bit in iv2
    let random_byte_index = rng.gen_range(0..16);
    let random_bit_index = rng.gen_range(0..8);
    iv2[random_byte_index] ^= 1 << random_bit_index;

    (key2, iv2)
}

pub fn export_to_file(data: &[f64], filename: &str) {
    let mut file = File::create(filename).expect("Failed to create file");
    for &value in data {
        writeln!(file, "{}", value).expect("Failed to write to file");
    }
}

pub fn perform_uniformity_analysis(ciphertext: &[u8], bin_size: usize) -> f64 {
    let mut observed_frequencies = vec![0; bin_size];

    for &byte in ciphertext {
        observed_frequencies[byte as usize] += 1;
    }

    let expected_frequency = ciphertext.len() as f64 / bin_size as f64;
    let chi_square: f64 = observed_frequencies
        .iter()
        .map(|&observed| {
            let diff = observed as f64 - expected_frequency;
            diff * diff / expected_frequency
        })
        .sum();

    chi_square
}

pub const MNEMONIC_DATA: [&str; 30] = [
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    "legal winner thank year wave sausage worth useful legal winner thank yellow",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
    "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
    "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
    "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
    "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
    "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
    "scheme spot photo card baby mountain device kick cradle pact join borrow",
    "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
    "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
    "cat swing flag economy stadium alone churn speed unique patch report train",
    "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
    "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
    "vessel ladder alter error federal sibling chat ability sun glass valve picture",
    "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
    "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
    "abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace agrese",
    "abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abeille",
    "abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco abete",
    "あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あおぞら",
    "가격 가격 가격 가격 가격 가격 가격 가격 가격 가격 가격 가능",
    "abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate abater"
];

pub const MNEMONIC_DATA_MODIFIED_BACK: [&str; 30] = [
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon legal",
    "legal winner thank year wave sausage worth useful legal winner thank letter",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage zoo",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon",
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon legal",
    "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal letter",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter zoo",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon",
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon legal",
    "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth letter",
    "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic zoo",
    "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo ozone",
    "ozone drill grab fiber curtain grace pudding thank cruise elder eight machine",
    "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow hamster",
    "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay scheme",
    "scheme spot photo card baby mountain device kick cradle pact join horn",
    "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack panda",
    "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost cat",
    "cat swing flag economy stadium alone churn speed unique patch report light",
    "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud all",
    "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt vessel",
    "vessel ladder alter error federal sibling chat ability sun glass valve scissors",
    "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress void",
    "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve abdikace",
    "abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abaisser",
    "abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaco",
    "abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco abacate",
    "あおぞら　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん",
    "가격 가격 가격 가격 가격 가격 가격 가격 가격 가격 가격 가능 くしん",
    "abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate abaisser",
];

pub const MNEMONIC_DATA_MODIFIED_FRONT: [&str; 30] = [
    "legal abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon",
    "letter legal winner thank year wave sausage worth useful legal winner thank",
    "zoo letter advice cage absurd amount doctor acoustic avoid letter advice cage",
    "abandon zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo",
    "legal abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon",
    "letter legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal",
    "zoo letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter",
    "abandon zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo",
    "legal abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon",
    "letter legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth",
    "zoo letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic",
    "ozone zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo",
    "machine ozone drill grab fiber curtain grace pudding thank cruise elder eight",
    "hamster gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow",
    "scheme hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay",
    "horn scheme spot photo card baby mountain device kick cradle pact join",
    "panda horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack",
    "cat panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost",
    "light cat swing flag economy stadium alone churn speed unique patch report",
    "all light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud",
    "vessel all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt",
    "scissors vessel ladder alter error federal sibling chat ability sun glass valve",
    "void scissors invite lock maple supreme raw rapid congress muscle digital elegant little brisk hair mango congress",
    "abdikace void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve",
    "abaisser abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace abdikace",
    "abaco abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser abaisser",
    "abacate abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco abaco",
    "あいこくしん　あおぞら　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん",
    "くしん 가격 가격 가격 가격 가격 가격 가격 가격 가격 가격 가능",
    "abaisser abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate abacate",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let a = [0b10101010, 0b11110000];
        let b = [0b01010101, 0b00001111];
        assert_eq!(hamming_distance(&a, &b), 16);
    }
}
