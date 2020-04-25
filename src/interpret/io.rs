use std::io;
use std::io::prelude::*;

//trait RevRead;
//trait RevWrite;
//trait RevSeek;
//struct RevFile;

/** A handle to a reversible standard output stream.

To allow retrieving data from stdout (e.g. when backtracking), a backup buffer
is kept of all the data that was passed.
*/
#[derive(Debug)]
pub struct RevStdout {
	stdout: io::Stdout,
	history: Vec<u8>,
}

impl RevStdout {
	pub fn new() -> Self {
		RevStdout {
			stdout: io::stdout(),
			history: Vec::new(),
		}
	}
	
	pub fn unwrite(&mut self, buf: &[u8]) {
		assert!(self.history.ends_with(buf));
		let new_len = self.history.len() - buf.len();
		self.history.truncate(new_len);
	}
	
	/// When this function is called, all data will be lost, and we won't be
	/// able to go any further in reverse if something goes wrong.
	// + Should stdout be written to immediately? or on program close? or have a 
	//   flush procedure that can be called?
	pub fn reset(&mut self) {
		self.history.clear();
	}
}

impl Write for RevStdout {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.history.copy_from_slice(buf);
		self.stdout.write(buf)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.stdout.flush()
	}
}



/** A handle to a reversible standard input stream.

To allow pushing data to stdin (e.g. when backtracking), a backup buffer is kept
of all the data that was passed back, which is then reused when going forward
again.
*/
#[derive(Debug)]
pub struct RevStdin {
	stdin: io::Stdin,
	queue: Vec<u8>,
}

impl RevStdin {
	pub fn new() -> Self {
		RevStdin {
			stdin: io::stdin(),
			queue: Vec::new(),
		}
	}
	
	pub fn unread(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		unimplemented!()
	}
	
	/// When this function is called, all data will be lost. TODO finish
	pub fn reset(&mut self) {
		self.queue.clear();
	}
}

impl Read for RevStdin {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		if self.queue.is_empty() {
			self.stdin.read(buf)
		} else {
			self.queue.as_slice().read(buf)
		}
	}
}
