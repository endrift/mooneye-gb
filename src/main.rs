// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
#[macro_use]
extern crate bitflags;
extern crate crc;
extern crate docopt;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
#[macro_use]
extern crate imgui;
extern crate nalgebra;
extern crate num;
extern crate podio;
extern crate rustc_serialize;
extern crate sdl2;
extern crate time;
extern crate url;

#[cfg(test)]
extern crate quickcheck;

use docopt::Docopt;
use std::path::Path;
use std::process;

use config::{Bootrom, Cartridge};
use frontend::SdlFrontend;

mod config;
mod cpu;
mod emulation;
mod frontend;
mod gameboy;
mod hardware;
mod machine;
mod util;

#[cfg(feature = "acceptance_tests")]
mod acceptance_tests;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = concat!("Mooneye GB v", env!("CARGO_PKG_VERSION"), "

Usage:
  mooneye-gb [options] [<rom>]
  mooneye-gb (-h | --help)

Options:
  -h, --help                   Help
  -b=<file>, --bootrom=<file>  Use a boot ROM
");

#[derive(Debug, RustcDecodable)]
struct Args {
  arg_rom: Option<String>,
  flag_bootrom: Option<String>
}

fn main() {
  let args: Args =
    Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

  let bootrom =
    match args.flag_bootrom {
      Some(path) => Some(Bootrom::from_path(&Path::new(&path)).unwrap_or_else(|err| {
        println!("Failed to read boot rom from \"{}\" ({})", path, err);
        process::exit(1)
      })),
      _ => Bootrom::from_default_bootrom()
    };

  let cartridge =
    args.arg_rom.map(|path| {
      Cartridge::from_path(&Path::new(&path)).unwrap_or_else(|err| {
        println!("Failed to read rom from \"{}\" ({})", path, err);
        process::exit(1)
      })
    });

  let frontend = match SdlFrontend::init() {
    Err(error) => panic!("{}", error),
    Ok(frontend) => frontend
  };

  let result = frontend.main(bootrom, cartridge);
  if let Err(error) = result {
    panic!("{}", error);
  }
}
