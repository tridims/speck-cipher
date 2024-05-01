// pub fn pad_pkcs7(message: &str, block_size: usize) -> String {
//     let padding_size = block_size - message.len() % block_size;
//     let padding_char = padding_size as u8 as char;
//     let padding: String = (0..padding_size).map(|_| padding_char).collect();
//     format!("{}{}", message, padding)
// }

pub fn pad_pkcs7(message: &[u8], block_size: usize) -> Vec<u8> {
    let padding_size = block_size - message.len() % block_size;
    let padding: Vec<u8> = vec![padding_size as u8; padding_size];
    [message, &padding].concat()
}

pub fn unpad_pkcs7(message: &[u8]) -> Vec<u8> {
    let padding_size = message[message.len() - 1] as usize;
    message[..message.len() - padding_size].to_vec()
}
