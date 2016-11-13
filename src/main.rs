#![allow(unused_variables)]

extern crate clap;

use self::clap::{App, Arg, ArgMatches};
use std::{path, fs};
use std::io::Read;

enum MergeType {
  Override,
  Default,
  Conflict,
  Replace,
}

trait ArgMatchesExt {
  fn merge_config(&self, config: &str, merge_type: MergeType) {
  }
}

impl<'a> ArgMatchesExt for ArgMatches<'a> {
}

trait ArgExt where Self: Sized {
  fn allow_in_config(self, allow: bool) -> Self {
    self
  }
}

impl<'a, 'b> ArgExt for Arg<'a, 'b> {
}

pub fn slurp<P: AsRef<path::Path>>(path: P) -> String {
  let mut file = fs::File::open(path).unwrap_or_else(|err| panic!("slurp {}", err));

  let mut s = String::new();

  if let Err(err) = file.read_to_string(&mut s) {
    panic!("slurp: {}", err)
  }

  s
}

fn main() {
  let matches = App::new("example")
    .arg(Arg::with_name("config")
         .long("config")
         .takes_value(true)
         .allow_in_config(false)
         .help("Read options from <config>"))
    .arg(Arg::with_name("foo")
         .long("foo"))
    .arg(Arg::with_name("bar")
         .takes_value(true)
         .long("bar"))
    .arg(Arg::with_name("baz")
         .takes_value(true)
         .allow_in_config(false)
         .long("bar"))
    .get_matches();

  if let Some(config_path) = matches.value_of("config") {
    let config = slurp(config_path);
    // use config options when both are specified
    matches.merge_config(&config, MergeType::Override);
    // OR use command line options when both are specified
    matches.merge_config(&config, MergeType::Default);
    // OR don't allow options to be specified in both places
    matches.merge_config(&config, MergeType::Conflict);
    // OR ignore options from the command line
    matches.merge_config(&config, MergeType::Replace);
  }

  // use matches as you would normall
}
