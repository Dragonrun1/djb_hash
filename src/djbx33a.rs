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
pub struct Djbx33a {
    hash: u64,
}
impl Djbx33a {
    pub fn new() -> Self {
        Djbx33a { hash: 5381 }
    }
}
impl Hasher for Djbx33a {
    fn finish(&self) -> u64 {
        self.hash
    }
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 5).wrapping_add(self.hash).wrapping_add(*byte as u64);
        }
    }
}
#[cfg(test)]
mod tests {
    use std::hash::Hasher;
    use super::*;
    #[test]
    fn it_does_hash_correctly() {
        let mut sut = Djbx33a::new();
        let input = [69, 122];
        sut.write(&input);
        assert_eq!(sut.finish(), 5862308u64);
        let mut sut = Djbx33a::new();
        let input = [70, 89];
        sut.write(&input);
        assert_eq!(sut.finish(), 5862308u64);
    }
}
