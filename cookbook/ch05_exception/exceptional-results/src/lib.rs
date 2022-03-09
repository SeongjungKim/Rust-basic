#[derive(Debug, PartialEq)]
pub enum FindError {
    EmptyNeedle,
    EmptyHaystack,
    NotFound,
}

/// 
/// Finds a needle in a haystack, returns a proper Result 
/// 
pub fn best_find(needle: &str, haystack: &str) -> Result<usize, FindError> {
    if needle.len() <= 0 {
        Err(FindError::EmptyNeedle)
    } else if haystack.len() <= 0 {
        Err(FindError::EmptyHaystack)
    } else {
        haystack.find(needle).map_or(Err(FindError::NotFound), |n| Ok(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_practice() {
        assert_eq!(best_find("a", "hello world"), Err(FindError::NotFound));
        assert_eq!(best_find("e", "hello world"), Ok(1));
        assert_eq!(best_find("", "hello world"), Err(FindError::EmptyNeedle));
        assert_eq!(best_find("e", ""), Err(FindError::EmptyHaystack));  
    }
}
