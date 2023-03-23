use core::panic;
use crate::finite_field_element::gf256_element::GF256Element;

pub trait Crypter {
    fn encode(&self, message: &str) -> Vec<u8>;
    fn decode(&self, message: &[u8]) -> String;
}

pub struct AesService<'a> {
    key: &'a[u8]
}

impl <'a> Crypter for AesService<'a>{
    fn encode(&self, message: &str) -> Vec<u8> {
        let mut message_bytes = vec![0; message.as_bytes().len()];
        message_bytes.copy_from_slice(message.as_bytes());
        if message_bytes.len() % 16 != 0{
            panic!("Incorrect message size. Ensure 16 divides it's byte size")
        } 
        for i in 0..(message_bytes.len() / 16){
            for j in 0..16{
                for k in 0..16{
                    if message_bytes[i*16 + k] != 0{
                        message_bytes[i*16 + k] = GF256Element::from_byte(message_bytes[i*16 + k]).inverse().to_byte();
                    }
                }
                for k in 0..16{
                    message_bytes[i*16  + k] ^= self.key[j*16+k];
                }
                swap_rows(&mut message_bytes[i*16..(i+1)*16], 0, 3);
                swap_rows(&mut message_bytes[i*16..(i+1)*16], 1, 2);
                swap_columns(&mut message_bytes[i*16..(i+1)*16], 0, 1);
                swap_columns(&mut message_bytes[i*16..(i+1)*16], 2, 3);
            }
    }
    message_bytes
} 

    fn decode(&self, encoded_message: &[u8]) -> String {
        let mut message_copy = vec![0; encoded_message.len()];
        message_copy.copy_from_slice(encoded_message);
        for i in 0..(encoded_message.len() / 16){
            for j in 0..16{
                swap_columns(&mut message_copy[i*16..(i+1)*16], 0, 1);
                swap_columns(&mut message_copy[i*16..(i+1)*16], 2, 3);
                swap_rows(&mut message_copy[i*16..(i+1)*16], 0, 3);
                swap_rows(&mut message_copy[i*16..(i+1)*16],1 , 2);
                for k in 0..16 {
                    message_copy[i*16  + k] ^= self.key[16*(16-j-1) + k];
                }
                for k in 0..16{
                    if message_copy[i*16 + k] != 0{
                        message_copy[i*16 + k] = GF256Element::from_byte(message_copy[i*16 + k]).inverse().to_byte();
                    }
                }
            }
        }
        convert_to_string(message_copy)
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

fn convert_to_string(bytes_slice: Vec<u8>) -> String{
    match std::str::from_utf8(&bytes_slice) {
        Ok(s) => s.to_string(),
        Err(_) => panic!("Could not convert bytes slice to string. Found invalid utf8.")
    }
}