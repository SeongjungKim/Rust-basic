#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn option_unwrap() {
        assert_eq!(Some(10).unwrap(), 10);
        assert_eq!(None.unwrap_or(10), 10);
        assert_eq!(None.unwrap_or_else(|| 5 * 2), 10);

        // panic occured
        Option::<i32>::None.unwrap();
        Option::<i32>::None.expect("Better say something when panicking");
    }

    #[test]
    fn option_working_with_value() {
        let mut o = Some(42);
        // take the value out and replace it with None
        let nr = o.take();
        assert!(o.is_none());
        // nr holds and option containing the value
        assert_eq!(nr, Some(42));

        let mut o = Some(42);
        // someties it's better to replace the value right away
        assert_eq!(o.replace(1535), Some(42));
        assert_eq!(o, Some(1535));

        let o = Some(1535);
        // map function works only son Some() values
        assert_eq!(o.map(|v| format!("{:#x}", v)), Some("0x5ff".to_owned()));

        let o = Some(1535);
        // Options can be transformed into a Result easily. "Nope" is the Err()
        match o.ok_or("Nope") {
            Ok(nr) => assert_eq!(nr, 1535),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn option_pattern_matching() {
        match Some(100) {
            Some(v) => assert_eq!(v, 100),
            None => assert!(false),
        };

        if let Some(v) = Some(42) {
            assert_eq!(v, 42);
        } else {
            assert!(false);
        }
    }
}
