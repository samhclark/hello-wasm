use std::{fmt::Display, ops::Not};

enum CodeSet {
    A,
    B,
    C,
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Size {
    height: u64,
    width: u64,
}

#[derive(Clone, Copy)]
enum BarColor {
    Black,
    White,
}

impl Not for BarColor {
    type Output = BarColor;
    fn not(self) -> Self::Output {
        match self {
            BarColor::Black => BarColor::White,
            BarColor::White => BarColor::Black,
        }
    }
}

impl Display for BarColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarColor::Black => write!(f, "black"),
            BarColor::White => write!(f, "white"),
        }
    }
}

struct Bar {
    position: Point,
    size: Size,
    color: BarColor,
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<rect x="{}" y="{}" height="{}" width="{}" fill="{}"/>"#,
            self.position.x, self.position.y, self.size.height, self.size.width, self.color
        )
    }
}

const WIDTHS_LUT: [[u8; 6]; 107] = [
    [2, 1, 2, 2, 2, 2], //   0
    [2, 2, 2, 1, 2, 2], //   1
    [2, 2, 2, 2, 2, 1], //   2
    [1, 2, 1, 2, 2, 3], //   3
    [1, 2, 1, 3, 2, 2], //   4
    [1, 3, 1, 2, 2, 2], //   5
    [1, 2, 2, 2, 1, 3], //   6
    [1, 2, 2, 3, 1, 2], //   7
    [1, 3, 2, 2, 1, 2], //   8
    [2, 2, 1, 2, 1, 3], //   9
    [2, 2, 1, 3, 1, 2], //  10
    [2, 3, 1, 2, 1, 2], //  11
    [1, 1, 2, 2, 3, 2], //  12
    [1, 2, 2, 1, 3, 2], //  13
    [1, 2, 2, 2, 3, 1], //  14
    [1, 1, 3, 2, 2, 2], //  15
    [1, 2, 3, 1, 2, 2], //  16
    [1, 2, 3, 2, 2, 1], //  17
    [2, 2, 3, 2, 1, 1], //  18
    [2, 2, 1, 1, 3, 2], //  19
    [2, 2, 1, 2, 3, 1], //  20
    [2, 1, 3, 2, 1, 2], //  21
    [2, 2, 3, 1, 1, 2], //  22
    [3, 1, 2, 1, 3, 1], //  23
    [3, 1, 1, 2, 2, 2], //  24
    [3, 2, 1, 1, 2, 2], //  25
    [3, 2, 1, 2, 2, 1], //  26
    [3, 1, 2, 2, 1, 2], //  27
    [3, 2, 2, 1, 1, 2], //  28
    [3, 2, 2, 2, 1, 1], //  29
    [2, 1, 2, 1, 2, 3], //  30
    [2, 1, 2, 3, 2, 1], //  31
    [2, 3, 2, 1, 2, 1], //  32
    [1, 1, 1, 3, 2, 3], //  33
    [1, 3, 1, 1, 2, 3], //  34
    [1, 3, 1, 3, 2, 1], //  35
    [1, 1, 2, 3, 1, 3], //  36
    [1, 3, 2, 1, 1, 3], //  37
    [1, 3, 2, 3, 1, 1], //  38
    [2, 1, 1, 3, 1, 3], //  39
    [2, 3, 1, 1, 1, 3], //  40
    [2, 3, 1, 3, 1, 1], //  41
    [1, 1, 2, 1, 3, 3], //  42
    [1, 1, 2, 3, 3, 1], //  43
    [1, 3, 2, 1, 3, 1], //  44
    [1, 1, 3, 1, 2, 3], //  45
    [1, 1, 3, 3, 2, 1], //  46
    [1, 3, 3, 1, 2, 1], //  47
    [3, 1, 3, 1, 2, 1], //  48
    [2, 1, 1, 3, 3, 1], //  49
    [2, 3, 1, 1, 3, 1], //  50
    [2, 1, 3, 1, 1, 3], //  51
    [2, 1, 3, 3, 1, 1], //  52
    [2, 1, 3, 1, 3, 1], //  53
    [3, 1, 1, 1, 2, 3], //  54
    [3, 1, 1, 3, 2, 1], //  55
    [3, 3, 1, 1, 2, 1], //  56
    [3, 1, 2, 1, 1, 3], //  57
    [3, 1, 2, 3, 1, 1], //  58
    [3, 3, 2, 1, 1, 1], //  59
    [3, 1, 4, 1, 1, 1], //  60
    [2, 2, 1, 4, 1, 1], //  61
    [4, 3, 1, 1, 1, 1], //  62
    [1, 1, 1, 2, 2, 4], //  63
    [1, 1, 1, 4, 2, 2], //  64
    [1, 2, 1, 1, 2, 4], //  65
    [1, 2, 1, 4, 2, 1], //  66
    [1, 4, 1, 1, 2, 2], //  67
    [1, 4, 1, 2, 2, 1], //  68
    [1, 1, 2, 2, 1, 4], //  69
    [1, 1, 2, 4, 1, 2], //  70
    [1, 2, 2, 1, 1, 4], //  71
    [1, 2, 2, 4, 1, 1], //  72
    [1, 4, 2, 1, 1, 2], //  73
    [1, 4, 2, 2, 1, 1], //  74
    [2, 4, 1, 2, 1, 1], //  75
    [2, 2, 1, 1, 1, 4], //  76
    [4, 1, 3, 1, 1, 1], //  77
    [2, 4, 1, 1, 1, 2], //  78
    [1, 3, 4, 1, 1, 1], //  79
    [1, 1, 1, 2, 4, 2], //  80
    [1, 2, 1, 1, 4, 2], //  81
    [1, 2, 1, 2, 4, 1], //  82
    [1, 1, 4, 2, 1, 2], //  83
    [1, 2, 4, 1, 1, 2], //  84
    [1, 2, 4, 2, 1, 1], //  85
    [4, 1, 1, 2, 1, 2], //  86
    [4, 2, 1, 1, 1, 2], //  87
    [4, 2, 1, 2, 1, 1], //  88
    [2, 1, 2, 1, 4, 1], //  89
    [2, 1, 4, 1, 2, 1], //  90
    [4, 1, 2, 1, 2, 1], //  91
    [1, 1, 1, 1, 4, 3], //  92
    [1, 1, 1, 3, 4, 1], //  93
    [1, 3, 1, 1, 4, 1], //  94
    [1, 1, 4, 1, 1, 3], //  95
    [1, 1, 4, 3, 1, 1], //  96
    [4, 1, 1, 1, 1, 3], //  97
    [4, 1, 1, 3, 1, 1], //  98
    [1, 1, 3, 1, 4, 1], //  99
    [1, 1, 4, 1, 3, 1], // 100
    [3, 1, 1, 1, 4, 1], // 101
    [4, 1, 1, 1, 3, 1], // 102
    [2, 1, 1, 4, 1, 2], // 103
    [2, 1, 1, 2, 1, 4], // 104
    [2, 1, 1, 2, 3, 2], // 105
    [2, 3, 3, 1, 1, 1], // 106 
];

