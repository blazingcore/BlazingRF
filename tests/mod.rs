mod configure;

#[cfg(test)]
mod tests {
    use blast::*;

    #[test]
    fn blast_init_test() {
        let result = Blast::default();
        assert!(!result.get_secret().is_empty());
    }
}
