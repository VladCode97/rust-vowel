use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn create_file(name_file: String) -> std::io::Result<File> {
    File::create(name_file)
}

fn write_file(file: File, content: String) {
    let mut buff_writer = BufWriter::new(file);
    buff_writer
        .write_all(content.to_string().as_bytes())
        .expect("Error write file");
    buff_writer.flush().expect("Error closed data");
}

fn read_file(file: File) -> std::io::Result<String> {
    let content_reader = BufReader::new(file);
    let mut text: String = String::new();
    for line in content_reader.lines() {
        text += &line?;
    }
    Ok(text)
}

fn open_file(path: String) -> File {
    File::open(path).expect("File not found")
}

fn main() {
    let name_file = "new File.txt";
    let file_created = create_file(name_file.to_string()).ok().unwrap();
    write_file(file_created, "content new from file".to_string());
    let text = read_file(open_file(name_file.to_string()))
    .expect("Error read file");
    println!("{}", text);
}
