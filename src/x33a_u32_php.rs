// New BSD License
//
// Copyright © 2018-present, Michael Cummings <mgcummings@yahoo.com>.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the copyright holder nor the names of its
//       contributors may be used to endorse or promote products derived from
//       this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
use std::hash::Hasher;
use super::HasherU32;

///
/// Implements 32 bit version of one of the original hash functions post by Daniel J. Bernstein but
/// with final OR to set the high bit.
///
/// PHP uses a zero hash value to signal an empty hash that will need to be calculated. To insure no
/// actual hash ends up being zero a final step of binary OR is used to always set the high bit.
///
/// # Examples
///
/// ```rust
/// use std::hash::Hasher;
/// use djb_hash::HasherU32;
/// use djb_hash::x33a_u32_php::*;
/// let input = "Ez";
/// let mut hasher = X33aU32Php::new();
/// hasher.write(&input.as_bytes());
/// assert_eq!(hasher.finish(), 2153345956u64);
/// assert_eq!(hasher.finish_u32(), 2153345956u32);
/// ```
///
pub struct X33aU32Php {
    hash: u32,
}

impl X33aU32Php {
    ///
    /// Creates a new hash using the original 5381 prime number salt value used by DJB.
    ///
    pub fn new() -> Self {
        X33aU32Php { hash: 5381 }
    }
    ///
    /// Creates a new hash using user supplied salt value.
    ///
    /// The supplied salt needs to be a prime number. It should have bits in
    /// more than just the lower 8 bits but setting any bits past half the size
    /// of the hash is of limited use as they are quickly lost during the
    /// multiplication stage for long values and tend to because static for very
    /// short values. Primes between 16 to 32 bits for 64 bit hashes seem to
    /// work best in most cases and between 16 to 24 bits for 32 bit hashes.
    ///
    pub fn new_with_salt(s: u32) -> Self {
        X33aU32Php { hash: s }
    }
}

impl HasherU32 for X33aU32Php {
    fn finish_u32(&self) -> u32 {
        self.hash | 0x80000000u32
    }
}

impl Hasher for X33aU32Php {
    fn finish(&self) -> u64 {
        (self.hash | 0x80000000u32) as u64
    }
    ///
    /// Writes byte slice to hash.
    ///
    /// Does hash * 33 + byte but is implemented as hash << 5 (*32) + hash + byte as this is faster
    /// on most processors vs normal multiplication.
    ///
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 5).wrapping_add(self.hash).wrapping_add(*byte as u32);
        }
    }
}
#[cfg(test)]
mod tests {
    use std::hash::Hasher;
    use super::*;

    #[test]
    fn it_does_hash_correctly() {
        let mut sut = X33aU32Php::new();
        let input = [69, 122];
        sut.write(&input);
        assert_eq!(sut.finish(), 2153345956u64);
        assert_eq!(sut.finish_u32(), 2153345956u32);
        let mut sut = X33aU32Php::new();
        let input = [70, 89];
        sut.write(&input);
        assert_eq!(sut.finish(), 2153345956u64);
        let mut sut = X33aU32Php::new_with_salt(5381);
        let input = [70, 89];
        sut.write(&input);
        assert_eq!(sut.finish(), 2153345956u64);
    }
}
