pub fn id(v: bool) -> bool {
    v
}

fn main() {
    print!("{:?}\n", id(true))
}

#[cfg(test)]
mod test {
    use super::id;

    #[test]
    fn basic() {
        assert_eq!(id(true), true);
        assert_eq!(id(false), false);
    }
}