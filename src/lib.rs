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
//!
//! This library is a collection of simple and fast hash functions based on
//! designs done by Daniel J. Bernstein. Since these hash functions are very
//! simple they have several known limitations and should be use with
//! ___extreme caution___ if used for things like key hashes in HashMap, etc.
//! Most of them are venerable to key collisions which can lead to DOS attacks
//! if exposed in any way to external bad actors.
//!
//! After all that being said, many other languages, like Java, JS, Python,
//! and PHP, have use versions of them internally and it was felt that there
//! would be some utility for Rust to have the hash functions available when
//! doing interoperable code in these cases.
//!
//! ## Understanding hash names
//!
//! The hash names are based on the mathematical functions they use to create
//! hashes. I'll start with breaking down one here: "X33a". The "X##" is the
//! multiplier stage, so before appending the next byte of data to be hashed the
//! existing hash value will be multiplied by the number given which in this
//! case is 33. This number will always be prime and to make the calculation as
//! fast as possible only primes that are binary multiples + or - one are used.
//! The reason is that actual multiplication is slow but bit shifting which act
//! like multiplying in binary and addition or subtraction are much faster.
//! For 33 it is convert to a shift of 5 (times 32) and then add the original
//! number to equal the final 33.
//!
//! Next there is the "a" part. The "a" stand for "add". After multiplying the
//! running hash total as given above the next byte to be hashed is added to it
//! to form the new hash total. Some hashes instead of adding use XOR
//! (Exclusive OR) and would have an "x" where the "a" is in example I've been
//! using. Using XOR usually does a better job of distributing the effect of the
//! new byte across more of the hash bits but the effect is less noticeable when
//! using single byte on a longer hash like is done here.
//!
//! Finally there are the one or more "Xxx" suffixes. The suffix "U32" means
//! that internally a 32 bit unsigned integer hash total is used instead of the
//! normal 64 bit one. Often even on 64 bit platforms 32 bit operations can be
//! faster and may be better fit for an application. Rust by default always
//! returns 64 bit hashes from its finish() function but since often only a
//! 32 bit hash is needed I have added 32 bit version s as well. To return a 64
//! bit hash the internal 32 bit one will be zero extended to 64 bits in
//! finish(). To save from converting to 64 bits then back to 32 bits when only
//! 32 bits are needed the finish_u32() function can be used instead of the
//! normal finish() function with all of the 32 bit hash versions.
//!
//! The last "Xxx" part of the suffix denotes some additional operation or
//! other specialization like is done in some application. A good example of
//! this is in PHP where the high bit is always set because they use a zero hash
//! value to detect an unset hash.
//!
use std::hash::Hasher;

pub mod x33a;
pub mod x33a_php;
pub mod x33a_u32;
pub mod x33a_u32_php;
pub mod x33x;
pub mod x33x_u32;

///
/// This trait is used by 32 bit hashes.
///
pub trait HasherU32: Hasher {
    ///
    /// Returns a 32 bit hash instead of the normal 64 bit one.
    ///
    /// Rust has settled on using 64 bit hashes by default but in many cases for
    /// smaller HashMaps 32 bit hashes are enough. For those cases this function
    /// saves convert to, then back from 64 bit, where the internal hash uses 32
    /// bits and only 32 bits hash are needed.
    ///
    fn finish_u32(&self) -> u32;
}
#[cfg(test)]
mod tests {}
