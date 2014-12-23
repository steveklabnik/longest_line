extern crate test;
use test::Bencher;

use std::io::BufferedReader;
use std::io::File;

#[bench]
fn cn34big(b: &mut Bencher) {
    b.iter(|| {
        let mut max_length = 0u;
        let mut longest_line : Option<String> = None;

        let path = Path::new("test.txt");
        let mut file = BufferedReader::new(File::open(&path));

        for line in file.lines().map(|l| l.unwrap()) {
            if line.len() > max_length {
                max_length = line.len();
                longest_line = Option::Some(line);
            }
        }
    });
}

#[bench]
fn cn2uqe7(b: &mut Bencher) {
    b.iter(|| {
        let mut max_length = 0u;
        let mut longest_line : Option<String> = None;

        let path = Path::new("test.txt");
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines().take_while(|l|l.is_ok()) {
            drop(line.map(|s| {
                if s.len() > max_length {
                    max_length = s.len();
                    longest_line = Some(s.to_string());
                }
            }));
        }
    });
}

#[bench]
fn cn36eec(b: &mut Bencher) {
    b.iter(|| {
        let mut max_length = 0u;
        let mut longest_line : Option<String> = None;

        let path = Path::new("test.txt");
        let mut file = BufferedReader::new(File::open(&path));
        for mut line in file.lines().map(|l| l.unwrap()) {
            let ts = line.as_slice().trim_right().len();
            if ts > max_length {
                line.truncate(ts);
                max_length = ts;
                longest_line = Some(line);
            }
        }
    });
}

#[bench]
fn cn36xn6(b: &mut Bencher) {
    b.iter(|| {
        let mut max_length = 0;
        let mut longest_line : Option<String> = None;

        let path = Path::new("test.txt");
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            let line = line.unwrap();
            let contents = line.as_slice().trim_right();
            let line_length = contents.len();
            if line_length > max_length {
                max_length = line_length;
                longest_line = Some(contents.to_string());
            }
        }
    });
}

#[bench]
fn cn34t06(b: &mut Bencher) {
    b.iter(|| {
        let mut max_length = 0u;
        let mut longest_line : Option<&str> = None;

        let path = Path::new("test.txt");
        let mut file = BufferedReader::new(File::open(&path));

        match file.lines().filter_map(|l| l.ok()).max_by(|x| x.len()) {
            Option::Some(line) => (),
            _          => (),
        }
    });
}
