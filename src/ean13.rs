use rand::Rng;

static WEIGHTS: [u8; 12] = [3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1];

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut ean_numbers: [u8; 12] = [0; 12];

    ean_numbers[0] = rng.gen_range(1, 10);
    for i in 1..12 {
        ean_numbers[i] = rng.gen_range(0, 10);
    }

    let checksum_digit = checksum_digit(&ean_numbers);

    let ean_numbers: String = ean_numbers
        .iter()
        .rev()
        .map(|&digit| digit.to_string())
        .collect();

    format!("{}{}", ean_numbers, checksum_digit)
}

pub fn is_valid(ean: &str) -> bool {
    if ean.len() != 13 {
        return false;
    }

    let mut digits: [u8; 13] = [0; 13];

    for (i, d) in ean.chars().enumerate() {
        match d.to_digit(10) {
            Some(value) => digits[i] = value as u8,
            None => return false,
        }
    }

    let mut ean_numbers: [u8; 12] = [0; 12];

    for i in 0..12 {
        ean_numbers[11 - i] = digits[i];
    }

    checksum_digit(&ean_numbers) == digits[12]
}

fn checksum_digit(ean_numbers: &[u8; 12]) -> u8 {
    let mut checksum: u8 = 0;

    for i in 0..12 {
        checksum += ean_numbers[i] * &WEIGHTS[i];
    }

    if checksum % 10 == 0 {
        0
    } else {
        10 - (checksum % 10)
    }
}
