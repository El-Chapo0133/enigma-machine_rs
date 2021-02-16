

pub struct Rotor {
        values: Vec<char>,
        pub values_len: usize,
        pub rolls: u64,
}

impl Rotor {
        pub fn new(values: Vec<char>) -> Rotor {
                Rotor {
                        values: values,
                        values_len: values.len(),
                        rolls: 1,
                }
        }

        pub fn get(&self, input: char) -> char {
                // find the index of the value
                let mut input_index: i64 = -1;
                for (index, value) in self.values.iter().enumerate() {
                        if value == &input {
                                input_index = index as i64;
                        }
                }
                if input_index == -1 {
                        panic!("Could not find the index of the value {}", input);
                }

                self.values[(input_index as u64 + self.rolls) as usize]
        }

        pub fn get_rolls(&self) -> u64 {
                self.rolls
        }
        pub fn set_rolls(&mut self, input: u64) {
                self.rolls = input;
        }
        pub fn roll(&mut self) {
                self.rolls += 1;
        }

        // pub fn print_values(&self) {
        //         println!("{:?}", self.values);
        // }
}