pub fn get_svg_barcode(value: &str) -> String {
    String::new()
}

pub fn generate_barcode_code128(_value: &str) -> String {
    // let x dimension = 1
    // hardcode
    let begin = r#"<svg width="300" height="70" xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="10" width="10" height="50" fill="white"/>

    <rect x="10" y="10" width="2" height="50" fill="black"/>
    <rect x="12" y="10" width="1" height="50" fill="white"/>
    <rect x="13" y="10" width="1" height="50" fill="black"/>
    <rect x="14" y="10" width="4" height="50" fill="white"/>
    <rect x="18" y="10" width="1" height="50" fill="black"/>
    <rect x="19" y="10" width="2" height="50" fill="white"/>"#;

    let content = get_bars_for_string("CSE370", 21);

    // <rect x="21" y="10" width="1" height="50" fill="black"/>
    // <rect x="22" y="10" width="3" height="50" fill="white"/>
    // <rect x="25" y="10" width="1" height="50" fill="black"/>
    // <rect x="26" y="10" width="3" height="50" fill="white"/>
    // <rect x="29" y="10" width="2" height="50" fill="black"/>
    // <rect x="31" y="10" width="1" height="50" fill="white"/>

    // <rect x="32" y="10" width="2" height="50" fill="black"/>
    // <rect x="34" y="10" width="1" height="50" fill="white"/>
    // <rect x="35" y="10" width="3" height="50" fill="black"/>
    // <rect x="38" y="10" width="1" height="50" fill="white"/>
    // <rect x="39" y="10" width="1" height="50" fill="black"/>
    // <rect x="40" y="10" width="3" height="50" fill="white"/>

    // <rect x="43" y="10" width="1" height="50" fill="black"/>
    // <rect x="44" y="10" width="3" height="50" fill="white"/>
    // <rect x="47" y="10" width="2" height="50" fill="black"/>
    // <rect x="49" y="10" width="1" height="50" fill="white"/>
    // <rect x="50" y="10" width="1" height="50" fill="black"/>
    // <rect x="51" y="10" width="3" height="50" fill="white"/>

    // <rect x="54" y="10" width="2" height="50" fill="black"/>
    // <rect x="56" y="10" width="2" height="50" fill="white"/>
    // <rect x="58" y="10" width="1" height="50" fill="black"/>
    // <rect x="59" y="10" width="1" height="50" fill="white"/>
    // <rect x="60" y="10" width="3" height="50" fill="black"/>
    // <rect x="63" y="10" width="2" height="50" fill="white"/>

    // <rect x="65" y="10" width="3" height="50" fill="black"/>
    // <rect x="66" y="10" width="1" height="50" fill="white"/>
    // <rect x="68" y="10" width="2" height="50" fill="black"/>
    // <rect x="70" y="10" width="1" height="50" fill="white"/>
    // <rect x="71" y="10" width="3" height="50" fill="black"/>
    // <rect x="74" y="10" width="1" height="50" fill="white"/>

    // <rect x="75" y="10" width="1" height="50" fill="black"/>
    // <rect x="76" y="10" width="2" height="50" fill="white"/>
    // <rect x="78" y="10" width="3" height="50" fill="black"/>
    // <rect x="81" y="10" width="1" height="50" fill="white"/>
    // <rect x="82" y="10" width="2" height="50" fill="black"/>
    // <rect x="84" y="10" width="2" height="50" fill="white"/>

    let end = r#"<rect x="86" y="10" width="2" height="50" fill="black"/>
    <rect x="88" y="10" width="2" height="50" fill="white"/>
    <rect x="90" y="10" width="1" height="50" fill="black"/>
    <rect x="91" y="10" width="2" height="50" fill="white"/>
    <rect x="93" y="10" width="3" height="50" fill="black"/>
    <rect x="96" y="10" width="1" height="50" fill="white"/>

    <rect x="97" y="10" width="2" height="50" fill="black"/>
    <rect x="99" y="10" width="3" height="50" fill="white"/>
    <rect x="102" y="10" width="3" height="50" fill="black"/>
    <rect x="105" y="10" width="1" height="50" fill="white"/>
    <rect x="106" y="10" width="1" height="50" fill="black"/>
    <rect x="107" y="10" width="1" height="50" fill="white"/>

    <rect x="108" y="10" width="2" height="50" fill="black"/>

    <rect x="110" y="10" width="10" height="50" fill="white"/>
</svg>"#;

    let mut to_return = String::new();
    to_return.push_str(begin);
    to_return.push_str(content.as_str());
    to_return.push_str(end);
    to_return
}

