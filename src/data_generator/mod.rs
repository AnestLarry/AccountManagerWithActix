#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate rand;

use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

mod string_letter {
    pub static ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    pub static ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub static ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub static NUMBER: &str = "0123456789";
    pub static PUNCTUATION_MARKS: &str = r##"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;
    pub static PRINTABLE: &str = r##"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;
}

#[derive(Clone)]
pub struct RandomGenerator {
    generator: ThreadRng,
}

impl RandomGenerator {
    fn get_num(&mut self) -> String {
        self.generator.gen_range(0..10).to_string()
    }
    fn get_lowercase(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::ASCII_LOWERCASE.len());
        string_letter::ASCII_LOWERCASE.get(temp..temp + 1).unwrap().into()
    }
    fn get_uppercase(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::ASCII_UPPERCASE.len());
        string_letter::ASCII_UPPERCASE.get(temp..temp + 1).unwrap().into()
    }
    fn get_letters(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::ASCII_LETTERS.len());
        string_letter::ASCII_LETTERS.get(temp..temp + 1).unwrap().into()
    }
    fn get_punctuation(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::PUNCTUATION_MARKS.len());
        string_letter::PUNCTUATION_MARKS.get(temp..temp + 1).unwrap().into()
    }
    fn get_all(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::PRINTABLE.len());
        string_letter::PRINTABLE.get(temp..temp + 1).unwrap().into()
    }
}

impl RandomGenerator {
    pub fn new() -> Self {
        let rg = rand::thread_rng();
        RandomGenerator {
            generator: rg
        }
    }
    pub fn get_account(&mut self) -> String {
        let length: usize = self.generator.gen_range(7..11);
        let mut result = Vec::with_capacity(11);
        result.push(self.get_letters());
        for _ in 0..length {
            if self.generator.gen_bool(0.5) {
                result.push(self.get_num());
            } else {
                result.push(self.get_letters());
            }
        }
        result.join("")
    }
    pub fn get_password_1(&mut self) -> String {
        let mut result = Vec::with_capacity(6);
        for _ in 0..6 {
            result.push(self.get_num())
        }
        result.join("")
    }
    pub fn get_password_2(&mut self) -> String {
        let length: usize = self.generator.gen_range(8..14);
        let mut result = Vec::with_capacity(14);
        for _ in 0..length {
            match self.generator.gen_range(0..3) {
                0 => result.push(self.get_num()),
                1 => result.push(self.get_lowercase()),
                2 => result.push(self.get_uppercase()),
                _ => {}
            }
        }
        result.join("")
    }
    pub fn get_password_3(&mut self) -> String {
        let length: usize = self.generator.gen_range(8..14);
        let mut result = Vec::with_capacity(14);
        for _ in 0..length {
            // let temp = ;
            match self.generator.gen_range(0..30) {
                0..=7 => result.push(self.get_num()),
                8..=15 => result.push(self.get_lowercase()),
                16..=23 => result.push(self.get_uppercase()),
                24..=29 => result.push(self.get_punctuation()),
                _ => {}
            }
        }
        result.join("")
    }
    pub fn get_password_max(&mut self) -> String {
        let funcs = [RandomGenerator::get_num, RandomGenerator::get_lowercase,
            RandomGenerator::get_uppercase, RandomGenerator::get_letters, RandomGenerator::get_punctuation,
            RandomGenerator::get_all];
        let mut result:Vec<String>=Vec::with_capacity(21);
        for f in funcs.iter()
        {
            for _ in 0..2 {
                result.push(f(self));
            }
        }
        for _ in 0..9 {
            if self.generator.gen_bool(0.5) {
                result.push(self.get_all());
            }
        }
        result.push(self.get_punctuation());
        result.push(self.get_punctuation());
        result.shuffle(&mut self.generator);
        result.join("")
    }
}
