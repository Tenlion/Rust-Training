
use std::path::PathBuf;
use std::fs;
use std::fs::{File};
use std::io::BufReader;
use ncw::{NcwReader, SampleFormat};
use ncw::BlockHeader;
use hound;

const FILE_ORIGINAL_TYPE: &str = "ncw";
const FILE_NEW_TYPE: &str = "wav";

fn main() {

    let mut sel_folder = PathBuf::new();
    sel_folder.push("/home/Solaas/Desktop/Test/");

    search(&sel_folder);
}

fn search(folder: &PathBuf) {

    let entries = fs::read_dir(&folder).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        let file_type = metadata.file_type();

        if file_type.is_dir() {
            search(&path);
        }
        else {
            let file_type_test: String = String::from("name");
            let new_thing: String = format!("{}.{}", file_type_test, FILE_ORIGINAL_TYPE);
            let mut file_type_blah = PathBuf::new();
            file_type_blah.push(new_thing);

            if path.extension() == file_type_blah.extension(){
                convert(&path);
            }
        }
    }
}

fn convert(file: &PathBuf) {

    let mut output_path = file.clone();
    output_path.set_extension(FILE_NEW_TYPE);

    let ncw_file = File::open(&file).unwrap();
    let mut ncw_data = NcwReader::read(&ncw_file).unwrap();
    let samples = ncw_data.decode_samples();


    let spec = hound::WavSpec {
        channels: ncw_data.header.channels,
        sample_rate: ncw_data.header.sample_rate,
        bits_per_sample: ncw_data.header.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&output_path, spec).unwrap();

    for sample in samples {
        let length_of_sample = sample.len();
        for i in 0..length_of_sample {
            writer.write_sample(sample[i]).unwrap();
        }
    }

    writer.finalize().unwrap();
}
