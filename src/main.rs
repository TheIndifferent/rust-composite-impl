use std::io;
use std::io::Read;
use std::fs::File;

trait Section {
    fn read_byte(&mut self) -> io::Result<u8>;
    fn sub_section(&mut self, len: u64) -> impl Section;
}

struct FileSection {
    open_file: File,
    cursor: u64,
    length: u64
}

struct SubSection<P: Section> {
    parent_section: P,
    offset: u64,
    cursor: u64,
    limit: u64
}

impl Section for FileSection {
    fn read_byte(&mut self) -> io::Result<u8> {
        // random implementation, not the point of this example:
        let mut buf: [u8; 1] = [0; 1];
        self.open_file.read_exact(&mut buf)?;
        self.cursor = self.cursor + 1;
        return Ok(buf[0]);
    }
    fn sub_section(&mut self, len: u64) -> impl Section {
        return SubSection {
            parent_section: self,
            offset: self.cursor,
            cursor: 0,
            limit: len
        }
    }
}

impl Section for SubSection<P> {
    fn read_byte(&mut self) -> io::Result<u8> {
        let res = self.parent_section.read_byte()?;
        self.cursor = self.cursor + 1;
        return res;
    }
    fn sub_section(&mut self, len: u64) -> impl Section {
        return SubSection {
            parent_section: self,
            offset: self.cursor,
            cursor: 0,
            limit: len
        }
    }
}

fn main() {
    let f = File::open("/etc/hosts")?;
    let mut file_section = FileSection {
        open_file: f,
        cursor: 0,
        length: f.metadata().map(|m|m.len()).expect("file should have a size")
    };
    file_section.read_byte()?;
    file_section.read_byte()?;

    let mut sub_section_1 = SubSection {
        parent_section: &file_section,
        offset: file_section.cursor,
        cursor: 0,
        limit: 10
    };
    sub_section_1.read_byte()?;
    sub_section_1.read_byte()?;

    let mut sub_section_2 = SubSection {
        parent_section: &sub_section_1,
        offset: sub_section_1.cursor,
        cursor: 0,
        limit: 10
    };
    sub_section_2.read_byte()?;
    sub_section_2.read_byte()?;
}
