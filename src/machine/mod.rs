
mod rotor;



pub struct Machine {
        pub rotors: Vec<rotor::Rotor>
}

impl Machine {
        pub fn new(rotor_count: i64) -> Machine {
                let mut temp = Machine {
                        rotors: Vec::new(),
                };
                for _ in 0..rotor_count {
                        temp.rotors.push(rotor::Rotor::new([].to_vec()));
                }
                temp
        }

        pub fn encode(&self, input: String) -> String {

                String::new()
        }

        fn rolls_rotors(&self) {

        }
}