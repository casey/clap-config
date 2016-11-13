This might be out of scope of the clap project, but I think it would be pretty neat.

Many programs wind up configuring themselves both from command line options and from a config file. A lot of the time there is a lot of overlap, with options being configurable in both. Other times there are accidental omissions, where something that should be configurable on the command line can only be configured in a config file, or vice versa.

It would be pretty awesome if clap would generate a simple YAML or TOML config parser based on the command line options, and then allow pulling options from that or in addition to the command line.

Lots of things wouldn't make sense, for example subcommands, but the majority of things probably would.

Also, since YAML or TOML are typed languages, type restrictions would be easy to enforce.
