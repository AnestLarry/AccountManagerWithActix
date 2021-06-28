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
        string_letter::ASCII_LOWERCASE.chars().nth(temp).unwrap().to_string()
    }
    fn get_uppercase(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::ASCII_UPPERCASE.len());
        string_letter::ASCII_UPPERCASE.chars().nth(temp).unwrap().to_string()
    }
    fn get_letters(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::ASCII_LETTERS.len());
        string_letter::ASCII_LETTERS.chars().nth(temp).unwrap().to_string()
    }
    fn get_punctuation(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::PUNCTUATION_MARKS.len());
        string_letter::PUNCTUATION_MARKS.chars().nth(temp).unwrap().to_string()
    }
    fn get_all(&mut self) -> String {
        let temp = self.generator.gen_range(0..string_letter::PRINTABLE.len());
        string_letter::PRINTABLE.chars().nth(temp).unwrap().to_string()
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
        let mut result = vec![self.get_letters()];
        for _ in 0..length {
            let temp = self.generator.gen_range(0..2);
            if temp == 0 {
                result.push(self.get_num());
            } else {
                result.push(self.get_letters());
            }
        }
        result.join("")
    }
    pub fn get_password_1(&mut self) -> String {
        let mut result = String::new();
        for _ in 0..6 {
            result.push_str(&self.get_num())
        }
        result
    }
    pub fn get_password_2(&mut self) -> String {
        let length: usize = self.generator.gen_range(8..14);
        let mut result = String::new();
        for _ in 0..length {
            let temp = self.generator.gen_range(0..3);
            match temp {
                0 => result.push_str(&self.get_num()),
                1 => result.push_str(&self.get_lowercase()),
                2 => result.push_str(&self.get_uppercase()),
                _ => {}
            }
        }
        result
    }
    pub fn get_password_3(&mut self) -> String {
        let length: usize = self.generator.gen_range(8..14);
        let mut result = String::new();
        for _ in 0..length {
            let temp = self.generator.gen_range(0..30);
            match temp {
                0..=7 => result.push_str(&self.get_num()),
                8..=15 => result.push_str(&self.get_lowercase()),
                16..=23 => result.push_str(&self.get_uppercase()),
                24..=29 => result.push_str(&self.get_punctuation()),
                _ => {}
            }
        }
        result
    }
    pub fn get_password_max(&mut self) -> String {
        let funcs = [RandomGenerator::get_num, RandomGenerator::get_lowercase,
            RandomGenerator::get_uppercase, RandomGenerator::get_letters, RandomGenerator::get_punctuation,
            RandomGenerator::get_all];
        let mut result = String::new();
        for f in funcs.iter()
        {
            for _ in 0..2 {
                result.push_str(&f(self));
            }
        }
        for _ in 0..6 {
            if self.generator.gen_range(0..2) == 1 {
                result.push_str(&self.get_all());
            }
        }
        {
            result.push_str(&*self.get_punctuation());
            result.push_str(&*self.get_punctuation());
            let mut t = result.as_bytes().to_owned();
            t.shuffle(&mut self.generator);
            result = String::from_utf8(t.to_vec()).unwrap();
        }
        result
    }
}
