//! Korean fork: the text drawn in Korean.
//!
//! - Game names (items, weapons, armor, skills, ...): read when the utility first draws, from the game's own tables
//!   (the GMDs the game reads, so a Korean patch's when one is installed). The patch writes Hangul as kanji: its font
//!   (resident.arc, font_festa) draws the 2350 KS X 1001 syllables over the glyphs from the one at U+7A00, in order,
//!   so the n-th code from there in that font is the n-th syllable. That table is read from the font here. Which GMD string an English name is comes from
//!   ko_keys (tools/kor/keys.py). Nothing of the game's text is in the plugin.
//! - Everything else: ko_dict (tools/kor/ui_ko.tsv), our own translations.
//! A string that is neither (or a name the game's file has no Korean for: an unpatched game) is drawn as it is.
use alloc::{borrow::Cow, collections::BTreeMap, string::String, vec::Vec};
use core::cell::UnsafeCell;
use core::ffi::{c_char, c_void};
use forge::log::prelude::info;
use super::{ko_keys as keys, ko_dict, ko_ks};

#[repr(C)]
#[derive(Clone, Copy)]
struct FileHandle(u32);
unsafe extern "C" {
    #[link_name = "_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci"]
    fn open_file(handle: *mut FileHandle, path: *const c_char, mode: i32) -> u32;
    #[link_name = "_ZN2nn2fs11GetFileSizeEPxNS0_10FileHandleE"]
    fn get_file_size(size: *mut i64, handle: FileHandle) -> u32;
    #[link_name = "_ZN2nn2fs8ReadFileENS0_10FileHandleExPvj"]
    fn read_file(handle: FileHandle, offset: i64, bytes: *mut c_void, size: usize) -> u32;
    #[link_name = "_ZN2nn2fs9CloseFileENS0_10FileHandleE"]
    fn close_file(handle: FileHandle);
}

struct Names { loaded: bool, pool: Vec<u8>, spans: Vec<(u32, u32)>, composed: BTreeMap<String, String> }
struct Local(UnsafeCell<Names>);
unsafe impl Sync for Local {}
static NAMES: Local = Local(UnsafeCell::new(Names { loaded: false, pool: Vec::new(), spans: Vec::new(), composed: BTreeMap::new() }));

const MAX_FILE: i64 = 8 << 20;

fn read(path: &str) -> Option<Vec<u8>> { read_at(path, 0, None) }

/// `size` bytes of a RomFS file from `offset` (None: the rest of the file).
fn read_at(path: &str, offset: usize, size: Option<usize>) -> Option<Vec<u8>> {
    let mount = forge::fs::romfs::mount_point();
    let mut full = Vec::with_capacity(mount.len() + path.len() + 3);
    full.extend_from_slice(mount.as_bytes());
    full.extend_from_slice(b":/");
    full.extend_from_slice(path.as_bytes());
    full.push(0);
    let mut file = FileHandle(0);
    if unsafe { open_file(&mut file, full.as_ptr().cast(), 1) } != 0 {
        return None;
    }
    let want = size;
    let mut size = 0i64;
    let mut data = Vec::new();
    let mut ok = unsafe { get_file_size(&mut size, file) } == 0 && (offset as i64) < size;
    if ok {
        let len = want.unwrap_or(size as usize - offset);
        ok = len as i64 <= MAX_FILE && (offset + len) as i64 <= size;
        if ok {
            data.resize(len, 0);
            ok = unsafe { read_file(file, offset as i64, data.as_mut_ptr().cast(), len) } == 0;
        }
    }
    unsafe { close_file(file) };
    ok.then_some(data)
}

fn u32_at(d: &[u8], at: usize) -> usize { u32::from_le_bytes([d[at], d[at + 1], d[at + 2], d[at + 3]]) as usize }

/// The strings of a GMD (MT Framework message file): the last section of the file, `count` strings ending in NUL.
fn strings(d: &[u8]) -> Option<(&[u8], usize)> {
    if d.len() < 41 || &d[..4] != b"GMD\0" {
        return None;
    }
    let (labels, count, size, name) = (u32_at(d, 20), u32_at(d, 24), u32_at(d, 32), u32_at(d, 36));
    let start = if labels == 0 { 41 + name } else { d.len().checked_sub(size)? };
    (start <= d.len()).then(|| (&d[start..], count))
}

const ARC_GFD: u32 = 0x2d46_2600;
const SYLLABLES: usize = 2350;

