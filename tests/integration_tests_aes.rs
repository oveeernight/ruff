use ruff::aes_service::{AesService, Crypter};

const KEY : [u8; 256] = 
[1u8, 94u8, 60u8, 112u8, 97u8, 153u8, 84u8, 44u8, 48u8, 97u8, 143u8, 76u8, 133u8, 122u8, 61u8, 148u8, 
73u8, 141u8, 220u8, 37u8, 118u8, 94u8, 63u8, 182u8, 4u8, 147u8, 23u8, 27u8, 131u8, 233u8, 40u8, 81u8, 
148u8, 9u8, 157u8, 206u8, 35u8, 253u8, 114u8, 58u8, 205u8, 158u8, 198u8, 100u8, 171u8, 129u8, 247u8, 242u8, 
53u8, 14u8, 73u8, 134u8, 107u8, 150u8, 118u8, 246u8, 24u8, 9u8, 78u8, 93u8, 29u8, 15u8, 111u8, 244u8, 
210u8, 163u8, 50u8, 55u8, 136u8, 40u8, 150u8, 153u8, 97u8, 58u8, 9u8, 241u8, 167u8, 244u8, 118u8, 215u8, 
128u8, 72u8, 186u8, 46u8, 85u8, 167u8, 127u8, 235u8, 218u8, 4u8, 76u8, 121u8, 255u8, 128u8, 42u8, 161u8, 
31u8, 196u8, 174u8, 126u8, 110u8, 162u8, 222u8, 118u8, 72u8, 231u8, 167u8, 163u8, 125u8, 42u8, 61u8, 73u8, 
100u8, 130u8, 97u8, 138u8, 214u8, 5u8, 54u8, 248u8, 161u8, 57u8, 148u8, 37u8, 211u8, 146u8, 9u8, 72u8, 
146u8, 95u8, 233u8, 235u8, 152u8, 101u8, 239u8, 187u8, 123u8, 206u8, 89u8, 166u8, 96u8, 26u8, 56u8, 185u8, 
164u8, 46u8, 211u8, 67u8, 18u8, 167u8, 158u8, 154u8, 74u8, 96u8, 169u8, 73u8, 168u8, 14u8, 234u8, 187u8, 
134u8, 171u8, 134u8, 167u8, 142u8, 153u8, 98u8, 31u8, 150u8, 68u8, 207u8, 249u8, 45u8, 13u8, 174u8, 71u8, 
13u8, 142u8, 93u8, 230u8, 182u8, 251u8, 100u8, 154u8, 55u8, 145u8, 36u8, 141u8, 206u8, 113u8, 9u8, 75u8, 
100u8, 67u8, 188u8, 125u8, 125u8, 236u8, 189u8, 74u8, 89u8, 188u8, 14u8, 135u8, 214u8, 84u8, 165u8, 227u8, 
36u8, 174u8, 210u8, 127u8, 18u8, 112u8, 19u8, 20u8, 105u8, 211u8, 172u8, 77u8, 124u8, 35u8, 47u8, 64u8, 
21u8, 45u8, 60u8, 24u8, 121u8, 110u8, 198u8, 120u8, 145u8, 162u8, 75u8, 133u8, 172u8, 185u8, 214u8, 14u8, 
184u8, 19u8, 111u8, 15u8, 76u8, 26u8, 206u8, 0u8, 216u8, 9u8, 49u8, 209u8, 152u8, 175u8, 95u8, 228u8, 
];

#[test]
fn message_16bytes_equals_to_decoded_message(){
    let aes_service = AesService::new(&KEY);
    let message = "abcdefghtyrfghpj"; 
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
    assert_eq!(message, decrypted_message)
}

#[test]
fn message_16bytes_russian_literals_equals_to_decoded_message(){
    let aes_service = AesService::new(&KEY);
    let message = "АБВГДЕёж"; 
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
    assert_eq!(message, decrypted_message)
}

#[test]
fn message_32bytes_russian_en_literals_equals_to_decoded_message(){
    let aes_service = AesService::new(&KEY);
    let message = "АБВГДЕёжabcdefghtyrfghpj"; 
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
    assert_eq!(message, decrypted_message)
}

#[test]
fn message_112bytes_russian_en_literals_equals_to_decoded_message(){
    let aes_service = AesService::new(&KEY);
    let message = "АБВГДЕёжabcdefghtyrfghpjAbcDfGtErUioPqgn89145nBghPqZxfdQАБВешу12uioplk,456TYRGHPLK9o7b5f8h5d0h5gа"; 
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
    assert_eq!(message, decrypted_message)
}

#[test]
fn message_16bytes_bonus_equals_to_decoded_message(){
    let aes_service = AesService::new(&KEY);
    let message = "i hate rust af!!"; 
    let encrypted_message = aes_service.encode(message);
    let decrypted_message = aes_service.decode(&encrypted_message);
    assert_eq!(message, decrypted_message)
}



