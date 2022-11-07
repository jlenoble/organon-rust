#![allow(dead_code)]

use synex_testsuite::ExpectStructSingleField;

#[derive(ExpectStructSingleField)]
struct Foo {
    bar: String,
}