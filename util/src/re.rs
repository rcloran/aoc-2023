use std::fmt::Debug;
use std::str::FromStr;

use regex::{bytes::Regex as BytesRegex, Regex};

pub fn extract<S, T>(lines: impl Iterator<Item = S>, re: Regex) -> impl Iterator<Item = Vec<T>>
where
    S: AsRef<str>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines.map(move |l| {
        re.captures_iter(l.as_ref())
            .map(|c| c.extract())
            .map(|(n, [])| {
                n.parse()
                    .or_else(|e| {
                        if n.starts_with("-") {
                            n[1..].parse()
                        } else {
                            Err(e)
                        }
                    })
                    .unwrap()
            })
            .collect()
    })
}

pub fn extract_groups<S>(
    lines: impl Iterator<Item = S>,
    re: Regex,
) -> impl Iterator<Item = Vec<Option<String>>>
where
    S: AsRef<str>,
{
    lines.map(move |l| {
        re.captures_iter(l.as_ref())
            .map(|c| {
                c.iter()
                    .skip(1)
                    .map(|op_m| op_m.map(|m| m.as_str().to_owned()))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    })
}

pub fn extract_u8(lines: impl Iterator<Item = Vec<u8>>, re: &BytesRegex) -> Vec<Vec<Vec<u8>>> {
    lines
        .map(|l| {
            re.captures_iter(&l)
                .map(|c| c.extract())
                .map(|(n, [])| n.to_vec())
                .collect()
        })
        .collect()
}

pub fn extract_numbers<S, T>(lines: impl Iterator<Item = S>) -> impl Iterator<Item = Vec<T>>
where
    S: AsRef<str>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let re = Regex::new(r"[-+]?[0-9]+").unwrap();
    extract(lines, re)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers_2016_04_unsigned() {
        let s = include_str!("test-extract_numbers-2016-04.txt");
        let r = extract_numbers::<_, u32>(s.split("\n")).collect::<Vec<_>>();
        assert_eq!(r[0], [561]);
        assert_eq!(r[1], [866]);
        assert_eq!(r[2], [743]);
    }

    #[test]
    fn test_extract_numbers_2016_04_signed() {
        let s = include_str!("test-extract_numbers-2016-04.txt");
        let r = extract_numbers::<_, i32>(s.split("\n")).collect::<Vec<_>>();
        assert_eq!(r[0], [-561]);
        assert_eq!(r[1], [-866]);
        assert_eq!(r[2], [-743]);
    }

    #[test]
    #[should_panic]
    fn test_extract_numbers_2016_04_wontfit() {
        let s = include_str!("test-extract_numbers-2016-04.txt");
        let _ = extract_numbers::<_, i8>(s.split("\n")).collect::<Vec<_>>();
    }

    #[test]
    fn test_extract_numbers_2016_20_unsigned() {
        let s = include_str!("test-extract_numbers-2016-20.txt");
        let r = extract_numbers::<_, usize>(s.split("\n")).collect::<Vec<_>>();
        assert_eq!(r[0], [2803551464, 2812875810]);
        assert_eq!(r[1], [3863319608, 3871068145]);
        assert_eq!(r[2], [881357481, 892360003]);
        assert_eq!(r[3], [1109987968, 1119969449]);
        assert_eq!(r[4], [3658860150, 3661459953]);
        assert_eq!(r[5], [3315323905, 3317364128]);
        assert_eq!(r[6], [4198570301, 4213558177]);
        assert_eq!(r[7], [3652285991, 3657587289]);
        assert_eq!(r[8], [1721298471, 1722969206]);
        assert_eq!(r[9], [3232746849, 3243358546]);
    }

    #[test]
    fn test_extract_numbers_2016_20_signed() {
        let s = include_str!("test-extract_numbers-2016-20.txt");
        let r = extract_numbers::<_, isize>(s.split("\n")).collect::<Vec<_>>();
        assert_eq!(r[0], [2803551464, -2812875810]);
        assert_eq!(r[1], [3863319608, -3871068145]);
        assert_eq!(r[2], [881357481, -892360003]);
        assert_eq!(r[3], [1109987968, -1119969449]);
        assert_eq!(r[4], [3658860150, -3661459953]);
        assert_eq!(r[5], [3315323905, -3317364128]);
        assert_eq!(r[6], [4198570301, -4213558177]);
        assert_eq!(r[7], [3652285991, -3657587289]);
        assert_eq!(r[8], [1721298471, -1722969206]);
        assert_eq!(r[9], [3232746849, -3243358546]);
    }

    #[test]
    fn test_extract_numbers_2016_22() {
        let s = include_str!("test-extract_numbers-2016-22.txt");
        let r = extract_numbers::<_, usize>(s.split("\n")).collect::<Vec<_>>();
        assert_eq!(r[0], []);
        assert_eq!(r[1], []);
        assert_eq!(r[2], [0, 0, 89, 65, 24, 73]);
        assert_eq!(r[3], [0, 1, 92, 64, 28, 69]);
    }

    #[test]
    fn test_extract_2016_04() {
        let s = include_str!("test-extract_numbers-2016-04.txt");
        let re = Regex::new(r"((?:[a-z]+-)+)([0-9]+)\[([a-z]+)\]").unwrap();
        let r = extract_groups(s.split("\n"), re).collect::<Vec<_>>();
        println!("{:?}", r);
        assert_eq!(
            r[0],
            [
                Some("aczupnetwp-dnlgpyrpc-sfye-dstaatyr-".to_string()),
                Some("561".to_string()),
                Some("patyc".to_string())
            ]
        );
    }
}
