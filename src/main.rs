// XCX Archive Tools - extract files from the XCX DE archive format
// Copyright (C) 2025  Violet Kurtz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use clap::{Parser, Subcommand};

use xcx_archive::Archive;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
  /// Archive header file
  #[arg(long)]
  header: String,

  /// Archive data file
  #[arg(long)]
  data: String,

  /// Output directory
  #[arg(long)]
  output_dir: String,

  /// List of filenames
  #[arg(long)]
  filenames: Option<String>,

  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// List all files
  ListAll {},

  /// Extract all files
  ExtractAll {},

  /// Extract one file
  ExtractFile { file: String },

  /// Extract list of files
  ExtractFiles { file_list_file: String },
}


fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
  let file = File::open( filename )?;
  Ok( io::BufReader::new( file ).lines() )
}

fn get_file_list( filename: &String ) -> Vec<String> {
  let mut file_list: Vec<String> = Vec::new();

  // TODO Open file, read line by line into vec
  if let Ok( lines ) = read_lines( filename ) {
    for line in lines.map_while( Result::ok ) {
      file_list.push( line );
    }
  }

  file_list
}

fn main() {
  let args = Args::parse();
  let mut archive = Archive::open_archive( &args.header, &args.data );

  match &args.filenames {
    None => {},
    Some( filenames ) => {
      // Get the list of files as a Vec<&str>
      let file_list: Vec<String> = get_file_list( filenames );

      let files: Vec<&str> = file_list
        .iter()
        .map( |x| &**x )
        .collect();

      archive.supply_filenames( &files );
    },
  }

  match &args.command {
    Commands::ListAll {} => {
      for fd in archive.header.file_descriptors {
        println!( "File {0} (0x{1:0>16X}): {2:X}, {3:X}", fd.filename, fd.filename_hash, fd.file_pos, fd.disk_size );
      }
    },

    Commands::ExtractAll {} => {
      archive.extract_all( &args.output_dir );
    },

    Commands::ExtractFile { file } => {
      archive.extract_file( file, &args.output_dir );
    },

    Commands::ExtractFiles { file_list_file } => {
      // Get the list of files as a Vec<&str>
      let file_list: Vec<String> = get_file_list( file_list_file );
      let files: Vec<&str> = file_list
        .iter()
        .map( |x| &**x )
        .collect();

      archive.extract_files( &files, &args.output_dir );
    },
  }
}