/// The patch's codes, in syllable order (KS X 1001), from the font the game draws its text with; empty when that font
/// is not the patch's (no glyph at U+7A00).
fn patch_codes() -> Vec<u32> {
    let path = "nativeNX/arc/resident.arc";
    let Some(head) = read_at(path, 0, Some(12)) else { return Vec::new() };
    if &head[..4] != b"ARC\0" {
        return Vec::new();
    }
    let count = u16::from_le_bytes([head[6], head[7]]) as usize;
    let Some(table) = read_at(path, 12, Some(count * 80)) else { return Vec::new() };
    for e in table.chunks_exact(80) {
        let name = &e[..64];
        let end = name.iter().position(|&b| b == 0).unwrap_or(64);
        if u32_at(e, 64) as u32 != ARC_GFD || !name[..end].to_ascii_lowercase().ends_with(b"font_festa") {
            continue;
        }
        let (csize, usize_, off) = (u32_at(e, 68), u32_at(e, 72) & 0x1fff_ffff, u32_at(e, 76));
        let Some(packed) = read_at(path, off, Some(csize)) else { break };
        let Ok(g) = miniz_oxide::inflate::decompress_to_vec_zlib_with_limit(&packed, usize_.max(1)) else { break };
        if g.len() < 0x48 || &g[..3] != b"GFD" {
            break;
        }
        let (n, name) = (u32_at(&g, 0x1c), u32_at(&g, 0x40));
        let mut base = 0x44 + name + 1;
        if base + n * 20 != g.len() {
            let Some(b) = g.len().checked_sub(n * 20) else { break };
            base = b;
        }
        let codes: Vec<u32> = (0..n).map(|i| u32_at(&g, base + i * 20) as u32).collect();
        let Some(first) = codes.iter().position(|&c| c >= 0x7A00) else { break };
        if codes[first] != 0x7A00 || first + SYLLABLES > codes.len() {
            break;
        }
        return codes[first..first + SYLLABLES].to_vec();
    }
    Vec::new()
}

/// The patch's text as Unicode Hangul; None when it is not Korean (kana or other kanji: the original Japanese).
fn decode(src: &[u8], out: &mut Vec<u8>, map: &[(u32, u16)]) -> bool {
    let Ok(text) = core::str::from_utf8(src) else { return false };
    let mut hangul = false;
    for ch in text.chars() {
        let c = ch as u32;
        if let Ok(i) = map.binary_search_by_key(&c, |&(code, _)| code) {
            hangul = true;
            let mut b = [0; 4];
            let syllable = char::from_u32(ko_ks::SYLLABLES[map[i].1 as usize] as u32).unwrap_or('?');
            out.extend_from_slice(syllable.encode_utf8(&mut b).as_bytes());
            continue;
        }
        let to = match c {
            0x3000 => 0x20,
            0xFF01..=0xFF5E => c - 0xFEE0,
            0x30FB => 0xB7,
            0x30FC => '-' as u32,
            0x3041..=0x30FA | 0x3400..=0x9FFF => return false,
            _ => c,
        };
        let mut b = [0; 4];
        out.extend_from_slice(char::from_u32(to).unwrap_or('?').encode_utf8(&mut b).as_bytes());
    }
    hangul
}

/// The Korean of a GMD's strings these keys want; into `n` when given, else only counted.
fn fill(data: &[u8], want: &[(u16, u16)], map: &[(u32, u16)], mut n: Option<&mut Names>) -> usize {
    let Some((body, count)) = strings(data) else { return 0 };
    let mut scratch = Vec::new();
    let mut korean = 0;
    let mut next = 0;
    for (index, row) in body.split(|&b| b == 0).take(count).enumerate() {
        while next < want.len() && (want[next].0 as usize) < index {
            next += 1;
        }
        if next == want.len() {
            break;
        }
        if want[next].0 as usize != index {
            continue;
        }
        match n.as_deref_mut() {
            Some(n) => {
                let start = n.pool.len();
                if decode(row, &mut n.pool, map) {
                    let len = (n.pool.len() - start) as u32;
                    for &(i, slot) in &want[next..] {
                        if i as usize != index {
                            break;
                        }
                        n.spans[slot as usize] = (start as u32, len);
                        korean += 1;
                    }
                } else {
                    n.pool.truncate(start);
                }
            }
            None => {
                if decode(row, &mut scratch, map) {
                    korean += want[next..].iter().take_while(|&&(i, _)| i as usize == index).count();
                }
                scratch.clear();
            }
        }
    }
    korean
}

