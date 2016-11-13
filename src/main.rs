extern crate clap;

use self::clap::{App, Arg, ArgGroup, AppSettings};

/*
This is probably out of scope of the project, but I think it would be pretty neat.

Many programs wind up configuring themselves both from command line options and from a config file. A lot of the time there is a lot of overlap, with configuration options being present in both. Also, sometimes projects add more and more command line options, get enough to want to start supporting a config file.

It would be pretty awesome if clap would generate a simple YAML or TOML config parser from your command line options, and then allow pulling options from that instead of or in addition to the command line.
*/

enum MergeType {
  Override,
  Default,
  Conflict,
}

fn main() {
  let matches = App::new("example")
    .arg(Arg::with_name("config")
         .long("config")
         .takes_value(true)
         .help("Read options from <config>"))
    .arg(Arg::with_name("foo")
         .long("foo"))
    .arg(Arg::with_name("bar")
         .takes_value(true)
         .long("foo"))
    // other arguments here
    .get_matches();

  if let Some(config) = matches.value_of("config") {
    // use config options when both are specified
    matches.merge_config_file(config, MergeType::Override);
    // OR use command line options when both are specified
    matches.merge_config_file(config, MergeType::Default);
    // OR don't allow options to be specified in both places
    matches.merge_config_file(config, MergeType::Conflict);
  }

  // use matches as normally
}
