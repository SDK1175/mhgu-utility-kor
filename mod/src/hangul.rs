//! Korean fork: Hangul typing for the PC search (2-beolsik, the standard Korean keyboard layout).
//!
//! The game window gets plain key presses (no system input method), so the syllables are put together here. The
//! Han/Eng key toggles it: the key marked 한/영 (LANG1) or, on keyboards without one, Right Alt.
#![allow(dead_code)]

const CHO: [char; 19] = ['ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
const JUNG: [char; 21] = ['ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ', 'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ'];
const JONG: [char; 28] = ['\0', 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ', 'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ'];
const JONG2: [(char, char, char); 11] = [('ㄱ', 'ㅅ', 'ㄳ'), ('ㄴ', 'ㅈ', 'ㄵ'), ('ㄴ', 'ㅎ', 'ㄶ'), ('ㄹ', 'ㄱ', 'ㄺ'), ('ㄹ', 'ㅁ', 'ㄻ'), ('ㄹ', 'ㅂ', 'ㄼ'),
    ('ㄹ', 'ㅅ', 'ㄽ'), ('ㄹ', 'ㅌ', 'ㄾ'), ('ㄹ', 'ㅍ', 'ㄿ'), ('ㄹ', 'ㅎ', 'ㅀ'), ('ㅂ', 'ㅅ', 'ㅄ')];
const JUNG2: [(char, char, char); 7] = [('ㅗ', 'ㅏ', 'ㅘ'), ('ㅗ', 'ㅐ', 'ㅙ'), ('ㅗ', 'ㅣ', 'ㅚ'), ('ㅜ', 'ㅓ', 'ㅝ'), ('ㅜ', 'ㅔ', 'ㅞ'), ('ㅜ', 'ㅣ', 'ㅟ'), ('ㅡ', 'ㅣ', 'ㅢ')];

/// The jamo of a key (a..z), 2-beolsik; Shift gives the doubled consonants and ㅒ ㅖ.
pub fn jamo(key: u8, shift: bool) -> Option<char> {
    if shift {
        let doubled = match key { b'q' => Some('ㅃ'), b'w' => Some('ㅉ'), b'e' => Some('ㄸ'), b'r' => Some('ㄲ'), b't' => Some('ㅆ'), b'o' => Some('ㅒ'), b'p' => Some('ㅖ'), _ => None };
        if doubled.is_some() {
            return doubled;
        }
    }
    const MAP: &[u8; 26] = b"abcdefghijklmnopqrstuvwxyz";
    const JAMO: [char; 26] = ['ㅁ', 'ㅠ', 'ㅊ', 'ㅇ', 'ㄷ', 'ㄹ', 'ㅎ', 'ㅗ', 'ㅑ', 'ㅓ', 'ㅏ', 'ㅣ', 'ㅡ', 'ㅜ', 'ㅐ', 'ㅔ', 'ㅂ', 'ㄱ', 'ㄴ', 'ㅅ', 'ㅕ', 'ㅍ', 'ㅈ', 'ㅌ', 'ㅛ', 'ㅋ'];
    MAP.iter().position(|&k| k == key).map(|i| JAMO[i])
}

fn vowel(c: char) -> bool { JUNG.contains(&c) }
fn pair(t: &[(char, char, char)], a: char, b: char) -> Option<char> { t.iter().find(|p| p.0 == a && p.1 == b).map(|p| p.2) }
fn split(t: &[(char, char, char)], c: char) -> Option<(char, char)> { t.iter().find(|p| p.2 == c).map(|p| (p.0, p.1)) }

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct State { cho: Option<char>, jung: Option<char>, jong: Option<char> }

impl State {
    fn empty(&self) -> bool { self.cho.is_none() && self.jung.is_none() }
    fn push(&self, out: &mut Out) {
        match (self.cho, self.jung) {
            (Some(c), Some(v)) => {
                let i = CHO.iter().position(|&x| x == c).unwrap_or(0) as u32;
                let m = JUNG.iter().position(|&x| x == v).unwrap_or(0) as u32;
                let f = self.jong.and_then(|j| JONG.iter().position(|&x| x == j)).unwrap_or(0) as u32;
                out.push(char::from_u32(0xAC00 + (i * 21 + m) * 28 + f).unwrap_or('?'));
            }
            (Some(c), None) => out.push(c),
            (None, Some(v)) => out.push(v),
            _ => {}
        }
    }
}

/// Text to put in place of the syllable being typed: syllables done, then the one still open.
pub struct Out { pub bytes: [u8; 24], pub len: usize, pub open: usize }
impl Out {
    fn new() -> Self { Self { bytes: [0; 24], len: 0, open: 0 } }
    fn push(&mut self, c: char) {
        let mut b = [0; 4];
        let s = c.encode_utf8(&mut b);
        if self.len + s.len() <= self.bytes.len() {
            self.bytes[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
            self.len += s.len();
        }
    }
    pub fn str(&self) -> &str { core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("") }
}

/// The syllable being typed (it is in the field already, `open` bytes before the caret) and how it was built.
pub struct Composer { state: State, history: [State; 8], depth: usize, pub open: usize }

impl Composer {
    pub const fn new() -> Self { Self { state: State { cho: None, jung: None, jong: None }, history: [State { cho: None, jung: None, jong: None }; 8], depth: 0, open: 0 } }
    pub fn active(&self) -> bool { !self.state.empty() }
    /// Done with the syllable (another key, the caret moved): it stays as typed.
    pub fn finish(&mut self) { *self = Self::new(); }

    fn set(&mut self, s: State) {
        if self.depth < self.history.len() {
            self.history[self.depth] = self.state;
            self.depth += 1;
        }
        self.state = s;
    }
    fn restart(&mut self, out: &mut Out, next: State) {
        self.state.push(out);
        self.depth = 0;
        self.state = next;
    }

    /// One jamo: the text that replaces the `open` bytes before the caret (then `open` is its last syllable's).
    pub fn key(&mut self, c: char) -> Out {
        let mut out = Out::new();
        let s = self.state;
        let next = State::default();
        if vowel(c) {
            match (s.cho, s.jung, s.jong) {
                (_, Some(v), None) => match pair(&JUNG2, v, c) {
                    Some(v2) => self.set(State { jung: Some(v2), ..s }),
                    None => self.restart(&mut out, State { jung: Some(c), ..next }),
                },
                (Some(_), None, _) => self.set(State { jung: Some(c), ..s }),
                (Some(_), Some(_), Some(j)) => {
                    // the final consonant starts the next syllable (the second of a pair)
                    let (keep, moved) = match split(&JONG2, j) { Some((a, b)) => (Some(a), b), None => (None, j) };
                    self.state = State { jong: keep, ..s };
                    self.restart(&mut out, State { cho: Some(moved), jung: Some(c), jong: None });
                    self.history[0] = State { cho: Some(moved), ..next };
                    self.depth = 1;
                }
                _ => self.restart(&mut out, State { jung: Some(c), ..next }),
            }
        } else {
            match (s.cho, s.jung, s.jong) {
                (Some(_), Some(_), None) if JONG.contains(&c) => self.set(State { jong: Some(c), ..s }),
                (Some(_), Some(_), Some(j)) => match pair(&JONG2, j, c) {
                    Some(j2) => self.set(State { jong: Some(j2), ..s }),
                    None => self.restart(&mut out, State { cho: Some(c), ..next }),
                },
                (None, None, _) => self.set(State { cho: Some(c), ..next }),
                _ => self.restart(&mut out, State { cho: Some(c), ..next }),
            }
        }
        let before = out.len;
        self.state.push(&mut out);
        out.open = out.len - before;
        self.open = out.open;
        out
    }

    /// Backspace while typing a syllable: one jamo back. None when there was none (an ordinary delete).
    pub fn back(&mut self) -> Option<Out> {
        if !self.active() {
            return None;
        }
        self.state = if self.depth > 0 { self.depth -= 1; self.history[self.depth] } else { State::default() };
        let mut out = Out::new();
        self.state.push(&mut out);
        out.open = out.len;
        self.open = out.len;
        Some(out)
    }
}

/// Compatibility consonant (ㄱ..ㅎ) -> the initial of a syllable, for initial-consonant search (ㄹㅇㅅ).
pub fn initial(c: char) -> Option<char> {
    let u = c as u32;
    if (0xAC00..=0xD7A3).contains(&u) { Some(CHO[((u - 0xAC00) / 588) as usize]) } else { None }
}
pub fn consonant(c: char) -> bool { CHO.contains(&c) }
/// A syllable with a single final consonant -> (the syllable without it, that consonant): "렝" -> ('레', 'ㅇ'), for a
/// search typed up to the next syllable's initial.
pub fn split_final(c: char) -> Option<(char, char)> {
    let u = c as u32;
    if !(0xAC00..=0xD7A3).contains(&u) { return None; }
    let f = ((u - 0xAC00) % 28) as usize;
    let j = JONG[f];
    (f > 0 && CHO.contains(&j)).then(|| (char::from_u32(u - f as u32).unwrap_or(c), j))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::string::String;
    fn typed(keys: &str) -> String {
        let mut c = Composer::new();
        let mut s = String::new();
        for k in keys.bytes() {
            if k == b'<' {
                let open = c.open;
                match c.back() { Some(o) => { s.truncate(s.len() - open); s.push_str(o.str()); } None => { s.pop(); } }
                continue;
            }
            let j = jamo(k.to_ascii_lowercase(), k.is_ascii_uppercase()).unwrap();
            let open = c.open;
            let o = c.key(j);
            s.truncate(s.len() - open);
            s.push_str(o.str());
        }
        s
    }
    #[test] fn words() {
        assert_eq!(typed("fpdntm"), "레우스");
        assert_eq!(typed("rkdcjf"), "강철");
        assert_eq!(typed("dlfr"), "읽");
        assert_eq!(typed("dlfrdj"), "읽어");
        assert_eq!(typed("dlfrj"), "일거");
        assert_eq!(typed("rkd<"), "가");
        assert_eq!(typed("rhk<"), "고");
        assert_eq!(typed("fpdntm<<"), "레우");
        assert_eq!(typed("ghks"), "환");
        assert_eq!(typed("Rk"), "까");
        assert_eq!(typed("dksgdk"), "않아");
        assert_eq!(typed("rhk"), "과");
        assert_eq!(typed("fhd"), "롱");
    }
}
