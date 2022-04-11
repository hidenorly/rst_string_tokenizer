/*
  Copyright (C) 2022 hidenorly

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

trait IStringTokenizer
{
    fn new( target : &str, token : &str ) -> Self;
    fn has_next(&self) -> bool { false }
    fn get_next(&mut self) -> String { String::from("") }
    fn get_next_with_trim(&mut self) -> String { String::from("") }
}


pub struct StringTokenizer
{
    target : String,
    token : String,
    last_pos : usize,
    target_len : usize,
    token_len : usize,
}


impl IStringTokenizer for StringTokenizer
{
    fn new( target : &str, token : &str ) -> Self {
        Self {
            target : target.to_string(),
            token : token.to_string(),
            last_pos : 0,
            target_len : target.len(),
            token_len : token.len(),
         }
    }

    fn has_next(&self) -> bool {
        if self.last_pos < self.target_len {
            true
        } else {
            false
        }
    }


    fn get_next(&mut self) -> String {
        let pos = self.target[self.last_pos..].find( &self.token );
        match pos{
            None => {
                let mut result = String::new();
                if self.last_pos != self.target_len {
                    result = self.target[self.last_pos..].to_string();
                }
                self.last_pos = self.target_len;
                return result
            },
            Option::Some(_pos) => {
                let result = self.target[self.last_pos..self.last_pos+_pos].to_string();
                self.last_pos = self.last_pos + _pos + self.token_len;
                return result
            }
        }
    }

    fn get_next_with_trim(&mut self) -> String {
        self.get_next().trim().to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_tokenizer_normal() {
        let target_string = "Hello,world,from,rust";

        let mut token = StringTokenizer::new( &target_string, "," );

        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next(), "Hello");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next(), "world");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next(), "from");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next(), "rust");
        assert_eq!(token.has_next(), false);
    }

    #[test]
    fn test_string_tokenizer_normal_2() {
        let target_string = "Hello,world,from,rust";

        let mut token = StringTokenizer::new( &target_string, "," );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 4);
    }

    #[test]
    fn test_string_tokenizer_without_token() {
        let target_string = "Hello,world,from,rust";

        let mut token = StringTokenizer::new( &target_string, "_" );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 1);
    }

    #[test]
    fn test_string_tokenizer_with_front_token() {
        let target_string = ",Hello,world,from,rust";

        let mut token = StringTokenizer::new( &target_string, "," );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 5);
    }

    #[test]
    fn test_string_tokenizer_with_token_last() {
        let target_string = "Hello,world,from,rust";

        let mut token = StringTokenizer::new( &target_string, "," );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 4);
    }

    #[test]
    fn test_string_tokenizer_with_multiple_token_bytes() {
        let target_string = "Hello,_,world,_,from,_,rust!,_,";

        let mut token = StringTokenizer::new( &target_string, ",_," );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 4);
    }

    #[test]
    fn test_string_tokenizer_only_token() {
        let target_string = ",,,";

        let mut token = StringTokenizer::new( &target_string, "," );

        let mut count = 0;
        while token.has_next() {
            println!( "{}", token.get_next() );
            count = count + 1;
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn test_string_tokenizer_with_trim() {
        let target_string = "Hello , world, from ,  rust  ";

        let mut token = StringTokenizer::new( &target_string, "," );

        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next_with_trim(), "Hello");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next_with_trim(), "world");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next_with_trim(), "from");
        assert_eq!(token.has_next(), true);
        assert_eq!(token.get_next_with_trim(), "rust");
        assert_eq!(token.has_next(), false);
    }
}
