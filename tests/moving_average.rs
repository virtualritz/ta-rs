struct Test {
    time_period: usize,
    index: usize,
    result: f64,
}

static SIMPLE_MOVING_AVERAGE_TESTS: [Test; 4] = [
    Test {
        time_period: 2,
        index: 0,
        result: 93.15,
    },
    Test {
        time_period: 2,
        index: 1,
        result: 94.59,
    },
    Test {
        time_period: 2,
        index: 2,
        result: 94.73,
    },
    Test {
        time_period: 2,
        index: 250,
        result: 108.31,
    },
];

static EXPONENTIAL_MOVING_AVERAGE_TESTS: [Test; 7] = [
    Test {
        time_period: 2,
        index: 0,
        result: 93.15,
    },
    Test {
        time_period: 2,
        index: 1,
        result: 93.96,
    },
    Test {
        time_period: 2,
        index: 250,
        result: 108.21,
    },
    Test {
        time_period: 10,
        index: 0,
        result: 93.22,
    },
    Test {
        time_period: 10,
        index: 1,
        result: 93.75,
    },
    Test {
        time_period: 10,
        index: 20,
        result: 86.46,
    },
    Test {
        time_period: 10,
        index: 242,
        result: 108.97,
    },
];

/* Misc tests: period 2, 10
   { 1, TA_ANY_MA_TEST, 0, 0, 251,  2, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,   0,  93.15, 1, 251 }, // First Value
   { 0, TA_ANY_MA_TEST, 0, 0, 251,  2, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,   1,  93.96, 1, 251 },
   { 0, TA_ANY_MA_TEST, 0, 0, 251,  2, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS, 250, 108.21, 1, 251 }, // Last Value

   { 1, TA_ANY_MA_TEST, 0, 0, 251,  10, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,    0,  93.22,  9, 243 }, // First Value
   { 0, TA_ANY_MA_TEST, 0, 0, 251,  10, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,    1,  93.75,  9, 243 },
   { 0, TA_ANY_MA_TEST, 0, 0, 251,  10, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,   20,  86.46,  9, 243 },
   { 0, TA_ANY_MA_TEST, 0, 0, 251,  10, TA_MAType_EMA, TA_COMPATIBILITY_DEFAULT, TA_SUCCESS,  242, 108.97,  9, 243 }, // Last Value
*/

use std::error::Error;
use ta::Next;

use crate::common::data::CLOSE;

mod common;
use common::*;

#[test]
fn simple_moving_average() -> Result<(), Box<dyn Error>> {
    use ta::{indicators::SimpleMovingAverage, Next};

    for test in &SIMPLE_MOVING_AVERAGE_TESTS {
        let sma = CLOSE[0..test.index + 1]
            .iter()
            .fold(
                SimpleMovingAverage::new(test.time_period).unwrap(),
                |mut sma, close| {
                    sma.next(*close);
                    sma
                },
            )
            .next(CLOSE[test.index + 1]);

        assert_eq!(test.result, clamp_to_two_digits(sma));
    }

    Ok(())
}

#[test]
fn exponential_moving_average() -> Result<(), Box<dyn Error>> {
    use ta::{indicators::ExponentialMovingAverage, Next};

    for (i, test) in EXPONENTIAL_MOVING_AVERAGE_TESTS.iter().enumerate() {
        let ema = CLOSE[0..test.index + 1]
            .iter()
            .fold(
                ExponentialMovingAverage::new(test.time_period).unwrap(),
                |mut ema, close| {
                    println!("{}", ema.next(*close));
                    ema
                },
            )
            .next(CLOSE[test.index + 1]);

        assert_eq!(test.result, clamp_to_two_digits(ema));
    }

    Ok(())
}
