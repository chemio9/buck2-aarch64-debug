/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::fmt;

/// Write out a u64 representing bytes as something more readable
#[derive(Debug)]
pub struct HumanizedBytes {
    bytes: u64,
    fixed_width: bool,
}

impl HumanizedBytes {
    pub fn new(bytes: u64) -> Self {
        HumanizedBytes {
            bytes,
            fixed_width: false,
        }
    }

    pub const FIXED_WIDTH_WIDTH: usize = 6;

    pub fn fixed_width(bytes: u64) -> Self {
        HumanizedBytes {
            bytes,
            fixed_width: true,
        }
    }
}

pub struct HumanizedBytesPerSecond {
    bytes_per_second: u64,
    fixed_width: bool,
}

impl HumanizedBytesPerSecond {
    pub fn new(bytes_per_second: u64) -> Self {
        HumanizedBytesPerSecond {
            bytes_per_second,
            fixed_width: false,
        }
    }

    pub const FIXED_WIDTH_WIDTH: usize = HumanizedBytes::FIXED_WIDTH_WIDTH + "/s".len();

    pub fn fixed_width(bytes_per_second: u64) -> Self {
        HumanizedBytesPerSecond {
            bytes_per_second,
            fixed_width: true,
        }
    }
}

pub struct HumanizedCount {
    count: u64,
    fixed_width: bool,
}

impl HumanizedCount {
    pub fn new(count: u64) -> Self {
        HumanizedCount {
            count,
            fixed_width: false,
        }
    }

    pub fn fixed_width(count: u64) -> Self {
        HumanizedCount {
            count,
            fixed_width: true,
        }
    }
}

struct Preformat {
    val: f64,
    label: &'static str,
    point: bool,
}

fn preformat(
    value: u64,
    factor: f64,
    one_label: &'static str,
    labels: &[&'static str],
) -> Preformat {
    let mut val = value as f64;
    let mut label = one_label;

    let mut labels = labels.iter();

    loop {
        if val < factor {
            break;
        }

        let next_label = match labels.next() {
            Some(l) => l,
            None => break,
        };

        val /= factor;
        label = next_label;
    }

    if f64::round(val) >= 1000.0 {
        if let Some(next_label) = labels.next() {
            val = 1.0;
            label = next_label;
        }
    }

    Preformat {
        val,
        label,
        point: value >= 1000 && f64::round(val) < 10.0,
    }
}

impl fmt::Display for HumanizedBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Preformat { val, label, point } =
            preformat(self.bytes, 1024.0, "B", &["KiB", "MiB", "GiB"]);

        match (point, self.fixed_width) {
            (false, false) => write!(f, "{:.0}{}", val, label),
            (true, false) => write!(f, "{:.1}{}", val, label),
            (false, true) => write!(f, "{:>3.0}{:<3}", val, label),
            (true, true) => write!(f, "{:>3.1}{:<3}", val, label),
        }
    }
}

impl fmt::Display for HumanizedBytesPerSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Preformat { val, label, point } = preformat(
            self.bytes_per_second,
            1024.0,
            "B/s",
            &["KiB/s", "MiB/s", "GiB/s"],
        );

        match (point, self.fixed_width) {
            (false, false) => write!(f, "{:.0}{}", val, label),
            (true, false) => write!(f, "{:.1}{}", val, label),
            (false, true) => write!(f, "{:>3.0}{:<5}", val, label),
            (true, true) => write!(f, "{:>3.1}{:<5}", val, label),
        }
    }
}

impl fmt::Display for HumanizedCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Preformat { val, label, point } = preformat(self.count, 1000.0, "", &["K", "M", "B"]);

        match (point, self.fixed_width) {
            (false, false) => write!(f, "{:.0}{}", val, label),
            (true, false) => write!(f, "{:.1}{}", val, label),
            (false, true) => write!(f, "{:>3.0}{:<1}", val, label),
            (true, true) => write!(f, "{:>3.1}{:<1}", val, label),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HumanizedBytes;
    use super::HumanizedBytesPerSecond;
    use super::HumanizedCount;

    #[allow(clippy::identity_op)]
    #[test]
    fn test_humanized() {
        fn t(value: u64, expected: &str, expected_fixed_width: &str) {
            assert_eq!(
                HumanizedBytes {
                    bytes: value,
                    fixed_width: false
                }
                .to_string(),
                expected
            );
            if value <= (1 << 9) {
                assert_eq!(
                    HumanizedBytes::FIXED_WIDTH_WIDTH,
                    expected_fixed_width.len()
                );
            }
            assert_eq!(
                HumanizedBytes {
                    bytes: value,
                    fixed_width: true,
                }
                .to_string(),
                expected_fixed_width
            );
        }

        t(0, "0B", "  0B  ");
        t(1, "1B", "  1B  ");
        t(10, "10B", " 10B  ");
        t(345, "345B", "345B  ");
        t(1023, "1.0KiB", "1.0KiB");
        t(1024, "1.0KiB", "1.0KiB");
        t(1536, "1.5KiB", "1.5KiB");
        t(10 * 1024 - 1, "10KiB", " 10KiB");
        t(10 * 1024 + 0, "10KiB", " 10KiB");
        t(654 * 1024 + 0, "654KiB", "654KiB");
        t(1024 * 1024 - 1, "1.0MiB", "1.0MiB");
        t(1024 * 1024 + 0, "1.0MiB", "1.0MiB");
        t(1024 * 1024 * 1024 - 1, "1.0GiB", "1.0GiB");
        t(2034 * 1024 * 1024 + 0, "2.0GiB", "2.0GiB");
        t(100500 * 1024 * 1024 * 1024, "100500GiB", "100500GiB");
    }

    #[test]
    fn test_humanized_bytes_per_second() {
        fn t(value: u64, expected: &str, expected_fixed_width: &str) {
            assert_eq!(
                HumanizedBytesPerSecond {
                    bytes_per_second: value,
                    fixed_width: false
                }
                .to_string(),
                expected
            );
            if value <= (1 << 9) {
                assert_eq!(
                    HumanizedBytesPerSecond::FIXED_WIDTH_WIDTH,
                    expected_fixed_width.len()
                );
            }
            assert_eq!(
                HumanizedBytesPerSecond {
                    bytes_per_second: value,
                    fixed_width: true,
                }
                .to_string(),
                expected_fixed_width
            );
        }

        t(0, "0B/s", "  0B/s  ");
        t(22, "22B/s", " 22B/s  ");
        t(1024 * 1024, "1.0MiB/s", "1.0MiB/s");
    }

    #[test]
    fn test_humanized_count() {
        fn t(value: u64, expected: &str, expected_fixed_width: &str) {
            assert_eq!(
                HumanizedCount {
                    count: value,
                    fixed_width: false
                }
                .to_string(),
                expected
            );
            assert_eq!(
                HumanizedCount {
                    count: value,
                    fixed_width: true,
                }
                .to_string(),
                expected_fixed_width
            );
        }

        t(0, "0", "  0 ");
        t(22, "22", " 22 ");
        t(1000000, "1.0M", "1.0M");
    }
}
