#![cfg(test)]

extern crate test;
use test::Bencher;

use super::routes;
use super::routes2;

use wayfinder::Match;

#[bench]
fn bench_old(b: &mut Bencher) {
    let route = test::black_box("/people/12345678901234567890123456789012/edit/");
    b.iter(|| {
        routes2::routes::match_route(route, wayfinder::Method::Get)
    })
}

#[bench]
fn bench_new(b: &mut Bencher) {
    let route = test::black_box("/people/12345678901234567890123456789012/edit/");
    b.iter(|| {
        routes::routes::match_route(route, wayfinder::Method::Get)
    })
}

#[test]
fn test_old() {
    let route = test::black_box("/people/12345678901234567890123456789012/edit/");
    match routes2::routes::match_route(route, wayfinder::Method::Get) {
        Ok(Match::Route(routes2::routes::Route::People(routes2::routes::people::Route::Edit(_)))) => {}
        _ => assert!(false),
    }
}

#[test]
fn test_new() {
    let route = test::black_box("/people/12345678901234567890123456789012/edit/");
    match routes::routes::match_route(route, wayfinder::Method::Get) {
        Ok(Match::Route(routes::routes::Route::People(routes::routes::people::Route::Edit(_)))) => {}
        _ => assert!(false),
    }
}

/*
#[bench]
fn iterator(b: &mut Bencher) {
    let n = test::black_box(100000);
    let haystack = test::black_box(&b"foobar"[..]);

    b.iter(|| (0..n).fold(0, |k, _| is_foobar_iter(haystack, k)));
}

#[bench]
fn matching(b: &mut Bencher) {
    let n = test::black_box(100000);
    let haystack = test::black_box(&b"foobar"[..]);

    b.iter(|| (0..n).fold(0, |k, _| is_foobar_matching(haystack, k)));
}

#[bench]
fn equals(b: &mut Bencher) {
    let n = test::black_box(100000);
    let haystack = test::black_box(&b"foobar"[..]);

    b.iter(|| (0..n).fold(0, |k, _| is_foobar_equals(haystack, k)));
}

fn is_foobar_equals(v: &[u8], n: u32) -> u32 {
    test::black_box(v.into_iter());
    if v == b"foobar" { n } else { 0 }
}

fn is_foobar_matching(v: &[u8], n: u32) -> u32 {
    test::black_box(v.into_iter());
    match v {
        b"foobar" => n,
        _ => 0,
    }
}

fn is_foobar_iter(v:&[u8], n: u32) -> u32 {
    test::black_box(v.into_iter());
    let mut v = v.into_iter();
    match v.next() {
        Some(b'f') => {},
        _ => return 0,
    }
    match v.next() {
        Some(b'o') => {},
        _ => return 0,
    }
    match v.next() {
        Some(b'o') => {},
        _ => return 0,
    }
    match v.next() {
        Some(b'b') => {},
        _ => return 0,
    }
    match v.next() {
        Some(b'a') => {},
        _ => return 0,
    }
    match v.next() {
        Some(b'r') => {},
        _ => return 0,
    }
    match v.next() {
        None => n,
        _ => 0,
    }
}
*/