fn load(n: &mut Names) {
    n.loaded = true;
    n.spans = alloc::vec![(0, 0); keys::KEYS.len()];
    n.pool = Vec::with_capacity(keys::POOL);
    let mut map: Vec<(u32, u16)> = patch_codes().into_iter().enumerate().map(|(i, c)| (c, i as u16)).collect();
    map.sort_unstable();
    if map.is_empty() {
        info!("Korean: the game's font is not the Korean patch's: names stay in English");
        return;
    }
    let mut files = 0;
    let mut names = 0;
    let mut missing = 0;
    let mut japanese = 0;
    let mut quiet = 0;
    for (f, paths) in keys::FILES.iter().enumerate() {
        let want = keys::BY_FILE[f];
        // The file of the candidates (the patch's GMD, the game's own) with the most of these names in Korean
        let mut best: Option<(usize, Vec<u8>)> = None;
        let mut found = false;
        for p in paths.iter() {
            let Some(data) = read(p) else { continue };
            found = true;
            let korean = fill(&data, want, &map, None);
            if korean > best.as_ref().map_or(0, |b| b.0) {
                best = Some((korean, data));
            }
        }
        if !found {
            if missing < 8 {
                info!("Korean: no {} in the game", paths[0]);
            }
            missing += 1;
            continue;
        }
        files += 1;
        let Some((_, data)) = best else {
            japanese += want.len();
            if quiet < 8 {
                info!("Korean: {} has none of its {} names in Korean", paths[0], want.len());
            }
            quiet += 1;
            continue;
        };
        let korean = fill(&data, want, &map, Some(n));
        names += korean;
        japanese += want.len() - korean;
    }
    info!("Korean: {names} game names from {files} of {} tables ({japanese} not Korean: the game's own text)", keys::FILES.len());
}

/// Korean for `s` when there is: a game name, then our dictionary, then a composite label put together from those
/// (compose); else `s`.
pub fn tr(s: &str) -> &str {
    if !s.bytes().any(|b| b.is_ascii_alphabetic()) {
        return s;
    }
    let n = unsafe { &mut *NAMES.0.get() };
    if !n.loaded {
        load(n);
    }
    if let Some(t) = exact(n, s) {
        return t;
    }
    if let Some(t) = n.composed.get(s) {
        // (an entry is never removed or changed: its text stays where it is)
        return unsafe { &*(t.as_str() as *const str) };
    }
    // (a text already put together in Korean, "여성   검사   HR 1", is not taken apart again)
    let t = if s.chars().any(is_hangul) { None } else { compose(n, s) }.unwrap_or_else(|| String::from(s));
    let t = n.composed.entry(String::from(s)).or_insert(t);
    unsafe { &*(t.as_str() as *const str) }
}

fn exact(n: &Names, s: &str) -> Option<&'static str> {
    if let Ok(i) = keys::KEYS.binary_search(&s) {
        let (start, len) = n.spans[i];
        if len > 0 {
            if let Ok(t) = core::str::from_utf8(&n.pool[start as usize..(start + len) as usize]) {
                // (the pool is only written while loading, before any of it is handed out)
                return Some(unsafe { &*(t as *const str) });
            }
        }
    }
    ko_dict::DICT.binary_search_by(|(k, _)| k.as_bytes().cmp(s.as_bytes())).ok().map(|i| ko_dict::DICT[i].1)
}

/// A part of a composite label: exactly known, or itself composite. None when not all of it is known.
fn part(n: &Names, s: &str) -> Option<String> {
    exact(n, s).map(String::from).or_else(|| compose(n, s))
}

