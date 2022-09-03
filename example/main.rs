use loopcat::lööp;

fn main() {
    lööp! {
        let mut amogs = String::new();

        println!("Mogus");

        std::io::stdin()
            .read_line(&mut amogs)
            .expect("Failed to read line");

        let amogs = amogs.trim();

        if amogs == "ng us" {
            println!("wtf");
            break;
        } else {
            println!("AMOGUS (It's time to stop!!)");
        }
    }
}
