

pub struct Rotor {
        values: Vec<String>,
        rolls: u64,
}

impl Rotor {
        pub fn new(values: Vec<String>) -> Rotor {
                Rotor {
                        values: values,
                        rolls: 0,
                }
        }

        pub fn get(&self, input: String) -> String {

                String::new()
        }

        pub fn print_values(&self) {
                println!("{:?}", self.values);
        }
}