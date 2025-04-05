use crate::index::lines_reader::LinesReader;

pub struct AllLinesReader {
    readers: Vec<LinesReader>,
    current: Vec<u64>,
}

impl AllLinesReader {
    pub fn new(mut readers: Vec<LinesReader>) -> anyhow::Result<Self> {
        let mut current = vec![0; readers.len()];
        Self::read_all_next(&mut current, &mut readers)?;
        Ok(Self { readers, current })
    }

    fn read_all_next(current: &mut Vec<u64>, readers: &mut [LinesReader]) -> anyhow::Result<()> {
        for (i, reader) in readers.iter_mut().enumerate() {
            if let Some(offset) = reader.next()? {
                current[i] = offset;
            } else {
                *current = Vec::new();
                break;
            }
        }
        Ok(())
    }
    pub fn next(&mut self) -> anyhow::Result<Option<u64>> {
        if self.current.is_empty() {
            return Ok(None);
        }
        loop {
            let mut max = self.current[0];
            let mut min = max;

            for &offset in &self.current[1..] {
                max = max.max(offset);
                min = min.min(offset);
            }

            if min == max {
                Self::read_all_next(&mut self.current, &mut self.readers)?;
                return Ok(Some(min));
            }
            // Advance readers with smallest offset
            for (i, current) in self.current.iter_mut().enumerate() {
                if *current < max {
                    if let Some(offset) = self.readers[i].next()? {
                        *current = offset;
                    } else {
                        self.current = Vec::new();
                        return Ok(None);
                    }
                }
            }
        }
    }

    pub fn print_debug(&self, indent: usize) {
        println!("{}All:", "  ".repeat(indent));
        for reader in &self.readers {
            reader.print_debug(indent + 1);
        }
    }
}
