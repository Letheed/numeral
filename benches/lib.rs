use bencher::{benchmark_group, benchmark_main, Bencher};
use numeral::Cardinal;

macro_rules! bench_call_on_range {
    ($fn_name:ident, $numtype:ty) => {
        fn $fn_name(b: &mut Bencher) {
            b.iter(|| {
                for n in (<$numtype>::MIN)..=(<$numtype>::MAX) {
                    n.cardinal();
                }
            })
        }
    };
}

bench_call_on_range!(bench_call_on_range_i8, i8);
bench_call_on_range!(bench_call_on_range_u8, u8);

#[rustfmt::skip]
benchmark_group!(
    call_on_range,
    bench_call_on_range_i8,
    bench_call_on_range_u8,
);

macro_rules! bench_call_on_min_max {
    ($fn_name:ident, $numtype:ty) => {
        fn $fn_name(b: &mut Bencher) {
            b.iter(|| {
                <$numtype>::MAX.cardinal();
                <$numtype>::MIN.cardinal();
            })
        }
    };
}

bench_call_on_min_max!(bench_call_on_min_max_i8, i8);
bench_call_on_min_max!(bench_call_on_min_max_u8, u8);
bench_call_on_min_max!(bench_call_on_min_max_i16, i16);
bench_call_on_min_max!(bench_call_on_min_max_u16, u16);
bench_call_on_min_max!(bench_call_on_min_max_i32, i32);
bench_call_on_min_max!(bench_call_on_min_max_u32, u32);
bench_call_on_min_max!(bench_call_on_min_max_i64, i64);
bench_call_on_min_max!(bench_call_on_min_max_u64, u64);
bench_call_on_min_max!(bench_call_on_min_max_isize, isize);
bench_call_on_min_max!(bench_call_on_min_max_usize, usize);

benchmark_group!(
    call_on_min_max,
    bench_call_on_min_max_i8,
    bench_call_on_min_max_u8,
    bench_call_on_min_max_i16,
    bench_call_on_min_max_u16,
    bench_call_on_min_max_i32,
    bench_call_on_min_max_u32,
    bench_call_on_min_max_i64,
    bench_call_on_min_max_u64,
    bench_call_on_min_max_isize,
    bench_call_on_min_max_usize,
);
benchmark_main!(call_on_range, call_on_min_max);
