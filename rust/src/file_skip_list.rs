use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, Write},
    path::{Path, PathBuf},
};

pub const MERGED_INDEX_DIR: &str = "inverted_index/merged";
const PARTITION: u16 = 100;

pub struct FileSkip {
    character: char,
    word: String,
    byte_offset: u64,
}

type FileSkipList = Vec<FileSkip>;

impl FileSkip {
    pub fn build_skip_list(path: PathBuf) -> FileSkipList {
        let initial_character: char = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .chars()
            .next()
            .unwrap();

        // if file doesn't exist, create it
        let file: File = if path.exists() {
            File::open(&path).unwrap()
        } else {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            File::create(&path).unwrap()
        };
        // create a buffered reader, which is more efficient for reading lines
        let mut reader: BufReader<File> = BufReader::new(file);
        // BufReader<R> can improve the speed of programs that make small and
        // repeated read calls to the same file.
        // It does not help when reading very large amounts at once, or reading just one or a few times.
        // It also provides no advantage when reading from a source that is already in memory, like a Vec<u8>.

        let mut skip_list: FileSkipList = Vec::new();
        let mut line_count: u32 = 0;
        let mut line: String = String::new();
        // read_line appends to the provided buffer, gets the line from the buffer and returns the number of bytes read
        while let Ok(bytes_read) = reader.read_line(&mut line) {
            if bytes_read == 0 {
                break;
            }

            // Get current position using the reader's position
            // self.seek(SeekFrom::Current(0)) equivalent to self.stream_position()
            // stream_position will point to the end of this line, so we need to subtract the bytes_read to get the start of the line
            let current_position = reader.stream_position().unwrap() - bytes_read as u64;

            // for every partition-th line, add a skip entry!!
            if line_count % PARTITION as u32 == 0 {
                if let Some(word) = line.split(':').next() {
                    skip_list.push(FileSkip {
                        character: initial_character,
                        word: word.to_string(),
                        byte_offset: current_position,
                    });
                }
            }

            line_count += 1;
            line.clear();
        }

        skip_list
    }

    pub fn write_skip_list(skip_list: &FileSkipList) {
        if skip_list.is_empty() {
            return;
        }

        let path = Path::new(MERGED_INDEX_DIR);
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
        }

        let character = skip_list[0].character;
        let file_path = format!("{}/{}_skiplist.txt", MERGED_INDEX_DIR, character);
        let mut file = File::create(file_path).unwrap();

        // Write skip list entries to file
        for (i, skip) in skip_list.iter().enumerate() {
            let entry = if i == skip_list.len() - 1 {
                format!("{}:{}", skip.word.trim(), skip.byte_offset)
            } else {
                format!("{}:{},", skip.word.trim(), skip.byte_offset)
            };
            file.write_all(entry.as_bytes()).unwrap();
        }
    }

    pub fn read_skip_list(character: char) -> FileSkipList {
        let file_path = format!("{}/{}_skiplist.txt", MERGED_INDEX_DIR, character);
        let path = Path::new(&file_path);

        if !path.exists() {
            return Vec::new();
        }

        let mut file = File::open(path).unwrap();
        let mut read_buffer = String::new();
        file.read_to_string(&mut read_buffer).unwrap();

        let mut skip_list = Vec::new();
        for entry in read_buffer.split(',') {
            if entry.is_empty() {
                continue;
            }

            if let Some((word, offset_str)) = entry.split_once(':') {
                if let Ok(byte_offset) = offset_str.parse::<u64>() {
                    skip_list.push(FileSkip {
                        character,
                        word: word.to_string(),
                        byte_offset,
                    });
                }
            }
        }

        skip_list
    }
}
