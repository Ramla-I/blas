#![feature(macro_rules)]

extern crate libc;

use libc::{c_char, c_double, c_int};

#[link(name = "gfortran")]
#[link(name = "blas", kind = "static")]
extern {
    fn dgemv_(trans: *const c_char, m: *const c_int, n: *const c_int,
        alpha: *const c_double, a: *const c_double, lda: *const c_int,
        x: *const c_double, incx: *const c_int, beta: *const c_double,
        y: *mut c_double, incy: *const c_int);

    fn dgemm_(transa: *const c_char, transb: *const c_char, m: *const c_int,
        n: *const c_int, k: *const c_int, alpha: *const c_double,
        a: *const c_double, lda: *const c_int, b: *const c_double,
        ldb: *const c_int, beta: *const c_double, c: *mut c_double,
        ldc: *const c_int);
}

pub static NORMAL: i8 = 'N' as i8;
pub static TRANSPOSED: i8 = 'T' as i8;

#[inline]
pub fn dgemv(trans: i8, m: i32, n: i32, alpha: f64, a: *const f64,
    lda: i32, x: *const f64, incx: i32, beta: f64, y: *mut f64, incy: i32) {

    unsafe {
        dgemv_(&trans, &m, &n, &alpha, a, &lda, x, &incx, &beta, y, &incy);
    }
}

#[inline]
pub fn dgemm(transa: i8, transb: i8, m: i32, n: i32, k: i32, alpha: f64, a: *const f64,
    lda: i32, b: *const f64, ldb: i32, beta: f64, c: *mut f64, ldc: i32) {

    unsafe {
        dgemm_(&transa, &transb, &m, &n, &k, &alpha, a, &lda, b, &ldb, &beta, c, &ldc);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use self::test::Bencher;

    macro_rules! assert_equal(
        ($given:expr , $expected:expr) => ({
            assert_eq!($given.len(), $expected.len());
            for i in range(0u, $given.len()) {
                assert_eq!($given[i], $expected[i]);
            }
        });
    )

    #[test]
    fn dgemv() {
        let (m, n) = (2, 3);
        let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
        let x = vec![1.0, 2.0, 3.0];
        let mut y = vec![6.0, 8.0];

        super::dgemv(super::NORMAL, m, n, 1.0, a.as_ptr(), m, x.as_ptr(), 1,
            1.0, y.as_mut_ptr(), 1);

        let expected_y = vec![20.0, 40.0];
        assert_equal!(y, expected_y);
    }

    #[test]
    fn dgemm() {
        let (m, n, k) = (2, 4, 3);
        let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
        let b = vec![1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0];
        let mut c = vec![2.0, 7.0, 6.0, 2.0, 0.0, 7.0, 4.0, 2.0];

        super::dgemm(super::NORMAL, super::NORMAL, m, n, k, 1.0, a.as_ptr(),
            m, b.as_ptr(), k, 1.0, c.as_mut_ptr(), m);

        let expected_c = vec![40.0, 90.0, 50.0, 100.0, 50.0, 120.0, 60.0, 130.0];
        assert_equal!(c, expected_c);
    }

    #[bench]
    fn dgemv_few_large(b: &mut Bencher) {
        let m = 1000;
        let a = Vec::from_elem(m * m, 1.0);
        let x = Vec::from_elem(m * 1, 1.0);
        let mut y = Vec::from_elem(m * 1, 1.0);

        b.iter(|| {
            super::dgemv(super::NORMAL, m as i32, m as i32, 1.0, a.as_ptr(),
                m as i32, x.as_ptr(), 1, 1.0, y.as_mut_ptr(), 1)
        })
    }

    #[bench]
    fn dgemv_many_small(b: &mut Bencher) {
        let m = 20;
        let a = Vec::from_elem(m * m, 1.0);
        let x = Vec::from_elem(m * 1, 1.0);
        let mut y = Vec::from_elem(m * 1, 1.0);

        b.iter(|| {
            for _ in range(0u, 20000) {
                super::dgemv(super::NORMAL, m as i32, m as i32, 1.0, a.as_ptr(),
                    m as i32, x.as_ptr(), 1, 1.0, y.as_mut_ptr(), 1);
            }
        })
    }
}
