// main.rs
// Date: Tue Nom 24 2026
// Educational and Practice Rust Programming Language Code

// Project: Numeric Data Type
// Goal: ...
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

fn main() {
    println!("\n");

    let number_one: i32 = 1379;
    let number_two: u64 = 9731;

    let result = add1(number_one, number_two);
    println!("add1: value of result is: {result}\n");

    // -------------------------------------------------------

    let result = add2(number_one, number_two);
    let Some(result) = result else {
        println!("Loss sign occurrence ...");
        return;
    };
    println!("add2: value of result is: {result}\n");

    // -------------------------------------------------------

    let result = add3(number_one, number_two);
    let Some(result) = result else {
        println!("Loss sign occurrence ...");
        return;
    };
    println!("add3: value of result is: {result}\n");

    // -------------------------------------------------------

    println!("\nThe End ...\n");
}

#[allow(dead_code)]
#[must_use]
const fn add1(one: i32, two: u64) -> u64 {
    (one as u64) + two
}

// ----------------------------

#[allow(dead_code)]
#[must_use]
const fn add2(one: i32, two: u64) -> Option<u64> {
    if one < 0 {
        None
    } else {
        Some((one as u64) + two)
    }
}

// ---------------------------

#[allow(dead_code)]
#[must_use]
const fn add3(one: i32, two: u64) -> Option<u64> {
    two.checked_add_signed(one as i64)
}
