#![feature(test, plugin)]
#![plugin(json_str)]

extern crate serde;
extern crate serde_json;
extern crate test;
extern crate chrono;
extern crate elastic_types;

use elastic_types::date::prelude::*;
use ::date_fixtures::*;

use test::Bencher;

#[bench]
fn parse_date_from_format(b: &mut Bencher) {
	b.iter(|| {
		ElasticDate::<BasicDateTime>::parse("20150620T134501.034Z").unwrap()
	});
}

#[bench]
fn parse_date_to_string(b: &mut Bencher) {
	let dt: ElasticDate<DefaultFormat> = ElasticDate::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn parse_date_from_epoch(b: &mut Bencher) {
	b.iter(|| {
		ElasticDate::<EpochMillis>::parse("1435935302478").unwrap()
	});
}

#[bench]
fn parse_date_to_epoch(b: &mut Bencher) {
	let dt = ElasticDate::<EpochMillis>::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn get_date_fmt_vec(b: &mut Bencher) {
	b.iter(|| {
		BasicDateTime::fmt()
	});
}

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyDateMapping::<DefaultFormat>::default();
		serde_json::to_string(&mapping).unwrap()
	});
}
