/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::buffers::RUMBuffer;
use crate::buffers::{buffer_find, buffer_find_byte};

pub struct RUMSliceSplitIter<'a, 'b> {
    pub remainder: &'a [u8],
    pub pattern: &'b [u8],
    pub last: usize,
    pub pattern_length: usize,
}

pub struct RUMSliceEnumerateIter<'a, 'b> {
    pub remainder: &'a [u8],
    pub pattern: &'b [u8],
    pub cummulative: usize,
    pub last: usize,
    pub pattern_length: usize,
}

pub trait RUMByteSliceSplitIterTrait {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait RUMByteSliceEnumeratorIterTrait {
    type Item;
    fn next(&mut self) -> Option<(usize, Self::Item)>;
}

pub trait RUMByteSliceIteratorExt<'a, 'b> {
    fn split_fast(&'a self, pattern: &'b [u8]) -> RUMSliceSplitIter<'a, 'b>;
    fn enumerate_fast(&'a self, pattern: &'b [u8]) -> RUMSliceEnumerateIter<'a, 'b>;
}

impl<'a, 'b> Iterator for RUMSliceSplitIter<'a, 'b> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.last = buffer_find(self.remainder, self.pattern);

        if self.remainder.len() > 0 {
            let r = Some(&self.remainder[..self.last]);
            let next = self.last + self.pattern_length;
            if next <= self.remainder.len() {
                self.remainder = &self.remainder[self.last + self.pattern_length..];
            } else {
                self.remainder = &self.remainder[self.last..];
            }
            r
        } else {
            None
        }
    }
}

impl<'a, 'b> Iterator for RUMSliceEnumerateIter<'a, 'b> {
    type Item = (usize, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        self.last = buffer_find(self.remainder, self.pattern);
        self.cummulative += self.last;

        if self.remainder.len() > 0 {
            let r = Some((self.cummulative, &self.remainder[..self.last]));
            self.remainder = &self.remainder[self.last + self.pattern.len()..];
            r
        } else {
            None
        }
    }
}

pub trait RUMBufferIteratorExt {
    fn split_fast(&self, byte: u8) -> RUMBufferSplitIter;
}

pub trait RUMBufferSplitIterTrait {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub struct RUMBufferSplitIter {
    pub remainder: RUMBuffer,
    pub byte: u8
}

impl RUMBufferSplitIter {
    #[inline]
    pub fn pop_item(&mut self) -> Option<RUMBuffer> {
        match buffer_find_byte(&self.remainder, self.byte) {
            Some(i) => {
                let mut v = self.remainder.split_to(i + 1)?;
                v.truncate(i);
                Some(v)
            },
            None => None
        }
    }
}

impl<'a> Iterator for RUMBufferSplitIter {
    type Item = RUMBuffer;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_item()
    }
}

impl<'a, 'b> RUMByteSliceIteratorExt<'a, 'b> for &[u8] {
    #[inline]
    fn split_fast(&'a self, pattern: &'b [u8]) -> RUMSliceSplitIter<'a, 'b> {
        RUMSliceSplitIter {
            pattern_length: pattern.len(),
            remainder: self.clone(),
            pattern: pattern.clone(),
            last: 0,
        }
    }

    #[inline]
    fn enumerate_fast(&'a self, pattern: &'b [u8]) -> RUMSliceEnumerateIter<'a, 'b> {
        RUMSliceEnumerateIter {
            pattern_length: pattern.len(),
            remainder: self.clone(),
            pattern: pattern.clone(),
            cummulative: 0,
            last: 0,
        }
    }
}

impl<'a> RUMBufferIteratorExt for RUMBuffer {
    fn split_fast(&self, byte: u8) -> RUMBufferSplitIter {
        RUMBufferSplitIter {
            remainder: self.clone(),
            byte,
        }
    }
}