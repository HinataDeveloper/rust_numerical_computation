// lib.rs
// Date: Mon Nov 23 2026
// Educational and Practice Rust Programming Language Code

// Dependency: ...

// Kernel Version: 7.3.4-300.fc45.x86_64

// rustc 1.102.0-nightly (5df8c02af 2026-11-16)
// binary: rustc
// commit-hash: 5df8c02afdf6b882bc301442cea70f026e3d3974
// commit-date: 2026-11-16
// host: x86_64-unknown-linux-gnu
// release: 1.102.0-nightly
// LLVM version: 23.1.5

// cargo 1.102.0-nightly (ede6cefea 2026-11-11)
// release: 1.102.0-nightly
// commit-hash: ede6cefea9731cb595942338824c2f9a610b8189
// commit-date: 2026-11-11
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 45.0.0 [64-bit]

pub const VALUE_ONE: i32 = 1_570_000_001;
pub const VALUE_TWO: i32 = 1_570_000_001;

// unstable library feature
#[allow(dead_code)]
#[must_use]
pub const fn sub_borrwing(one: i32, two: i32) -> i32 {
    one + two
}

// unstable library feature
#[allow(dead_code)]
#[must_use]
pub const fn add_carring(one: i32, two: i32) -> i32 {
    one + two
}

#[allow(dead_code)]
#[must_use]
pub const fn add_checked(one: i32, two: i32) -> Option<i32> {
    one.checked_add(two)
}

#[allow(dead_code)]
#[must_use]
pub const fn add_overflowing(one: i32, two: i32) -> (i32, bool) {
    one.overflowing_add(two)
}

#[allow(dead_code)]
#[must_use]
pub const fn add_saturating(one: i32, two: i32) -> i32 {
    one.saturating_add(two)
}

#[allow(dead_code)]
#[must_use]
pub const fn add_strict(one: i32, two: i32) -> i32 {
    one.strict_add(two)
}

#[allow(dead_code)]
#[must_use]
pub const fn add_unchecked(one: i32, two: i32) -> i32 {
    unsafe { one.unchecked_add(two) }
}

#[allow(dead_code)]
#[must_use]
pub const fn add_widening(one: i32, two: i32) -> i32 {
    one.wrapping_add(two)
}

#[allow(dead_code)]
#[must_use]
pub const fn add_wrapping(one: i32, two: i32) -> i32 {
    one.wrapping_add(two)
}

pub fn separator(name: &str) {
    println!("\n~~~~ {name} ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
}
