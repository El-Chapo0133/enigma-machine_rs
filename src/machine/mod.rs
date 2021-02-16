
mod rotor;



pub struct Machine {
        pub rotors: Vec<rotor::Rotor>,
        roll_impair_rotor: bool
}

impl Machine {
        pub fn new(rotor_count: i64) -> Machine {
                let values = ['a', 'b', 'c', 'd'].to_vec();

                let mut temp = Machine {
                        rotors: Vec::new(),
                        roll_impair_rotor: false,
                };
                for _ in 0..rotor_count {
                        temp.rotors.push(rotor::Rotor::new(values.clone()));
                }
                temp
        }

        pub fn encode(&self, input: String) -> String {

                let mut temp_encoded = String::new();

                for cell in input.chars() {
                        let mut temp_cell = cell;
                        for index_rotor in 0..self.rotors.len() {
                                temp_cell = self.rotors[index_rotor].get(temp_cell.clone());
                        }
                        self.rolls_rotors();
                        temp_encoded.push(temp_cell);
                }

                temp_encoded
        }

        fn rolls_rotors(&mut self) {
                let values_len = self.rotors[0].values_len;
                let mut should_roll_next = false;

                for (index, rotor) in self.rotors.iter().enumerate() {
                        if index % 2 == 1 {
                                if !self.roll_impair_rotor {
                                        continue;
                                }
                        }
                        if rotor.rolls == (values_len - 1) as u64 {
                                rotor.set_rolls(0);
                                should_roll_next = true;
                        } else {
                                rotor.roll();
                        }
                        if !should_roll_next {
                                break;
                        }
                }
        }
}