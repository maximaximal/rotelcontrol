use std::io::prelude::*;
use std::net::TcpStream;

use std::path::PathBuf;

use clap::{arg, command, value_parser, Arg, ArgAction, Command};

mod command;

fn send(s: &mut TcpStream, cmd: String) -> std::io::Result<()> {
    s.write(cmd.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let matches = command!() // requires `cargo` feature
	.arg(Arg::new("address").short('a').long("address")
	     .help("Address of the Rotel A14")
	     .default_value("10.14.21.88:9590"))
	.arg(Arg::new("volume").short('v').long("volume")
	     .help("Target volume")
	     .value_parser(clap::value_parser!(u32).range(0..60)))
	.arg(Arg::new("up").short('u').long("up")
	     .help("Volume up")
	     .action(ArgAction::Count))
	.arg(Arg::new("down").short('d').long("down")
	     .help("Volume up")
	     .action(ArgAction::Count))
	.get_matches();

    let mut s = TcpStream::connect(matches.get_one::<String>("address").unwrap())?;

    let vol = matches.get_one::<u32>("volume");

    let up = matches.get_count("up");
    let down = matches.get_count("down");

    for i in 0 .. up {
	send(&mut s, command::volume_up())?;
    }

    for i in 0 .. down {
	send(&mut s, command::volume_down())?;
    }

    if let Some(v) = vol {
	send(&mut s, command::set_volume(*v))?;
    }
    
    Ok(())
}
