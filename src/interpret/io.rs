use std::io;
use std::io::prelude::*;

//trait RevRead;
//trait RevWrite;
//trait RevSeek;
//struct RevFile;

/** A handle to a reversible standard output stream.

To allow retrieving data from stdout (e.g. when backtracking or going in
reverse), a backup buffer is kept of all the data that was passed. When
"unwriting", data is drawn from the end of the backup buffer.
*/
#[derive(Debug)]
pub struct RevStdout {
	careful: bool,
	stdout: io::Stdout,
	history: Vec<u8>,
}

impl RevStdout {
	pub fn new(careful: bool) -> Self {
		RevStdout {
			// TODO make `careful` useful
			careful,
			stdout: io::stdout(),
			history: Vec::new(),
		}
	}
	
	pub fn unwrite(&mut self, bytes_read: usize) -> Vec<u8> {
		let len = self.history.len() - bytes_read;
		self.history.split_off(len)
	}
	
	/// When this function is called, all data will be lost, and we won't be
	/// able to go any further in reverse if something goes wrong.
	pub fn reset(&mut self) {
		self.history.clear();
	}
}

impl Write for RevStdout {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let bytes_read = self.stdout.write(buf)?;
		self.history.extend_from_slice(&buf[..bytes_read]);
		Ok(bytes_read)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.stdout.flush()
	}
}

/*
/** A handle to a reversible standard error stream.

To allow retrieving data from stderr (e.g. when backtracking), a backup buffer
is kept of all the data that was passed.
*/
#[derive(Debug)]
pub struct RevStderr {
	stderr: io::Stderr,
	history: Vec<u8>,
}

impl RevStderr {
	pub fn new() -> Self {
		RevStderr {
			stderr: io::stderr(),
			history: Vec::new(),
		}
	}
	
	pub fn unwrite(&mut self, buf: &[u8], bytes_read: usize) {
		assert!(self.history.ends_with(&buf[..bytes_read]));
		let new_len = self.history.len() - bytes_read;
		self.history.truncate(new_len);
	}
	
	/// When this function is called, all data will be lost, and we won't be
	/// able to go any further in reverse if something goes wrong.
	pub fn reset(&mut self) {
		self.history.clear();
	}
}

impl Write for RevStderr {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let bytes_read = self.stderr.write(buf)?;
		self.history.extend_from_slice(&buf[..bytes_read]);
		Ok(bytes_read)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.stderr.flush()
	}
}


*/
/** A handle to a reversible standard input stream.

To allow pushing data to stdin (e.g. when backtracking), a backup buffer is kept
of all the data that was passed back, which is then reused when going forward
again.
*/
#[derive(Debug)]
pub struct RevStdin {
	stdin: io::Stdin,
	buffer: Vec<u8>,
}

impl RevStdin {
	pub fn new() -> Self {
		RevStdin {
			stdin: io::stdin(),
			buffer: Vec::new(),
		}
	}
	
	// We unread by prepending to the internal buffer.
	pub fn unread(&mut self, buf: &mut [u8], bytes_read: usize) {
		let mut new_buf = buf[..bytes_read].to_vec();
		new_buf.append(&mut self.buffer);
		self.buffer = new_buf;
	}
	
	/// When this function is called, all data will be lost. TODO finish
	pub fn reset(&mut self) {
		self.buffer.clear();
	}
}

impl Read for RevStdin {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		if self.buffer.is_empty() {
			self.stdin.read(buf)
		} else {
			let bytes_read = self.buffer.as_slice().read(buf)?;
			self.buffer.drain(..bytes_read);
			Ok(bytes_read)
		}
	}
}
