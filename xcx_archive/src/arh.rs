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

// ARH Format
//  Header:
//   4 byte: magic ("arh2")
//   4 byte LE: file count
//   4 byte LE: ???
//   4 byte LE: ???
//  Entries:
//   8 byte LE: xxhash64 of filename
//   4 byte LE: disk filesize
//   4 byte LE: uncompressed filesize OR ZERO IF UNCOMPRESSED ON-DISK

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;

use indicatif::ProgressBar;
use xxhash_rust::xxh64::xxh64;

use read_binary::ReadBinary;


pub struct ArhFileDescriptor {
  pub filename_hash: u64,
  pub disk_size: u32,
  pub file_size: u32,
  pub file_pos: u64,
  pub filename: String,
}

pub struct Arh {
  pub magic: u32,
  pub file_count: u32,
  pub alignment: u32,
  unk_2: u32,

  pub file_descriptors: Vec<ArhFileDescriptor>,
}


impl Arh {
  pub fn create() -> Arh {
    Arh {
      magic: 0,
      file_count: 0,
      alignment: 0,
      unk_2: 0,
      file_descriptors: Vec::new(),
    }
  }

  pub fn open_from_file( filename: &str ) -> Arh {
    let mut archive = Arh::create();
    let err = archive.read( filename );
    match err {
      Ok(_) => {},
      Err(e) => {
        println!( "Could not read archive header: {e}" );
      },
    };

    archive
  }

  pub fn read( &mut self, filename: &str ) -> io::Result<()> {
    self.file_descriptors = Vec::new();
    let file = File::open( filename )?;
    let mut reader = BufReader::new( file );

    // Read the header
    reader.read_le_u32( &mut self.magic )?;
    reader.read_le_u32( &mut self.file_count )?;
    reader.read_le_u32( &mut self.alignment )?;
    reader.read_le_u32( &mut self.unk_2 )?;

    // Read the file entries
    let mut cur_pos: u64 = 0;
    for _ in 0..self.file_count {
      let mut next_filename_hash: u64 = 0;
      let mut next_disk_size: u32 = 0;
      let mut next_file_size: u32 = 0;
      reader.read_le_u64( &mut next_filename_hash )?;
      reader.read_le_u32( &mut next_disk_size )?;
      reader.read_le_u32( &mut next_file_size )?;

      let fd = ArhFileDescriptor {
        filename_hash: next_filename_hash,
        disk_size: next_disk_size,
        file_size: next_file_size,
        file_pos: cur_pos,
        filename: String::from( "" ),
      };

      let offset: u32 = ( next_disk_size + self.alignment - 1 ) & ( !self.alignment + 1 );
      cur_pos += offset as u64;
      self.file_descriptors.push( fd );
    }

    Ok(())
  }

  pub fn find_file_by_name( &mut self, filename: &str ) -> Option<&ArhFileDescriptor> {
    let hash = xxh64( filename.as_bytes(), 0 );
    for fd in &mut self.file_descriptors {
      if fd.filename == filename {
        return Some( fd );
      }

      if fd.filename_hash == hash {
        fd.filename = filename.to_string();
        return Some( fd );
      }
    }

    None
  }

  pub fn supply_filenames( &mut self, filenames: &[&str] ) {
    println!( "Supplying {0} filenames", filenames.len() );

    let mut hashes = HashMap::new();

    for filename in filenames {
      hashes.insert( xxh64( filename.as_bytes(), 0 ), *filename );
    }

    let progress_bar = ProgressBar::new( filenames.len() as u64 );

    for fd in &mut self.file_descriptors {
      if hashes.contains_key( &fd.filename_hash ) {
        fd.filename = String::from( *hashes.get( &fd.filename_hash ).unwrap() );
        progress_bar.inc( 1 );
        continue; // A match means we can just skip the rest of the files since filenames are unique
      }
    }

    progress_bar.finish_with_message( "Filenames supplied" );
  }
}
