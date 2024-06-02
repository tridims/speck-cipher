use rand::Rng;
use speck_cipher::speck_cbc_encrypt;
use std::fs::File;
use std::io::Write;

const MNEMONIC_DATA: [&str; 30] = [
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

const MNEMONIC_MODIFIED_DATA: [&str; 30] = [
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

fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x ^ y).count_ones() as usize)
        .sum()
}

fn benchmark_avalanche_effect_on_data(test_data: &[&str], modified_data: &[&str]) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut avalanche_effects = vec![0.0; test_data.len()];

    for (i, (orig, modif)) in test_data.iter().zip(modified_data.iter()).enumerate() {
        let plaintext = orig.as_bytes();
        let modified_plaintext = modif.as_bytes();
        let key: [u8; 32] = rng.gen();
        let iv: [u8; 16] = rng.gen();

        let ciphertext = speck_cbc_encrypt(&key, &iv, plaintext);
        let modified_ciphertext = speck_cbc_encrypt(&key, &iv, modified_plaintext);
        let distance = hamming_distance(&ciphertext, &modified_ciphertext);
        avalanche_effects[i] = distance as f64 / (ciphertext.len() * 8) as f64;
    }

    avalanche_effects
}

fn main() {
    let avalanche_effects_on_data =
        benchmark_avalanche_effect_on_data(&MNEMONIC_DATA, &MNEMONIC_MODIFIED_DATA);

    let mut data_file_data = File::create("data/avalanche2/avalanche_2.txt").unwrap();

    for (index, effect) in avalanche_effects_on_data.iter().enumerate() {
        writeln!(data_file_data, "{} {}", index, effect).unwrap();
    }

    println!("Avalanche effect data has been written to data/avalanche2/avalanche_2.txt");
}
