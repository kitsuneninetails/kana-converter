#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HW_FW_KANA_MAP: HashMap<char, char> = {
        let hw_kana: Vec<char> = vec![
            ' ', '｡', '｢', '｣', '､', '･', 'ｰ',
            'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ',
            'ｶ', 'ｷ', 'ｸ', 'ｹ', 'ｺ',
            'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ',
            'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ',
            'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ',
            'ﾊ', 'ﾋ', 'ﾌ', 'ﾍ', 'ﾎ',
            'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ',
            'ﾔ',      'ﾕ',      'ﾖ',
            'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾛ',
            'ﾜ',                'ｦ',
            'ﾝ',
            'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ',
            'ｬ',      'ｭ',      'ｮ',
            'ｯ'];

        let fw_kana: Vec<char> = vec![
            '　', '。', '「', '」', '、', '・', 'ー',
            'ア', 'イ', 'ウ', 'エ', 'オ',
            'カ', 'キ', 'ク', 'ケ', 'コ',
            'サ', 'シ', 'ス', 'セ', 'ソ',
            'タ', 'チ', 'ツ', 'テ', 'ト',
            'ナ', 'ニ', 'ヌ', 'ネ', 'ノ',
            'ハ', 'ヒ', 'フ', 'ヘ', 'ホ',
            'マ', 'ミ', 'ム', 'メ', 'モ',
            'ヤ',       'ユ',       'ヨ',
            'ラ', 'リ', 'ル', 'レ', 'ロ',
            'ワ',                   'ヲ',
            'ン',
            'ァ', 'ィ', 'ゥ', 'ェ', 'ォ',
            'ャ',       'ュ',       'ョ',
            'ッ'];
        hw_kana.into_iter().zip(fw_kana.into_iter()).collect()
    };
}

/// Mode for conversion utilities, either only convert Kana (katakana and
/// hiragana) characters, ASCII characters, or both.
#[derive(Clone, PartialEq, Debug)]
pub enum ConvertMode {
    KanaOnly,
    AsciiOnly,
    KanaAndAscii
}
pub use ConvertMode::*;

