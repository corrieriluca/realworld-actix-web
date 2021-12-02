fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("1 + 2 = {}", add(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        assert_eq!(3, add(1, 2))
    }
}
