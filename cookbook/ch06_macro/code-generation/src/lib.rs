#![allow(unused_macros)]

macro_rules! default_enum {
    ($name: ident, $($variant: ident => $val:expr),+) => {
        #[derive(PartialEq, Clone, Debug)]
        pub enum $name {
            Invalid,
            $($variant = $val),+
        }
        impl Default for $name {
            fn default() -> Self { $name::Invalid }
        }
    };
}

macro_rules! make_fn {
    ($i: ident, $body: block) => {
        fn $i () $body
    }
}

macro_rules! n_times {
    ($n: expr, $f: block) => {
        for _ in 0..$n {
            $f()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_n_times() {
        let mut i = 0;
        n_times!(5, {
            i += 1;
        });

        assert_eq!(i, 5);
    }

    #[test]
    #[should_panic]
    fn test_failing_make_fn() {
        make_fn!(fail, { assert!(false) });
        fail();
    }

    #[test]
    fn test_default_enum() {
        default_enum!(Colors, Red => 0xFF0000, Blue => 0x0000FF);
        let color: Colors = Default::default();
        
        assert_eq!(color, Colors::Invalid);
        assert_eq!(Colors::Red as i32, 0xFF0000);
        assert_eq!(Colors::Blue as i32, 0x0000FF);
    }
}
