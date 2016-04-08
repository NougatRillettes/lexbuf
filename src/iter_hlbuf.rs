use hlbuf::HlBuf;

/// An `IterHlBuf` is a `HlBuf` built upon a type with the Iterator trait.
///
/// Its internal buffer is not of a limited size.
pub struct IterHlBuf<I>
    where I: Iterator,
          I::Item: Copy
{
    // highlight length
    hlen: usize,
    // highlight offset
    hoffset: usize,
    // buffer
    buf: Vec<I::Item>,
    // end indicator
    end_ind: I::Item,
    // inner iterator
    iter: I,
}

impl<I> IterHlBuf<I>
    where I: Iterator,
          I::Item: Copy
{
    /// Builds a new `IterHlBuf` upon `iter` with end indicator `ind`.
    ///
    /// The end indicator is written on the tape when the underlying interator `next()` method
    /// returns `None`.
    pub fn new(iter: I, ind: I::Item) -> IterHlBuf<I> {
        IterHlBuf {
            hlen: 0,
            hoffset: 0,
            buf: vec![],
            end_ind: ind,
            iter: iter,
        }
    }

    pub fn get_endind(&self) -> I::Item {
        self.end_ind
    }
}

impl<I> HlBuf for IterHlBuf<I>
    where I: Iterator,
          I::Item: Copy
{
    type Content = I::Item;

    fn get(&mut self) -> I::Item {
        if self.hoffset + self.hlen == self.buf.len() {
            let x = {
                let n = self.iter.next();
                match n {
                    None => self.end_ind,
                    Some(x) => x,
                }
            };
            self.buf.push(x);
            self.hlen += 1;
            x
        } else {
            self.hlen += 1;
            self.buf[self.hoffset + self.hlen - 1]
        }
    }

    fn unget(&mut self) {
        if self.hlen == 0 {
            panic!("Cannot unget, you ave moved on !")
        }
        self.hlen -= 1;
    }

    fn move_on(&mut self) {
        self.hlen = 0;
        self.hoffset = 0;
        self.buf.clear();
    }

    fn give_up(&mut self) {
        self.hlen = 0;
    }

    fn get_highlight(&self) -> Vec<I::Item> {
        self.buf[self.hoffset..self.hlen + self.hoffset].to_vec()
    }

    fn validate(&mut self) -> Vec<I::Item> {
        let res = self.get_highlight();
        self.move_on();
        res
    }

    fn shrink(&mut self) {
        if self.hlen == 0 {
            panic!("Current token is empty !")
        }
        self.hlen -= 1;
        self.hoffset += 1;
    }
}
