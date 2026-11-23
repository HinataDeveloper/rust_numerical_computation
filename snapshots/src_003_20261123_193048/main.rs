// main.rs
// Date: Mon Nov 23 2026
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

use numeric_methods::VALUE_ONE;
use numeric_methods::VALUE_TWO;
use numeric_methods::add_checked;
use numeric_methods::add_overflowing;
use numeric_methods::add_saturating;
// use numeric_methods::add_strict; // overflow
// use numeric_methods::add_unchecked; // overflow
use numeric_methods::add_widening;
use numeric_methods::add_wrapping;
use numeric_methods::separator;

fn main() {
    println!("\n");

    println!("{VALUE_ONE} + {VALUE_TWO} by diffrence add methods ...");

    separator("chekced");
    let result1 = add_checked(VALUE_ONE, VALUE_TWO);

    if let Some(val) = result1 {
        println!("value of result is: {val}");
    } else {
        println!("There is no value ...");
    }

    // --------------------------------------------------

    separator("overflowing");
    let (value, ovr) = add_overflowing(VALUE_ONE, VALUE_TWO);

    if ovr {
        println!("Overflow was occurred ...");
        println!("result is: {value} ...");
    } else {
        println!("result is: {value}");
    }

    // --------------------------------------------------

    separator("saturating");
    let result = add_saturating(VALUE_ONE, VALUE_TWO);
    println!("result is: {result}");

    // --------------------------------------------------

    separator("saturating");
    let result = add_saturating(VALUE_ONE, VALUE_TWO);
    println!("result is: {result}");

    // --------------------------------------------------

    // Panics
    separator("strict");
    // let result = add_strict(VALUE_ONE, VALUE_TWO);
    // println!("result is: {result}");
    println!("strict add mehtod panics ...");

    // --------------------------------------------------

    // Undefined behavior
    separator("unchecked (unsafe)");
    // let result = add_unchecked(VALUE_ONE, VALUE_TWO);
    // println!("result is: {result}");
    println!("Encounter with undefied behavior ...");

    // --------------------------------------------------

    separator("widening");
    let result = add_widening(VALUE_ONE, VALUE_TWO);
    println!("result is: {result}");

    // --------------------------------------------------

    separator("wrraping");
    let result = add_wrapping(VALUE_ONE, VALUE_TWO);
    println!("result is: {result}");

    // --------------------------------------------------
    println!("\nThe End ...\n");
}
