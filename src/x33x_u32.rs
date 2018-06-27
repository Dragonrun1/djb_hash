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
/// Implements 64 bit version of one of the "improved" hash functions post by Daniel J. Bernstein.
///
pub struct X33xU32 {
    hash: u32,
}

impl X33xU32 {
    ///
    /// Creates a new hash using the original 5381 prime number salt value used by DJB.
    ///
    pub fn new() -> Self {
        X33xU32 { hash: 5381 }
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
        X33xU32 { hash: s }
    }
}

impl HasherU32 for X33xU32 {
    fn finish_u32(&self) -> u32 {
        self.hash
    }
}

impl Hasher for X33xU32 {
    fn finish(&self) -> u64 {
        self.hash as u64
    }
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 5).wrapping_add(self.hash) ^ *byte as u32;
        }
    }
}
#[cfg(test)]
mod tests {
    use std::hash::Hasher;
    use super::*;

    #[test]
    fn it_does_hash_correctly() {
        let mut sut = X33xU32::new();
        let input = [69, 122];
        sut.write(&input);
        assert_eq!(sut.finish(), 5861786u64);
        let mut sut = X33xU32::new();
        let input = [70, 89];
        sut.write(&input);
        assert_eq!(sut.finish(), 5861914u64);
    }
}
