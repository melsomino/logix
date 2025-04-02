use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::index::lines_reader::LinesReader;

pub struct AnyTokenLinesReader {
    pub(crate) readers: Vec<LinesReader>,
    heap: BinaryHeap<Reverse<(usize, usize)>>, // (offset, reader_index)
}

impl AnyTokenLinesReader {
    pub fn new(mut readers: Vec<LinesReader>) -> anyhow::Result<Self> {
        let mut heap = BinaryHeap::new();

        for (i, reader) in readers.iter_mut().enumerate() {
            if let Some(offset) = reader.next()? {
                heap.push(Reverse((offset, i)));
            }
        }

        Ok(Self { readers, heap })
    }

    pub fn next(&mut self) -> anyhow::Result<Option<usize>> {
        if let Some(Reverse((offset, idx))) = self.heap.pop() {
            if let Some(next_offset) = self.readers[idx].next()? {
                self.heap.push(Reverse((next_offset, idx)));
            }
            Ok(Some(offset))
        } else {
            Ok(None)
        }
    }
}
