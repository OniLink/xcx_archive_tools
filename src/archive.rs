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

mod arh;
mod ard;

use indicatif::ProgressBar;

use crate::archive::arh::Arh;
use crate::archive::ard::Ard;


pub struct Archive {
  pub header: Arh,
  pub archive: Ard,
}


impl Archive {
  pub fn open_archive( archive_header: &str, archive_data: &str ) -> Archive {
    Archive {
      header: Arh::open_from_file( archive_header ),
      archive: Ard::open_from_file( archive_data ),
    }
  }

  pub fn extract_file( &mut self, filename: &str, directory: &str ) {
    if let Some( fd ) = self.header.find_file_by_name( filename ) {
      let result = self.archive.extract_file( fd, directory );
      match result {
        Ok(_) => {},
        Err(e) => {
          println!( "Could not extract archive file {0} ({1}): {2}", fd.filename, fd.filename_hash, e );
        },
      }
    }
  }

  pub fn extract_files( &mut self, filenames: &[&str], directory: &str ) {
    for filename in filenames {
      self.extract_file( filename, directory );
    }
  }

  pub fn extract_all( &mut self, directory: &str ) {
    println!( "Extracting {0} files", self.header.file_descriptors.len() );
    let progress_bar = ProgressBar::new( self.header.file_descriptors.len() as u64 );

    for file in &mut self.header.file_descriptors {
      let result = self.archive.extract_file( file, directory );
      match result {
        Ok(_) => {},
        Err(e) => {
          println!( "Could not extract archive file {0} ({1}): {2}", file.filename, file.filename_hash, e );
        },
      }

      progress_bar.inc( 1 );
    }

    progress_bar.finish_with_message( "Files extracted" );
  }

  pub fn supply_filenames( &mut self, filenames: &[&str] ) {
    self.header.supply_filenames( filenames );
  }
}
