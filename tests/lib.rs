use std::{fs, mem};

use numeral::Cardinal;

macro_rules! test_call_on_min_max {
    ($fn_name:ident, $numtype:ty) => {
        #[test]
        fn $fn_name() {
            <$numtype>::MAX.cardinal();
            <$numtype>::MIN.cardinal();
        }
    };
}

test_call_on_min_max!(call_on_min_max_i8, i8);
test_call_on_min_max!(call_on_min_max_u8, u8);
test_call_on_min_max!(call_on_min_max_i16, i16);
test_call_on_min_max!(call_on_min_max_u16, u16);
test_call_on_min_max!(call_on_min_max_i32, i32);
test_call_on_min_max!(call_on_min_max_u32, u32);
test_call_on_min_max!(call_on_min_max_i64, i64);
test_call_on_min_max!(call_on_min_max_u64, u64);
test_call_on_min_max!(call_on_min_max_isize, isize);
test_call_on_min_max!(call_on_min_max_usize, usize);

macro_rules! test_call_on_full_range {
    ($fn_name:ident, $numtype:ty) => {
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::MIN)..=(<$numtype>::MAX) {
                n.cardinal();
            }
        }
    };
}

test_call_on_full_range!(call_on_full_range_i8, i8);
test_call_on_full_range!(call_on_full_range_u8, u8);
test_call_on_full_range!(call_on_full_range_i16, i16);
test_call_on_full_range!(call_on_full_range_u16, u16);

macro_rules! test_call_on_critical_ranges {
    ($fn_name:ident, $numtype:ty) => {
        #[test]
        fn $fn_name() {
            for n in (<$numtype>::MIN)..=(<$numtype>::MIN) + 130 {
                n.cardinal();
            }
            for n in (<$numtype>::MAX) - 130..=(<$numtype>::MAX) {
                n.cardinal();
            }
            if <$numtype>::MIN != 0 {
                for n in -130..=130 {
                    n.cardinal();
                }
            }
        }
    };
}

test_call_on_critical_ranges!(call_on_critical_ranges_i32, i32);
test_call_on_critical_ranges!(call_on_critical_ranges_u32, u32);
test_call_on_critical_ranges!(call_on_critical_ranges_i64, i64);
test_call_on_critical_ranges!(call_on_critical_ranges_u64, u64);
test_call_on_critical_ranges!(call_on_critical_ranges_isize, isize);
test_call_on_critical_ranges!(call_on_critical_ranges_usize, usize);

#[test]
fn cardinal_value_m256_256() {
    let cardinals = fs::read_to_string("tests/cardinal_-256..=256.txt").unwrap();
    assert!(cardinals.lines().eq((-256..=256).map(|n: i32| n.cardinal())));
}

#[test]
fn cardinal_value_min_max_int() {
    let cardinals = fs::read_to_string("tests/cardinal_min_max.txt").unwrap();
    let mut lines = cardinals.lines();
    macro_rules! assert_eq_min_max {
        ($signed:ty, $unsigned:ty) => {
            assert_eq!(lines.next().unwrap(), <$signed>::MIN.cardinal());
            assert_eq!(lines.next().unwrap(), <$signed>::MAX.cardinal());
            assert_eq!(lines.next().unwrap(), <$unsigned>::MAX.cardinal());
        };
    }
    assert_eq!(lines.next().unwrap(), 0.cardinal());
    assert_eq_min_max!(i8, u8);
    assert_eq_min_max!(i16, u16);
    assert_eq_min_max!(i32, u32);
    assert_eq_min_max!(i64, u64);
}

#[test]
fn cardinal_value_min_max_ptr() {
    macro_rules! assert_eq_min_max_if_ptr_is {
        ($ptr:ty, $int:ty) => {
            if mem::size_of::<$ptr>() == mem::size_of::<$int>() {
                assert_eq!(<$ptr>::MIN.cardinal(), <$int>::MIN.cardinal());
                assert_eq!(<$ptr>::MAX.cardinal(), <$int>::MAX.cardinal());
            }
        };
    }
    assert_eq_min_max_if_ptr_is!(isize, i8);
    assert_eq_min_max_if_ptr_is!(isize, i16);
    assert_eq_min_max_if_ptr_is!(isize, i32);
    assert_eq_min_max_if_ptr_is!(isize, i64);
    assert_eq_min_max_if_ptr_is!(usize, u8);
    assert_eq_min_max_if_ptr_is!(usize, u16);
    assert_eq_min_max_if_ptr_is!(usize, u32);
    assert_eq_min_max_if_ptr_is!(usize, u64);
}
