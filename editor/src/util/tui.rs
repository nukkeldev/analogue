use compact_str::CompactString;
use ratatui::buffer::Buffer;

pub trait BufferExt {
    /// Gets a line from the buffer as a string.
    ///
    /// Returns `None` if `line` is out of bounds.
    fn get_line(&self, line: usize) -> Option<CompactString>;
}

impl BufferExt for Buffer {
    fn get_line(&self, line: usize) -> Option<CompactString> {
        self.content()
            .chunks(self.area.width as usize)
            .nth(line)
            .map(|row| row.iter().map(|c| c.symbol()).collect::<CompactString>())
    }
}
