//! Lexing buffers are buffers with a notion of *token*.
//!
//! #Description
//! This crates provides a single struct: `LexBuf` and its associated methods.
//!
//! This struct intends to ease the hand-writting of lexers as it carries a notion
//! of "current token".
//!
//! #Caveat
//!
//! The "current token" may not be larger than 4096 `u8`. If it is, internal functions may panic.
//!
//!


use std::io::Read;

const BUFSIZE: usize = 4096;

/// A `LexBuf` is built upon a type with trait `Read`.
///
/// #Abstract view
/// The user may see a LexBuf as an infinite read-only tape with two pointer on its, *head* and
/// *tail*, delimiting a (current) token [tail,head[.
///
/// The content of the tape is the content of the T : Read it's built upon, with infinite zeroes on
/// the right.
///
/// #Caveats
///
/// As no computer has yet achieved infinite memory, the following limits have to be taken into
/// account:
///
///  1. One can never go back beyond tail : once we have recognized the the current token and
///     **moved on**, it is definitely lost.
///
///  2. Head and tail shall not be distant of more than 4096 cells. If they are, methods calls are
///     likely to panic.

pub struct LexBuf<T: Read> {
    // iner reader upon wich the LexBuf is built
    r: T,
    // internal buffer
    buf: [u8; BUFSIZE],
    // begining of the current token
    tail: usize,
    // next character to be read
    head: usize,
}

impl<T: Read> LexBuf<T> {
    /// `new` takes a reader and consumes it to
    /// build a lexing buffer with an empty *token*.
    pub fn new(r: T) -> LexBuf<T> {
        let mut new_buf = LexBuf {
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

    /// `get` returns the next unread character and moves the head forward, effectively adding the
    /// read character to the current token.
    pub fn get(&mut self) -> u8 {
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

    /// `get_char` behaves like `get`, except that it returns a `char` instead of an `u8` to ease
    /// later matching.
    pub fn get_char(&mut self) -> char {
        self.get() as char
    }

    /// `unget()` moves head backward of 1 cell. Panics if this would bring head behind tail (ie.
    /// if you have made more unget than get since the last time you moved on).
    pub fn unget(&mut self) {
        if self.head <= self.tail {
            panic!("Cannot unget, you ave moved on !")
        }
        self.head -= 1;
    }

    /// `move_on` move tail to head, effectively resetting the current token to the empty one.
    pub fn move_on(&mut self) {
        self.tail = self.head;
    }

    /// `give_up` gives up on the current token and move head back to tail, ie. the `LexBuf`  goes back to the
    /// state it was in after the last `move-on()` (or `new()`).
    pub fn give_up(&mut self) {
        self.head = self.tail;
    }

    /// Get the current token.
    pub fn get_token(&self) -> Vec<u8> {
        self.buf[self.tail..self.head].to_vec()
    }

    /// Get the current token and moves on.
    pub fn validate(&mut self) -> Vec<u8> {
        let res = self.get_token();
        self.move_on();
        res
    }

    /// Shrinks the current token on the left (that is moves tail forward)
    ///
    /// This function panics if the current token size is 0.
    pub fn shrink(&mut self) {
        if self.tail >= self.head {
            panic!("Current token is empty !")
        }
        self.tail += 1;
    }
}
