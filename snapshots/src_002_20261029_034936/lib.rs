// lib.rs
// Date: Thu Oct 29 2026
// Educational and Practice Rust Programming Language Code

// Dependency: Without any dependency

// rustc 1.101.0-nightly (16d1ffac1 2026-10-26)
// binary: rustc
// commit-hash: 16d1ffac17ad8b739e32483ac2e65fe7e1804977
// commit-date: 2026-10-26
// host: x86_64-unknown-linux-gnu
// release: 1.101.0-nightly
// LLVM version: 23.1.4

// cargo 1.101.0-nightly (cad07cf8e 2026-10-23)
// release: 1.101.0-nightly
// commit-hash: cad07cf8e597397fff5d0f6cb79c9f870c0b1dd2
// commit-date: 2026-10-23
// host: x86_64-unknown-linux-gnu
// libgit2: 1.9.6 (sys:0.21.0 vendored)
// libcurl: 8.21.0-DEV (sys:0.4.90+curl-8.21.0 vendored ssl:OpenSSL/3.6.3)
// ssl: OpenSSL 3.6.3 9 Jun 2026
// os: Fedora 44.0.0 [64-bit]

// Kernel Version: 7.2.13-200.fc44.x86_64
// Firmware Version: 71CN51WW(V1.21)

use std::fmt::Display;

#[derive(Debug, Default)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub const fn set_width(&mut self, with: f64) {
        self.width = with;
    }

    pub const fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    #[must_use]
    pub const fn width(&self) -> f64 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> f64 {
        self.height
    }

    #[must_use]
    pub const fn area(&self) -> f64 {
        self.width * self.height
    }

    #[must_use]
    pub const fn perimeter(&self) -> f64 {
        // (self.width * 2.0) + (self.height * 2.0)
        self.height.mul_add(2.0, self.width * 2.0)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[width: {}, height:{}, area:{}, perimeter:{}]",
            self.width(),
            self.height(),
            self.area(),
            self.perimeter()
        )
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl PartialOrd for Rectangle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.width.partial_cmp(&other.width) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.height.partial_cmp(&other.height)
    }
}