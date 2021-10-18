mod front_of_house;

pub use front_of_house::hosting;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn eat_at_restautant() {
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_in_restautant() {
    hosting::add_to_waitlist();
}
