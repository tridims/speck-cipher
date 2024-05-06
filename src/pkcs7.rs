pub fn pad_pkcs7(message: &[u8], block_size: usize) -> Vec<u8> {
    let padding_size = block_size - message.len() % block_size;
    let padding: Vec<u8> = vec![padding_size as u8; padding_size];
    [message, &padding].concat()
}

pub fn unpad_pkcs7(message: &[u8]) -> Vec<u8> {
    let padding_size = message[message.len() - 1] as usize;
    message[..message.len() - padding_size].to_vec()
}
