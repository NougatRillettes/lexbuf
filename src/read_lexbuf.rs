use lexbuf::LexBuf;
use std::io::Read;

const BUFSIZE: usize = 4096;

/// A `ReadLexBuf` is a `LexBuf` built upon a type with trait `Read`.
///
/// It buffers the read and provides the highlighting functionalities.
///
/// #Caveats
///
///  Head and tail shall not be distant of more than 4096 cells. If they are, methods calls are
///  likely to panic.

pub struct ReadLexBuf<T: Read> {
    // iner reader upon wich the ReadLexBuf is built
    r: T,
    // internal buffer
    buf: [u8; BUFSIZE],
    // begining of the current token
    tail: usize,
    // next character to be read
    head: usize,
}

impl<T: Read> ReadLexBuf<T> {
    /// `new` takes a reader and consumes it to
    /// build a lexing buffer with an empty *token*.
    pub fn new(r: T) -> ReadLexBuf<T> {
        let mut new_buf = ReadLexBuf {
            r: r,
            tail: 0,
            head: 0,
            buf: [0; BUFSIZE],
        };
        new_buf.fetch();
        new_buf
    }

    // internal function used to bufferize new data
    fn fetch(&mut self) {
        let keep_size = self.head - self.tail;
        if keep_size == BUFSIZE {
            panic!("Current token is longer than buffer");
        }
        let tmp_buf = &self.buf[self.tail..self.head].to_vec();
        &mut self.buf[0..keep_size].clone_from_slice(tmp_buf);
        let n = self.r
                    .read(&mut self.buf[keep_size..])
                    .unwrap();
        if n < BUFSIZE - keep_size {
            for i in &mut self.buf[keep_size + n..] {
                *i = 0;
            }
        }
        self.head -= self.tail;
        self.tail = 0;
    }

    /// `get_char` behaves like `get`, except that it returns a `char` instead of an `u8` to ease
    /// later matching.
    pub fn get_char(&mut self) -> char {
        self.get() as char
    }
}

impl<T: Read> LexBuf for ReadLexBuf<T> {
    fn get(&mut self) -> u8 {
        match self.buf.get(self.head) {
            Some(&c) => {
                self.head += 1;
                c
            }
            None => {
                self.fetch();
                self.get()
            }
        }
    }

    type Content = u8;

    fn unget(&mut self) {
        if self.head <= self.tail {
            panic!("Cannot unget, you ave moved on !")
        }
        self.head -= 1;
    }

    fn move_on(&mut self) {
        self.tail = self.head;
    }

    fn give_up(&mut self) {
        self.head = self.tail;
    }

    fn get_highlight(&self) -> Vec<u8> {
        self.buf[self.tail..self.head].to_vec()
    }

    fn validate(&mut self) -> Vec<u8> {
        let res = self.get_highlight();
        self.move_on();
        res
    }

    fn shrink(&mut self) {
        if self.tail >= self.head {
            panic!("Current token is empty !")
        }
        self.tail += 1;
    }
}
