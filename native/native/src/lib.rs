mod homes;
mod streets;

use homes::{load_homes, Home};
use rustler::Atom;
use std::{ffi::OsStr, path::PathBuf};

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn nearest_home(longitude: f32, latitude: f32) -> Option<&'static Home> {
    homes::nearest_home(longitude, latitude)
}

fn load<'a>(env: rustler::Env<'a>, args: rustler::Term<'a>) -> bool {
    let key = Atom::from_str(env, "priv_path").unwrap().to_term(env);
    let priv_path = args.map_get(key).unwrap();
    let priv_path = priv_path.into_binary().unwrap().as_slice();
    let priv_path = build_path_buf(priv_path);

    load_homes(priv_path);

    true
}

#[cfg(unix)]
fn build_path_buf(priv_path: &[u8]) -> PathBuf {
    use std::os::unix::prelude::OsStrExt;

    let priv_path = OsStr::from_bytes(priv_path);
    PathBuf::from(priv_path)
}

#[cfg(windows)]
fn build_path_buf(priv_path: &[u8]) -> PathBuf {
    let string_slice = std::str::from_utf8(priv_path).expect("Data is not valid UTF-8, we could decode it without valid UTF-8 requirements but lets not do that for now because its easier this way");
    let priv_path = OsStr::new(string_slice);
    PathBuf::from(priv_path)
}

rustler::init!("Elixir.Native", [add, nearest_home], load = load);
