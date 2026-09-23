use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let answer = random_answer()?;
    let mut attempts = 0;

    println!("Hit and Blow: Guess the four unique digits. The first digit may be 0.");
    println!("Enter q to quit.");

    loop {
        print!("Guess > ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!();
            break;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") {
            break;
        }

        let Some(guess) = parse_guess(input) else {
            println!("Enter four distinct digits.");
            continue;
        };

        attempts += 1;
        let (hits, blows) = score(&answer, &guess);
        println!("{hits} hit(s), {blows} blow(s)");

        if hits == 4 {
            println!("Correct! You guessed it in {attempts} attempt(s).");
            break;
        }
    }

    Ok(())
}

fn random_answer() -> io::Result<[u8; 4]> {
    let mut digits = *b"0123456789";
    let mut random = [0; 9];
    File::open("/dev/urandom")?.read_exact(&mut random)?;

    for index in (1..digits.len()).rev() {
        digits.swap(index, usize::from(random[index - 1]) % (index + 1));
    }

    Ok(digits[..4].try_into().expect("slice has four digits"))
}

fn parse_guess(input: &str) -> Option<[u8; 4]> {
    let digits: [u8; 4] = input.as_bytes().try_into().ok()?;
    if digits
        .iter()
        .enumerate()
        .all(|(index, digit)| digit.is_ascii_digit() && !digits[..index].contains(digit))
    {
        Some(digits)
    } else {
        None
    }
}

fn score(answer: &[u8; 4], guess: &[u8; 4]) -> (usize, usize) {
    let hits = answer
        .iter()
        .zip(guess)
        .filter(|(answer_digit, guess_digit)| answer_digit == guess_digit)
        .count();
    let matches = guess.iter().filter(|digit| answer.contains(digit)).count();
    (hits, matches - hits)
}

#[cfg(test)]
mod tests {
    use super::{parse_guess, score};

    #[test]
    fn validates_guesses() {
        assert_eq!(parse_guess("0123"), Some(*b"0123"));
        assert_eq!(parse_guess("1123"), None);
        assert_eq!(parse_guess("123"), None);
        assert_eq!(parse_guess("12a4"), None);
    }

    #[test]
    fn counts_hits_and_blows() {
        assert_eq!(score(b"1234", b"1234"), (4, 0));
        assert_eq!(score(b"1234", b"1325"), (1, 2));
        assert_eq!(score(b"1234", b"5678"), (0, 0));
    }
}
