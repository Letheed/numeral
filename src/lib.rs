#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![deny(unsafe_code)]

//! A library providing the written english form of a number.
//!
//! # Example
//!
//! ```rust
//! use numeral::Cardinal;
//!
//! let n = 127;
//! println!("{} is written {}", n, n.cardinal());
//! ```

const NUMBER: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

#[rustfmt::skip]
const TENS: [&str; 10] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

const MULTIPLIER: [&str; 9] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
    "septillion",
];

/// Provides the cardinal written form of a number.
pub trait Cardinal {
    /// Returns the cardinal form of a number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use numeral::Cardinal;
    /// let written_form = 127.cardinal();
    /// assert_eq!(written_form, "one hundred twenty-seven");
    /// ```
    fn cardinal(&self) -> String;
}

macro_rules! impl_numeral_signed {
    ($($numtype:ty),*) => {$(
        impl Cardinal for $numtype {
            fn cardinal(&self) -> String {
                cardinal_int(i64::from(*self).abs() as u64, self.is_negative())
            }
        }
    )*};
}

impl_numeral_signed!(i8, i16, i32);

macro_rules! impl_numeral_unsigned {
    ($($numtype:ty),*) => {$(
        impl Cardinal for $numtype {
            fn cardinal(&self) -> String {
                cardinal_int(u64::from(*self), false)
            }
        }
    )*};
}

impl_numeral_unsigned!(u8, u16, u32, u64);

impl Cardinal for i64 {
    fn cardinal(&self) -> String {
        #[allow(clippy::cast_sign_loss)] // Sign loss is intended.
        let n_abs = if *self == Self::MIN { *self as u64 } else { self.abs() as u64 };
        cardinal_int(n_abs, self.is_negative())
    }
}

impl Cardinal for isize {
    fn cardinal(&self) -> String {
        #[allow(clippy::cast_sign_loss)] // Sign loss is intended.
        let n_abs = if *self == Self::MIN { *self as usize } else { self.abs() as usize };
        cardinal_int(n_abs as u64, self.is_negative())
    }
}

impl Cardinal for usize {
    fn cardinal(&self) -> String {
        cardinal_int(*self as u64, false)
    }
}

/// Returns the written form of any 64-bit unsigned integer as a string.
fn cardinal_int(n: u64, negative: bool) -> String {
    if n == 0 {
        return String::from(NUMBER[0]);
    }
    #[allow(clippy::cast_possible_truncation)] // Truncation is intended.
    #[allow(clippy::cast_precision_loss)] // Order of magnitude is all we need.
    #[allow(clippy::cast_sign_loss)] // n>=1 => log10(n)>=0.
    let multiple_order = ((n as f32).log10() as u32) / 3;
    let max_len = multiple_order as usize * 8 + 6;
    let mut cardinal = Vec::with_capacity(max_len);
    if negative {
        cardinal.push("minus ");
    }
    compose_cardinal_int(n, multiple_order, &mut cardinal);
    debug_assert!(cardinal.len() <= max_len);
    cardinal.concat()
}

macro_rules! push {
    ($vec:ident, $table:ident[$index:ident]) => {
        debug_assert!(
            ($index as usize) < $table.len(),
            "{} out of {}'s range",
            stringify!($index),
            stringify!($table)
        );
        $vec.push($table[$index as usize]);
    };
}

// This should be in the 2nd debug_assert, above the lhs argument.
// But attributes on expressions are unstable.
#[allow(clippy::cast_possible_truncation)] // Truncation is intended.
#[allow(clippy::cast_precision_loss)] // Order of magnitude is all we need.
#[allow(clippy::cast_sign_loss)] // n>=1 => log10(n)>=0.
/// Pushes the strings composing the cardinal form of any 64-bit unsigned
/// integer on a `str` stack. Zero is ignored.
fn compose_cardinal_int(mut n: u64, mut multiple_order: u32, cardinal: &mut Vec<&str>) {
    debug_assert!(n != 0, "n == 0 in compose_cardinal_int()");
    debug_assert!(
        multiple_order == ((n as f32).log10() as u32) / 3,
        "wrong value for multiple_order in compose_cardinal_int()"
    );

    if multiple_order > 0 {
        let mut multiplier = 10_u64.pow(multiple_order * 3);
        let mut multiplicand;
        loop {
            multiplicand = n / multiplier;
            n %= multiplier;
            if multiplicand != 0 {
                push_triplet(multiplicand, cardinal);
                cardinal.push(" ");
                push!(cardinal, MULTIPLIER[multiple_order]);
                if n == 0 {
                    return;
                }
                cardinal.push(" ");
            }
            multiple_order -= 1;
            if multiple_order == 0 {
                break;
            }
            multiplier /= 1000;
        }
    }
    push_triplet(n, cardinal);
}

/// Takes an integer in [1,999] and adds it's written form to a `str` stack.
fn push_triplet(n: u64, cardinal: &mut Vec<&str>) {
    debug_assert!(n != 0, "n == 0 in push_triplet()");
    debug_assert!(n < 1000, "n >= 1000 in push_triplet()");

    let hundreds = n / 100;
    let rest = n % 100;
    if hundreds != 0 {
        push!(cardinal, NUMBER[hundreds]);
        if rest == 0 {
            cardinal.push(" hundred");
            return;
        }
        cardinal.push(" hundred ");
    }
    push_doublet(rest, cardinal);
}

/// Takes an integer in [1,99] and adds it's written form to a `str` stack.
fn push_doublet(n: u64, cardinal: &mut Vec<&str>) {
    debug_assert!(n != 0, "n == 0 in push_doublet()");
    debug_assert!(n < 100, "n >= 100 in push_doublet()");

    if n < 20 {
        push!(cardinal, NUMBER[n]);
    } else {
        let tens = n / 10;
        let ones = n % 10;
        push!(cardinal, TENS[tens]);
        if ones != 0 {
            cardinal.push("-");
            push!(cardinal, NUMBER[ones]);
        }
    }
}
