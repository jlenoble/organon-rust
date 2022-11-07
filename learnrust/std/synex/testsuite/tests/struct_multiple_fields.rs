#![allow(dead_code)]

use synex_testsuite::ExpectStructMultipleFields;

#[derive(ExpectStructMultipleFields)]
struct Foo {
    bar: String,
    qux: u8,
}