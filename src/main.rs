use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Tebak Angka!");
    // Mengenerate Angka Random 
    let secret_number = rand::thread_rng().gen_range(1..=100);    
    // Jika tebakan benar maka berhenti mengulang
    loop {   
        println!("Masukkan angka:");
        
        // Inisialisasi Variable         
        let mut guess = String::new();
        
        // Membaca masukan
        io::stdin()
            .read_line(&mut guess)
            .expect("Error");
        
        // Mengubah String menjadi number u32 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tebakan anda: {guess}");

        // Bandingkan angka
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Selamat anda menang!");
                println!("Angka yang benar adalah: {secret_number}");
                break;
            },
        }
    }
}

