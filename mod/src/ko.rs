//! Korean fork: the text drawn in Korean.
//!
//! - Game names (items, weapons, armor, skills, ...): read when the utility first draws, from the game's own tables
//!   (the GMDs the game reads, so a Korean patch's when one is installed). The patch writes Hangul as the kanji from
//!   U+7A00 (the n-th KS X 1001 syllable as U+7A00 + n), decoded here. Which GMD string an English name is comes from
//!   ko_keys (tools/kor/keys.py). Nothing of the game's text is in the plugin.
//! - Everything else: ko_dict (tools/kor/ui_ko.tsv), our own translations.
//! A string that is neither (or a name the game's file has no Korean for: an unpatched game) is drawn as it is.
use alloc::vec::Vec;
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

struct Names { loaded: bool, pool: Vec<u8>, spans: Vec<(u32, u32)> }
struct Local(UnsafeCell<Names>);
unsafe impl Sync for Local {}
static NAMES: Local = Local(UnsafeCell::new(Names { loaded: false, pool: Vec::new(), spans: Vec::new() }));

const MAX_FILE: i64 = 8 << 20;

fn read(path: &str) -> Option<Vec<u8>> {
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
    let mut size = 0i64;
    let mut data = Vec::new();
    let mut ok = unsafe { get_file_size(&mut size, file) } == 0 && (41..=MAX_FILE).contains(&size);
    if ok {
        data.resize(size as usize, 0);
        ok = unsafe { read_file(file, 0, data.as_mut_ptr().cast(), data.len()) } == 0;
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

/// The patch's text as Unicode Hangul; None when it is not Korean (kana or other kanji: the original Japanese).
fn decode(src: &[u8], out: &mut Vec<u8>) -> bool {
    let Ok(text) = core::str::from_utf8(src) else { return false };
    let mut hangul = false;
    for ch in text.chars() {
        let c = ch as u32;
        let to = match c {
            0x7A00..=0x832D => { hangul = true; ko_ks::SYLLABLES[(c - 0x7A00) as usize] as u32 }
            0x3000 => 0x20,
            0xFF01..=0xFF5E => c - 0xFEE0,
            0x30FB => 0xB7,
            0x30FC => '-' as u32,
            0x3041..=0x30FA | 0x3400..=0x79FF | 0x832E..=0x9FFF => return false,
            _ => c,
        };
        let mut b = [0; 4];
        out.extend_from_slice(char::from_u32(to).unwrap_or('?').encode_utf8(&mut b).as_bytes());
    }
    hangul
}

fn load(n: &mut Names) {
    n.loaded = true;
    n.spans = alloc::vec![(0, 0); keys::KEYS.len()];
    n.pool = Vec::with_capacity(keys::POOL);
    let mut files = 0;
    let mut names = 0;
    let mut missing = 0;
    for (f, paths) in keys::FILES.iter().enumerate() {
        let Some(data) = paths.iter().find_map(|p| read(p)) else {
            if missing < 8 {
                info!("Korean: no {} in the game", paths[0]);
            }
            missing += 1;
            continue;
        };
        let Some((body, count)) = strings(&data) else { continue };
        files += 1;
        let want = keys::BY_FILE[f];
        let mut next = 0;
        for (index, row) in body.split(|&b| b == 0).take(count).enumerate() {
            while next < want.len() && (want[next].0 as usize) < index {
                next += 1;
            }
            if next == want.len() {
                break;
            }
            if want[next].0 as usize == index {
                let start = n.pool.len();
                if decode(row, &mut n.pool) {
                    for &(i, slot) in &want[next..] {
                        if i as usize != index {
                            break;
                        }
                        n.spans[slot as usize] = (start as u32, (n.pool.len() - start) as u32);
                        names += 1;
                    }
                } else {
                    n.pool.truncate(start);
                }
            }
        }
    }
    info!("Korean: {names} game names from {files} of {} tables", keys::FILES.len());
}

/// Korean for `s` when there is: a game name, then our dictionary; else `s`.
pub fn tr(s: &str) -> &str {
    if !s.bytes().any(|b| b.is_ascii_alphabetic()) {
        return s;
    }
    let n = unsafe { &mut *NAMES.0.get() };
    if !n.loaded {
        load(n);
    }
    if let Ok(i) = keys::KEYS.binary_search(&s) {
        let (start, len) = n.spans[i];
        if len > 0 {
            if let Ok(t) = core::str::from_utf8(&n.pool[start as usize..(start + len) as usize]) {
                // (the pool is only written while loading, before any of it is handed out)
                return unsafe { &*(t as *const str) };
            }
        }
    }
    match ko_dict::DICT.binary_search_by(|(k, _)| k.as_bytes().cmp(s.as_bytes())) {
        Ok(i) => ko_dict::DICT[i].1,
        Err(_) => s,
    }
}

pub fn is_hangul(c: char) -> bool { matches!(c as u32, 0x1100..=0x11FF | 0x3131..=0x318E | 0xAC00..=0xD7A3) }
