
mod machine;

fn main() {
        let machine = machine::Machine::new(1);

        // machine.rotors[0].print_values();

        println!("{}", machine.encode("aaa".to_string()));
}