/// Composite labels of the data, from known parts: "Rathalos Cap S LV5", "Basarios / Body Carve",
/// "Break Head G1-2", "HR Rathalos Materials", "Arctic Ridge (N)", "Guild ★13 / Reward Hunt 10", "Thn 40 Def+15",
/// "Rapid 2|Pierce 3|Spread 3*" (bow charge shots).
fn compose(n: &Names, s: &str) -> Option<String> {
    for sep in [" // ", " / "] {
        if s.contains(sep) {
            let mut out = String::new();
            let mut any = false;
            for (i, p) in s.split(sep).enumerate() {
                if i > 0 { out.push_str(sep); }
                match part(n, p) { Some(t) => { out.push_str(&t); any = true; } None => out.push_str(p) }
            }
            return any.then_some(out);
        }
    }
    if let Some((base, level)) = s.rsplit_once(" LV").filter(|(_, l)| !l.is_empty() && l.bytes().all(|b| b.is_ascii_digit())) {
        let mut out = part(n, base)?;
        out.push_str(" LV");
        out.push_str(level);
        return Some(out);
    }
    if let Some((base, range)) = s.rsplit_once(' ').filter(|(_, r)| level_range(r)) {
        let mut out = part(n, base)?;
        out.push(' ');
        out.push_str(range);
        return Some(out);
    }
    if let Some(base) = s.strip_suffix(" (N)") {
        let mut out = part(n, base)?;
        out.push_str(" (밤)");
        return Some(out);
    }
    if let Some(mid) = s.strip_suffix(" Materials") {
        for (rank, ko) in [("G ", "G급 "), ("HR ", "상위 "), ("LR ", "하위 ")] {
            if let Some(name) = mid.strip_prefix(rank) {
                let mut out = String::from(ko);
                let name = part(n, name).unwrap_or_else(|| String::from(name));
                out.push_str(&name);
                if !name.ends_with("소재") { out.push_str(" 소재"); }
                return Some(out);
            }
        }
    }
    if let Some((base, inner)) = s.strip_suffix(')').and_then(|t| t.rsplit_once(" (")) {
        let mut out = part(n, base)?;
        out.push_str(" (");
        out.push_str(&part(n, inner).unwrap_or_else(|| String::from(inner)));
        out.push(')');
        return Some(out);
    }
    if let Some(tree) = s.strip_prefix("A Decoration that boosts ").and_then(|t| t.strip_suffix(" skills.")) {
        let mut out = part(n, tree).unwrap_or_else(|| String::from(tree));
        out.push_str(" 스킬을 강화하는 장식주.");
        return Some(out);
    }
    if s.contains('|') {
        let mut out = String::new();
        for (i, shot) in s.split('|').enumerate() {
            if i > 0 { out.push('|'); }
            let (kind, rest) = shot.split_once(' ').unwrap_or((shot, ""));
            out.push_str(match kind { "Rapid" => "연사", "Pierce" => "관통", "Spread" => "확산", "Heavy" => "강사", k => k });
            if !rest.is_empty() { out.push(' '); out.push_str(rest); }
        }
        return Some(out);
    }
    words(n, s)
}

/// "G1-2", "Lv1-4": a level range after a source label
fn level_range(r: &str) -> bool {
    let digits = r.strip_prefix("Lv").or_else(|| r.strip_prefix('G'));
    digits.is_some_and(|d| d.split_once('-').is_some_and(|(a, b)| !a.is_empty() && !b.is_empty() && a.bytes().chain(b.bytes()).all(|c| c.is_ascii_digit())))
}

/// Word by word, when every word with letters is known ("Guild ★13", "Reward Hunt 10", "Fir 30 +5% Def+20");
/// single capitals (A, B, ...) and numbers stay.
fn words(n: &Names, s: &str) -> Option<String> {
    let mut out = String::new();
    let mut any = false;
    for (i, w) in s.split(' ').enumerate() {
        if i > 0 { out.push(' '); }
        if let Some(v) = w.strip_prefix("Def+") {
            out.push_str("방어+");
            out.push_str(v);
            any = true;
        } else if let Some(t) = exact(n, w) {
            out.push_str(t);
            any = true;
        } else if w.bytes().filter(|b| b.is_ascii_alphabetic()).count() <= 1 || w.starts_with("HP") {
            out.push_str(w);
        } else {
            return None;
        }
    }
    any.then_some(out)
}

pub fn is_hangul(c: char) -> bool { matches!(c as u32, 0x1100..=0x11FF | 0x3131..=0x318E | 0xAC00..=0xD7A3) }

/// `{{English name}}` in a text (the translated skill explanations, tools/kor/skill_effects_ko.py) as the game's name.
pub fn expand(s: &str) -> Cow<'_, str> {
    if !s.contains("{{") {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 64);
    let mut rest = s;
    while let Some(open) = rest.find("{{") {
        out.push_str(&rest[..open]);
        let after = &rest[open + 2..];
        let Some(close) = after.find("}}") else {
            out.push_str(&rest[open..]);
            return Cow::Owned(out);
        };
        out.push_str(tr(&after[..close]));
        rest = &after[close + 2..];
    }
    out.push_str(rest);
    Cow::Owned(out)
}
