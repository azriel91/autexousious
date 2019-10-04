#[cfg(test)]
mod tests {
    use team_model::play::IndependentCounter;

    #[test]
    fn get_and_increment_returns_incremented_value() {
        let mut independent_counter = IndependentCounter::new(5);
        let get = independent_counter.get_and_increment();

        assert_eq!(IndependentCounter::new(5), get);
    }

    #[test]
    fn get_and_increment_increments_self() {
        let mut independent_counter = IndependentCounter::new(5);
        let _ = independent_counter.get_and_increment();

        assert_eq!(IndependentCounter::new(6), independent_counter);
    }
}
