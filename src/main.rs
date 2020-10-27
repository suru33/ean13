use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut ean_numbers: [i16; 12] = [0; 12];
    let mut checksum: i16 = 0;
    for i in 0..12 {
        ean_numbers[i] = rng.gen_range(0, 10);
        let ean_weight: i16 = if (i + 1) % 2 == 0 { 1 } else { 3 };
        checksum += ean_numbers[i] * ean_weight;
    }
    let x = if checksum % 10 == 0 { 0 } else {
        10 - (checksum % 10)
    };
    for i in (0..12).rev() {
        print!("{}", ean_numbers[i])
    }
    println!("{}", x)
}
