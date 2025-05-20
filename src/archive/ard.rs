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
use std::io::{Read, Seek, Write};

use crate::archive::arh::ArhFileDescriptor;

pub struct Ard {
  filename: String,
}


impl Ard {
  pub fn create() -> Ard {
    Ard {
      filename: String::from( "" ),
    }
  }

  pub fn open_from_file( data_filename: &str ) -> Ard {
    Ard {
      filename: data_filename.to_string(),
    }
  }

  pub fn extract_file( &self, file_descriptor: &ArhFileDescriptor, directory: &str ) -> io::Result<()> {
    // Open the archive file and find our specific file in it
    let filename = &self.filename;
    let archive_file = File::open( filename.clone() )?;
    let mut reader = io::BufReader::new( archive_file );
    reader.seek( io::SeekFrom::Start( file_descriptor.file_pos ) )?;

    // If no supplied filename in the descriptor, then use the hash + magic
    let mut filename = file_descriptor.filename.clone();
    if filename.is_empty() {
      filename = format!( "{0:X}", file_descriptor.filename_hash );
    }

    // Copy file to destination
    let path_str = format!( "{directory}/{filename}");
    let path = std::path::Path::new( &path_str );
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all( prefix )?;

    let output_file = File::create( format!( "{directory}/{filename}" ) )?;
    let mut output_writer = io::BufWriter::new( output_file );
    let mut taker = reader.take( file_descriptor.disk_size.into() );
    std::io::copy( &mut taker, &mut output_writer )?;
    output_writer.flush()?;

    Ok(())
  }
}
