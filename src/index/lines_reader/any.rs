use crate::index::lines_reader::LinesReader;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct AnyLinesReader {
    pub(crate) readers: Vec<LinesReader>,
    heap: BinaryHeap<Reverse<(usize, usize)>>, // (offset, reader_index)
}

impl AnyLinesReader {
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

    pub fn print_debug(&self, indent: usize) {
        println!("{}Any:", "  ".repeat(indent));
        for reader in &self.readers {
            reader.print_debug(indent + 1);
        }
    }
}
