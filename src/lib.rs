// Copyright 2017 pdb Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


//! The `pdb` create parses Microsoft PDB (Program Database) files. PDB files contain debugging
//! information produced by most compilers that target Windows, including information about symbols,
//! types, modules, and so on.
//!
//! # Usage
//!
//! PDB files are accessed via the [`pdb::PDB` object](struct.PDB.html).
//!
//! # Example
//!
//! ```
//! # use pdb::FallibleIterator;
//! #
//! # fn test() -> pdb::Result<usize> {
//! let file = std::fs::File::open("fixtures/self/foo.pdb")?;
//! let mut pdb = pdb::PDB::open(file)?;
//!
//! let symbol_table = pdb.global_symbols()?;
//!
//! # let mut count: usize = 0;
//! let mut symbols = symbol_table.iter();
//! while let Some(symbol) = symbols.next()? {
//!     match symbol.parse() {
//!     	Ok(pdb::SymbolData::PublicSymbol{
//!     		function: true,
//!     		segment,
//!     		offset,
//!     		..
//!     	}) => {
//!     		// we found the location of a function!
//!     		println!("{:x}:{:08x} is {}", segment, offset, symbol.name()?);
//!             # count += 1;
//!     	}
//!     	_ => {}
//!     }
//! }
//!
//! # Ok(count)
//! # }
//! # assert!(test().expect("test") > 2000);
//! ```

extern crate byteorder;
extern crate fallible_iterator;

// modules
mod common;
mod dbi;
mod msf;
mod pdb;
mod source;
mod symbol;
mod tpi;

// exports
pub use common::{Error,Result,TypeIndex,RawString};
pub use dbi::{DebugInformation};
pub use pdb::PDB;
pub use source::*;
pub use symbol::{SymbolTable,Symbol,SymbolData,SymbolIter};
pub use tpi::{Type,TypeFinder,TypeInformation,TypeIter,TypeData};
pub use tpi::{ClassKind,EnumValue,FieldAttributes,FunctionAttributes,MethodListEntry,Indirection,PrimitiveType};

// re-export FallibleIterator for convenience
#[doc(no_inline)]
pub use fallible_iterator::FallibleIterator;
