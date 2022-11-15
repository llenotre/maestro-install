//! This module implements utility functions.

use std::io::BufRead;
use std::io;
use std::mem::size_of;
use std::process::Command;
use std::process::exit;

/// ioctl macro: TODO doc
macro_rules! ioc {
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		(($a) << 30) | (($b) << 8) | ($c) | (($d) << 16)
	}
}

/// ioctl macro: Read command.
#[macro_export]
macro_rules! ior {
	($a:expr, $b:expr, $c:ty) => {
		ioc!(2, $a, $b, std::mem::size_of::<$c>() as u64)
	}
}

/// ioctl command: Get size of disk in number of sectors.
pub const BLKGETSIZE64: u64 = ior!(0x12, 114, usize);

/// Performs the log2 operatin on the given integer.
///
/// If the result is undefined, the function returns None.
pub fn log2(n: u64) -> Option<u64> {
	let num_bits = (size_of::<u64>() * 8) as u64;

	let n = num_bits - n.leading_zeros() as u64;
	if n > 0 {
		Some(n - 1)
	} else {
		None
	}
}

/// Reads a line from the standard input and returns it.
///
/// If reading fails, the function exits the program.
pub fn read_line() -> String {
	let stdin = io::stdin();

	match stdin.lock().lines().next() {
		Some(Ok(line)) => line,

		Some(Err(_)) => {
			eprintln!("Failed to read line from input");
			exit(1);
		},

		None => exit(0),
	}
}

/// Reboots the system.
/// If the current process doesn't have the permission to reboot the system, the function prints an
/// error, then exits the process.
pub fn reboot() -> ! {
	let _ = Command::new("reboot")
		.arg("now")
		.status();

	eprintln!("Failed to reboot the system. Exiting...");
	exit(1)
}
