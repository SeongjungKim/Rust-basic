#[cfg(test)]
mod tests {
    #[test]
    fn positive_results() {
        let ok: Result<i32, f32> = Ok(42);

        // Ok(A) and Ok(B) is Ok(B)
        assert_eq!(ok.and(Ok(43)), Ok(43));
        // Ok and Err is Err
        assert_eq!(ok.and(err), err);
        // Ok or Err is Ok
        assert_eq!(ok.or(err), ok);

        // the positive results only have their own function
        // and then
        assert_eq!(ok.and_then(|r| Ok(r + 1)), Ok(43));
        // map
        assert_eq!(ok.map(|r| r + 1), Ok(43));

        let err: Result<i32, f32> = Err(-42.0);

    }

    #[test]
    fn negative_results() {
        let err: Result<i32, f32> = Err(-42.0);
        let err2: Result<i32, f32> = Err(-43.0);
        let ok: Result<i32, f32> = Ok(-41);

        // Err(A) and Err(B) is Err(A)
        assert_eq!(err.and(err2), err);
        // Err and Ok is Err
        assert_eq!(err.and(ok), err);

        // the negative results only have their own function
        // or_else (err -> ok)
        assert_eq!(err.or_else(|r| Ok(r as i32 + 1)), ok);
        // map_err (err -> err)
        assert_eq!(err.map_err(|r| r + 1.0), Err(-41.0));
    }
}