fn get_bars_for_string(s: &str, offset: i64) -> String {
    let mut bars = String::new();
    let mut current_offset = offset;

    for c in s.chars() {
        bars.push_str(get_bars_for_character(c, current_offset).as_str());
        current_offset += 11;
    }

    bars
}

fn get_bars_for_character(c: char, offset: i64) -> String {
    let mut bars = String::new();
    let mut current_offset = offset;
    let mut current_fill = BarColor::Black;
    let widths = get_widths_u8(c);

    for width in widths.iter() {
        let bar: Bar = Bar {
            position: Point {
                x: current_offset,
                y: 10,
            },
            size: Size {
                height: 50,
                width: *width as u64,
            },
            color: current_fill,
        };
        bars.push_str(bar.to_string().as_str());

        current_offset += i64::from(*width);
        current_fill = !current_fill;
    }

    bars
}

fn get_widths(c: char) -> &'static str {
    match c {
        'C' => "131321",
        'S' => "213113",
        'E' => "132113",
        '3' => "221132",
        '7' => "312131",
        '0' => "123122",
        _ => unreachable!(),
    }
}

fn get_widths_u8(c: char) -> [u8; 6] {
    match c {
        'C' => [1, 3, 1, 3, 2, 1],
        'S' => [2, 1, 3, 1, 1, 3],
        'E' => [1, 3, 2, 1, 1, 3],
        '3' => [2, 2, 1, 1, 3, 2],
        '7' => [3, 1, 2, 1, 3, 1],
        '0' => [1, 2, 3, 1, 2, 2],
        _ => unreachable!(),
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod code_128_tests {

    use super::*;

    #[test]
    fn get_widths_for_C() {
        assert_eq!(get_widths_u8('C'), [1, 3, 1, 3, 2, 1])
    }

    #[test]
    fn get_bars_for_C() {
        let expected = concat!(
            r#"<rect x="21" y="10" height="50" width="1" fill="black"/>"#,
            r#"<rect x="22" y="10" height="50" width="3" fill="white"/>"#,
            r#"<rect x="25" y="10" height="50" width="1" fill="black"/>"#,
            r#"<rect x="26" y="10" height="50" width="3" fill="white"/>"#,
            r#"<rect x="29" y="10" height="50" width="2" fill="black"/>"#,
            r#"<rect x="31" y="10" height="50" width="1" fill="white"/>"#
        );

        assert_eq!(get_bars_for_character('C', 21), expected)
    }

    #[test]
    fn get_bars_for_CS() {
        let expected = concat!(
            r#"<rect x="21" y="10" height="50" width="1" fill="black"/>"#,
            r#"<rect x="22" y="10" height="50" width="3" fill="white"/>"#,
            r#"<rect x="25" y="10" height="50" width="1" fill="black"/>"#,
            r#"<rect x="26" y="10" height="50" width="3" fill="white"/>"#,
            r#"<rect x="29" y="10" height="50" width="2" fill="black"/>"#,
            r#"<rect x="31" y="10" height="50" width="1" fill="white"/>"#,
            r#"<rect x="32" y="10" height="50" width="2" fill="black"/>"#,
            r#"<rect x="34" y="10" height="50" width="1" fill="white"/>"#,
            r#"<rect x="35" y="10" height="50" width="3" fill="black"/>"#,
            r#"<rect x="38" y="10" height="50" width="1" fill="white"/>"#,
            r#"<rect x="39" y="10" height="50" width="1" fill="black"/>"#,
            r#"<rect x="40" y="10" height="50" width="3" fill="white"/>"#
        );

        assert_eq!(get_bars_for_string("CS", 21), expected)
    }
}
