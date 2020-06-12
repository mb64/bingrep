use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};
use memmap::MmapOptions;
use structopt::StructOpt;

fn search(test: &[u8], file: &[u8]) {
    let progress = ProgressBar::new((file.len() - test.len()) as u64)
        .with_style(ProgressStyle::default_bar().template("{wide_bar} {bytes}/{total_bytes}"));
    for file_pos in 0..file.len() - test.len() {
        if test == &file[file_pos..file_pos + test.len()] {
            progress.println(format!(
                "Match from {} to {}",
                file_pos,
                file_pos + test.len()
            ));
        }

        // Only update progress bar every megabyte in case it's too expensive
        if file_pos % (1024 * 1024) == 0 {
            progress.inc(1024 * 1024)
        }
    }
}

fn os_str_to_vec(s: &OsStr) -> Result<Box<[u8]>, OsString> {
    if s.is_empty() {
        Err("Non-empty pattern required".into())
    } else {
        Ok(s.as_bytes().to_owned().into_boxed_slice())
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// The length of the region to check.  Defaults to the length of the file.
    #[structopt(short, long)]
    length: Option<usize>,

    /// The text to search for
    #[structopt(parse(try_from_os_str = os_str_to_vec))]
    pattern: Box<[u8]>,

    /// The file to search in
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    assert!(opt.pattern.len() > 0, "pattern should not be empty");

    let file = File::open(opt.file).expect("could not open file");

    let mut mmap_opts = MmapOptions::new();
    if let Some(len) = opt.length {
        mmap_opts.len(len);
    }
    let mmap = unsafe { mmap_opts.map(&file).expect("could not mmap file") };

    search(&opt.pattern, &mmap[..]);
}
