use core::panic;
use crate::finite_field_element::gf256_element::GF256Element;

pub trait Crypter {
    /// Encodes message and returns byte slice.
    fn encode(&self, message: &str) -> Vec<u8>;
    /// Decodes bytes slice to string.
    fn decode(&self, message: &[u8]) -> String;
}

/// Represents service that performs one of the stages of the AES algorithm
pub struct AesService<'a>{
    key: &'a[u8]
}


impl <'a> Crypter for AesService<'a>{
    // encode returns bytes slice rather than string due to algorithm often converts string to invalid utf8 string
    fn encode(&self, message: &str) -> Vec<u8> {
        let message_bytes =  message.as_bytes();
        let mut result : Vec<u8> = Vec::new();
        if message_bytes.len() % 16 != 0{
            panic!("Incorrect message size. Ensure 16 divides it's byte size")
        } 
        for i in 0..(message_bytes.len() / 16){
            let mut message_part = copy_message_part(&message_bytes[i*16..(i+1)*16]);
            for j in 0..16{
                let  key_part = &self.key[j*16..(j+1)*16];
                let reverse_bytes = get_reverse_bytes(&message_part);
                let mut xor_res : Vec<u8> = reverse_bytes.iter().zip(key_part).map(|(x,y)| x ^ y).collect();
                swap_rows(&mut xor_res, 0, 3);
                swap_rows(&mut xor_res, 1, 2);
                swap_columns(&mut xor_res, 0, 1);
                swap_columns(&mut xor_res, 2, 3);
                message_part = xor_res;
            }

            for item in message_part.iter().take(16){
                result.push(*item)
            }
        }

        result
    }

    

    fn decode(&self, encoded_message: &[u8]) -> String {
        let mut result = String::new();
        for i in 0..(encoded_message.len() / 16){
            let encoded_message_part = &encoded_message[16*i..16 * (i+1)];
            let mut message_part_copy = copy_message_part(encoded_message_part);
            for j in 0..16{
                let  key_part = &self.key[(16-j-1)*16..(16-j)*16];
                swap_columns(&mut message_part_copy, 0, 1);
                swap_columns(&mut message_part_copy, 2, 3);
                swap_rows(&mut message_part_copy, 0, 3);
                swap_rows(&mut message_part_copy,1 , 2);
                message_part_copy = message_part_copy.iter().zip(key_part).map(|(x, y)| x ^ y).collect();
                let reverse_bytes = get_reverse_bytes(&message_part_copy);
                message_part_copy = reverse_bytes
            }

            result.push_str(&convert_to_string(message_part_copy));
        }
        result
    }
}

impl <'a> AesService<'a>{
    pub fn new(key: &[u8]) -> AesService{
        if key.len() != 256{
            panic!("Key size must be equal to 256.")
        }
        AesService { key }
    }
}

fn get_reverse_bytes(message_part: &[u8]) -> Vec<u8>{
    let mut result = Vec::with_capacity(message_part.len());
    for i in 0..message_part.len(){
        result.push(0);
        if message_part[i] != 0{
            result[i] = GF256Element::from_byte(message_part[i]).inverse().to_byte()
        }
    }
    result
}

fn swap_rows(array: &mut[u8], row1: usize, row2: usize){
    for i in 0..4{
        array.swap(4 * row1 + i, 4 * row2 + i);
    }
}

fn swap_columns(array: &mut[u8], column1: usize, column2: usize){
    for i in 0..4{
        array.swap(4 * i + column1, 4 * i + column2);
    }
}

fn copy_message_part(message_part: &[u8]) -> Vec<u8>{
    let mut result = Vec::with_capacity(message_part.len());
    for i in 0.. message_part.len(){
        result.push(0);
        result[i] = message_part[i];
    }
    result
}

fn convert_to_string(bytes_slice: Vec<u8>) -> String{
    match std::str::from_utf8(&bytes_slice) {
        Ok(s) => s.to_string(),
        Err(_) => panic!("Could not convert bytes slice to string. Found invalid utf8.")
    }
}
