use rasm::Programm;

fn main() {
    let mut programm = Programm::try_new("main.rasm").unwrap();
    programm.initialize();
}
