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
    fn new( target : String, token : String ) -> Self;
    fn has_next(&self) -> bool { false }
    fn get_next(&mut self) -> String { String::from("") }
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
    fn new( target : String, token : String ) -> Self {
        Self {
            target : target.clone(),
            token : token.clone(),
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
}


fn main() {
    let target_string = String::from("Hello,_,world,_,from,_,rust!,_,");

    let mut token = StringTokenizer::new( target_string, ",_,".to_string() );

    while token.has_next() {
        println!( "{}", token.get_next() );
    }
}