/// Convert a string of single-byte ASCII and half-width katakana (半角 - hankaku)
/// characters into a string of full-width, double-byte (全角 - zenkaku) characters.
/// The `mode` parameter governs whether ASCII, katakana, or both type of characters
/// should be converted.
///
/// Example:
///
/// ```rust
/// use kana_converter::{to_double_byte, AsciiOnly, KanaOnly, KanaAndAscii};
///
/// let half_kana = "ｼﾝｸﾞﾙﾊﾞｲﾄｶﾅ";
/// let ascii = "ASCII";
/// let mixed = "ｼﾝｸﾞﾙﾊﾞｲﾄｶﾅ ASCII ダブルバイトカナ　ひあらがな　漢字";
///
/// assert_eq!(to_double_byte(half_kana, KanaOnly), "シングルバイトカナ");
/// assert_eq!(to_double_byte(half_kana, AsciiOnly), "ｼﾝｸﾞﾙﾊﾞｲﾄｶﾅ");
///
/// assert_eq!(to_double_byte(ascii, AsciiOnly), "ＡＳＣＩＩ");
/// assert_eq!(to_double_byte(ascii, KanaOnly), "ASCII");
///
/// assert_eq!(to_double_byte(mixed, AsciiOnly), "ｼﾝｸﾞﾙﾊﾞｲﾄｶﾅ　ＡＳＣＩＩ　ダブルバイトカナ　ひあらがな　漢字");
/// assert_eq!(to_double_byte(mixed, KanaOnly), "シングルバイトカナ　ASCII　ダブルバイトカナ　ひあらがな　漢字");
/// assert_eq!(to_double_byte(mixed, KanaAndAscii), "シングルバイトカナ　ＡＳＣＩＩ　ダブルバイトカナ　ひあらがな　漢字");
/// ```
pub fn to_double_byte(input: &str, mode: ConvertMode) -> String {
    fn check_voiced(next_char: Option<&char>) -> u32 {
        // Dakuten (voiced) and handakuten (semi-voiced) characters are
        // always one or two code points after (respectively) the base
        // un-voiced character.
        match next_char {
            Some(n_ch) =>
                match n_ch {
                    &'ﾞ' => 1, // Dakuten - ﾞ
                    &'ﾟ' => 2, // Handakuten - ﾟ
                    _ => 0
                },
            None => 0
        }
    }

    fn convert_kana_char(ch: char, next_ch: Option<&char>) -> char {
        HW_FW_KANA_MAP.get(&ch)
            .and_then(|c| std::char::from_u32(c.clone() as u32 + check_voiced(next_ch)))
            .unwrap_or(ch)
    }

    fn convert_ascii_char(ch: char) -> char {
        std::char::from_u32(
            if ch == ' ' {
                // Special ideographic (full-width) space
                0x3000
            } else {
                ch as u32 + 0xFEE0
            }).unwrap_or(ch)
    }

    fn hw_kana_check(byte_val: u32) -> bool {
        byte_val == 0x0020 || (byte_val >= 0xFF61 && byte_val <= 0xFF9D)
    }

    fn ascii_check(byte_val: u32) -> bool {
        byte_val >= 0x0020 && byte_val <= 0x007E
    }

    fn dakuten_check(byte_val: u32) -> bool {
        byte_val == 0xFF9E || byte_val == 0xFF9F
    }

    let mut out_chars: Vec<char> = Vec::new();
    let mut char_iter = input.chars().peekable();

    let change_ascii = mode == AsciiOnly || mode == KanaAndAscii;
    let change_kana = mode == KanaOnly || mode == KanaAndAscii;

    loop {
        let this_char_elem = char_iter.next();
        match this_char_elem {
            Some(ch) => {
                let byte_val = ch as u32;
                if change_ascii && ascii_check(byte_val) {
                    out_chars.push(convert_ascii_char(ch))
                } else if change_kana && hw_kana_check(byte_val) {
                    out_chars.push(convert_kana_char(ch, char_iter.peek()));
                } else if !change_kana || !dakuten_check(byte_val) {
                    // Skip dakuten and handakuten chars if kana is being changed
                    out_chars.push(ch)
                }
            },
            None => break
        }
    }
    out_chars.into_iter().collect()
}

#[cfg(test)] extern crate chrono;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_conversion_ascii() {
        let test_str = "ｶﾞｷﾞｸﾞｹﾞｺﾞｶｷｸｹｺｰ LATIN01234-!@#$%カタカナひらがな漢字";
        let out_str = to_double_byte(test_str, AsciiOnly);
        assert_eq!(out_str, "ｶﾞｷﾞｸﾞｹﾞｺﾞｶｷｸｹｺｰ　ＬＡＴＩＮ０１２３４－！＠＃＄％カタカナひらがな漢字")
    }

    #[test]
    fn test_conversion_kana() {
        let test_str = "ｶﾞｷﾞｸﾞｹﾞｺﾞｶｷｸｹｺｰ LATIN01234-!@#$%カタカナひらがな漢字";
        let out_str = to_double_byte(test_str, KanaOnly);
        assert_eq!(out_str, "ガギグゲゴカキクケコー　LATIN01234-!@#$%カタカナひらがな漢字")
    }

    #[test]
    fn test_conversion_both() {
        let test_str = "ｶﾞｷﾞｸﾞｹﾞｺﾞｶｷｸｹｺｰ LATIN01234-!@#$%カタカナひらがな漢字";
        let out_str = to_double_byte(test_str, KanaAndAscii);
        assert_eq!(out_str, "ガギグゲゴカキクケコー　ＬＡＴＩＮ０１２３４－！＠＃＄％カタカナひらがな漢字")
    }

    fn _test_conversion_speed() {
        let num_tests = 100_000;
        let start = Utc::now();
        println!("Start = {}", start);
        for _i in 0..num_tests {
            let test_str = "ｶﾞ";
            to_double_byte(test_str, KanaAndAscii);
        }
        let end = Utc::now();
        let dur = end.signed_duration_since(start);
        println!("End = {}", end);
        println!("Duration = {}", dur);
        println!("Fn/sec = {}", num_tests as f64 / (dur.num_milliseconds() as f64 / 1000.0));
    }
}