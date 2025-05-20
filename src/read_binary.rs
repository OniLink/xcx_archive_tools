// Binary File Reading Extensions for Rust
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

use std::io;
use std::io::Read;

pub trait ReadBinary {
  fn read_le_u32( &mut self, buf: &mut u32 ) -> io::Result<()>;
  fn read_le_u64( &mut self, buf: &mut u64 ) -> io::Result<()>;
}

impl<T: Read> ReadBinary for T {
  fn read_le_u32( &mut self, buf: &mut u32 ) -> io::Result<()> {
    let mut buffer: [u8; 4] = [0; 4];
    self.read_exact( &mut buffer )?;
    *buf = u32::from_le_bytes( buffer );
    Ok(())
  }

  fn read_le_u64( &mut self, buf: &mut u64 ) -> io::Result<()> {
    let mut buffer: [u8; 8] = [0; 8];
    self.read_exact( &mut buffer )?;
    *buf = u64::from_le_bytes( buffer );
    Ok(())
  }
}
