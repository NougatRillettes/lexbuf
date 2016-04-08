pub use read_hlbuf::ReadHlBuf;
pub use iter_hlbuf::IterHlBuf;

use std::io::Read;


/// This struct is used as an iterator on a HlBuf.
///
/// The main use case is to resort to std's functions on iterators
/// to recognize tokens.
pub struct HlIter<'a, T: 'a + HlBuf> {
    lb: &'a mut T,
}


/// This trait provides most of the functionalities.
///
/// #Abstract view
/// The user may see a HlBuf as an infinite read-only tape with two pointer on it, *head* and
/// *tail*, delimiting a (current) highlight [tail,head[.
///
///
/// #Caveats
///
/// As no computer has yet achieved infinite memory, the following limits have to be taken into
/// account:
///
///  1. One can never go back beyond tail : once we have finished working the the current highlight and
///     **moved on**, it is definitely lost.
pub trait HlBuf {
    /// The content of the HlBuf.
    type Content;

    /// Returns the next unread item and moves the head forward, effectively adding the
    /// read item to the current highlight.
    fn get(&mut self) -> Self::Content;

    /// Moves head backward of 1 item.
    ///
    /// **Caveat:**  This function panics if doing so would bring head behind tail (ie.
    /// if you have made more `unget()` than `get()` since the last time you moved on).
    fn unget(&mut self);

    /// Moves tail to head, effectively resetting the current highlight to the empty one.
    fn move_on(&mut self);

    /// Gives up on the current highlight and move head back to tail, ie. the `HlBuf`
    /// goes back to the state it was in after the last `move-on()` (or `new()`).
    fn give_up(&mut self);

    /// Get the current highlight.
    fn get_highlight(&self) -> Vec<Self::Content>;

    /// Shrinks the current highlight on the left (that is: moves tail forward)
    ///
    /// This function panics if the current token size is 0.
    fn shrink(&mut self);

    /// Get the current highlight and move on.
    fn validate(&mut self) -> Vec<Self::Content> {
        let res = self.get_highlight();
        self.move_on();
        res
    }

    /// Returns an iterator on the HlBuf, so that the std methods on Iterator may be used.
    fn iter(&mut self) -> HlIter<Self>
        where Self: Sized
    {
        HlIter { lb: self }
    }
}

impl<'a, T> Iterator for HlIter<'a, ReadHlBuf<T>>
    where ReadHlBuf<T>: 'a,
          T: Read
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let c = self.lb.get();
        if c == 0 {
            return None;
        }
        Some(c)
    }
}

impl<'a, I> Iterator for HlIter<'a, IterHlBuf<I>>
    where IterHlBuf<I>: 'a,
          I: Iterator,
          I::Item: Copy,
          I::Item: PartialEq
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let c = self.lb.get();
        if c == self.lb.get_endind() {
            return None;
        }
        Some(c)
    }
}
