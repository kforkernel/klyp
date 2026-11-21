/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // getting cmdline inputs
    let cmd_line_info = match Args::get_args() {
        Ok(info) => info,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        }
    };

    // checking flag
    let flag: String = cmd_line_info.mode;

    // checking verbose
    let verbose: bool = cmd_line_info.verbose;




    // running encoding with verbose
    if flag == "-e" && verbose == true {
        let encode_source_file: String = cmd_line_info.src;
        let encode_destination_file: String = cmd_line_info.dst;

        // opening source file and fetching its metadata size for the bar
        let input = match File::open(&encode_source_file) {
            Ok(file_handle) => file_handle,
            Err(file_error) => {
                eprintln!("Error: Could not open source file '{}'.\nReason: {}", encode_source_file, file_error);
                std::process::exit(1);
            }
        };
        
        // get metadata so we know the total file size for progress bar
        let total_size = input.metadata().map(|m| m.len()).unwrap_or(0);

        let reader = std::io::BufReader::new(input);

        let output = match File::create(&encode_destination_file) {
            Ok(ok_output) => ok_output,
            Err(err_output) => {
                eprintln!("Error: Couldn't create destination file {}. Reason: {}", encode_destination_file, err_output);
                std::process::exit(1);
            }
        };

        let writer = std::io::BufWriter::new(output);
        let mut encoder = GzEncoder::new(writer, Compression::default());

        // setup progress bar looks
        let mut tracked_reader = create_progress_reader(reader, total_size, "magenta");

        // copy now reads from tracked_reader instead of reader
        match std::io::copy(&mut tracked_reader, &mut encoder) {
            Ok(bytes_copied) => {
                println!("\nSuccessfully processed {} raw bytes.", bytes_copied);
            }
            Err(err) => {
                eprintln!("Error during compression stream: {}", err);
                std::process::exit(1);
            }
        }

        // stop encoding
        match encoder.finish() {
            Ok(_) => println!("Compression finished!"),
            Err(err) => {
                eprintln!("Error finalizing the compressed file (Disk full?): {}", err);
                std::process::exit(1);
            }
        }
    }






    // running decompression with verbose
    else if flag == "-d" && verbose == true{
        let decode_source_file: String = cmd_line_info.src;
        let decode_destination_file: String = cmd_line_info.dst;

        let input = match File::open(&decode_source_file) {
            Ok(file_handle) => file_handle,
            Err(file_error) => {
                eprintln!("Error: Could not open source file '{}'. Reason: {}", decode_source_file, file_error);
                std::process::exit(1);
            }
        };

        // for decompression total_size is the size of the COMPRESSED file
        let total_size = input.metadata().map(|m| m.len()).unwrap_or(0);

        let reader = std::io::BufReader::new(input);

        let output = match File::create(&decode_destination_file) {
            Ok(ok_output) => ok_output,
            Err(err_output) => {
                eprintln!("Error: Couldn't create destination file {}. Reason: {}", decode_destination_file, err_output);
                std::process::exit(1);
            }
        };

        let mut writer = std::io::BufWriter::new(output);

        // setup progress bar for decompression tracking
        let tracked_reader = create_progress_reader(reader, total_size, "red");
        let mut decoder = GzDecoder::new(tracked_reader);

        match std::io::copy(&mut decoder, &mut writer) {
            Ok(bytes_copied) => {
                println!("\nSuccessfully processed {} raw bytes.", bytes_copied);
            }
            Err(err) => {
                eprintln!("Error during decompression stream: {}", err);
                std::process::exit(1);
            }
        }

        // stop decoding and flush disk
        match writer.flush() {
            Ok(_) => println!("Decompression finished!"),
            Err(err) => {
                eprintln!("Error finalizing the decompressed file (Disk full?): {}", err);
                std::process::exit(1);
            }
        }
    }






    else if flag == "-e" && verbose == false{
        let encode_source_file: String = cmd_line_info.src;
        let encode_destination_file: String = cmd_line_info.dst;

        // opening source file and fetching its metadata size for the bar
        let input = match File::open(&encode_source_file) {
            Ok(file_handle) => file_handle,
            Err(file_error) => {
                eprintln!("Error: Could not open source file '{}'. Reason: {}", encode_source_file, file_error);
                std::process::exit(1);
            }
        };

        let mut reader = std::io::BufReader::new(input);

        let output = match File::create(&encode_destination_file) {
            Ok(ok_output) => ok_output,
            Err(err_output) => {
                eprintln!("Error: Couldn't create destination file {}. Reason: {}", encode_destination_file, err_output);
                std::process::exit(1);
            }
        };

        let writer = std::io::BufWriter::new(output);

        let mut encoder = GzEncoder::new(writer, Compression::default());

        // copy now reads from reader
        match std::io::copy(&mut reader, &mut encoder) {
            Ok(bytes_copied) => {
                println!("Successfully processed {} raw bytes.", bytes_copied);
            }
            Err(err) => {
                eprintln!("Error during compression stream: {}", err);
                std::process::exit(1);
            }
        }

        // stop encoding
        match encoder.finish() {
            Ok(_) => println!("Compression finished!"),
            Err(err) => {
                eprintln!("Error finalizing the compressed file (Disk full?): {}", err);
                std::process::exit(1);
            }
        }
    }






    else{
        let decode_source_file: String = cmd_line_info.src;
        let decode_destination_file: String = cmd_line_info.dst;

        let input = match File::open(&decode_source_file) {
            Ok(file_handle) => file_handle,
            Err(file_error) => {
                eprintln!("Error: Could not open source file '{}'. Reason: {}", decode_source_file, file_error);
                std::process::exit(1);
            }
        };

        let reader = std::io::BufReader::new(input);

        let output = match File::create(&decode_destination_file) {
            Ok(ok_output) => ok_output,
            Err(err_output) => {
                eprintln!("Error: Couldn't create destination file {}. Reason: {}", decode_destination_file, err_output);
                std::process::exit(1);
            }
        };

        let mut writer = std::io::BufWriter::new(output);

        let mut decoder = GzDecoder::new(reader);

        match std::io::copy(&mut decoder, &mut writer) {
            Ok(bytes_copied) => {
                println!("Successfully processed {} raw bytes.", bytes_copied);
            }
            Err(err) => {
                eprintln!("Error during decompression stream: {}", err);
                std::process::exit(1);
            }
        }

        // stop decoding and flush disk
        match writer.flush() {
            Ok(_) => println!("Decompression finished!"),
            Err(err) => {
                eprintln!("Error finalizing the decompressed file (Disk full?): {}", err);
                std::process::exit(1);
            }
        }
    }
    
}

struct Args {
    mode: String,
    src: String,
    dst: String,
    verbose: bool,
}

impl Args {
    fn get_args() -> Result<Args, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            eprintln!("Usage: klyp [-e | -d] <SRC> <DST> [-v]");
            std::process::exit(1);
        } 
        
        else if args.len() == 3 {
            let args_results = Args {
                mode: "-e".to_string(),
                src: args[1].clone(),
                dst: args[2].clone(),
                verbose: false,
            };
            return Ok(args_results);
        }

        if args.len() == 4 && args[3] == "-v"{
            let args_results = Args {
                mode: "-e".to_string(),
                src: args[1].clone(),
                dst: args[2].clone(),
                verbose: true,
            };
            return Ok(args_results);
        }

        let first_flag = &args[1];

        if first_flag == "-e" && args.len() == 4{
            let args_results: Args = Args {
                mode: args[1].clone(),
                src: args[2].clone(),
                dst: args[3].clone(),
                verbose: false,
            };
            return Ok(args_results);
        } 
        
        else if first_flag == "-d" && args.len() == 4{
            let args_results = Args {
                mode: args[1].clone(),
                src: args[2].clone(),
                dst: args[3].clone(),
                verbose: false,
                
            };
            return Ok(args_results);
        } 

        else if first_flag == "-e" && args.len() == 5 && args[4] == "-v"{
            let args_results: Args = Args {
                mode: args[1].clone(),
                src: args[2].clone(),
                dst: args[3].clone(),
                verbose: true,
            };
            return Ok(args_results);
        }

        else if first_flag == "-d" && args.len() == 5 && args[4] == "-v"{
            let args_results: Args = Args {
                mode: args[1].clone(),
                src: args[2].clone(),
                dst: args[3].clone(),
                verbose: true,
            };
            return Ok(args_results);
        }
        
        else {
            return Err("Usage: CMD [-e | -d] <SRC> <DST> [-v]".to_string());
        }
    }
}

// progress bar fn
fn create_progress_reader<R: Read>(reader: R, total_bytes: u64, color: &str) -> impl Read {
    // create progress bar
    let pb = ProgressBar::new(total_bytes);
    
    // choose color
    let template = format!(
        "{{spinner:.green}} [{{elapsed_precise}}] [{{bar:40.{}/blue}}] {{bytes}}/{{total_bytes}} ({{eta}})", 
        color
    );

    // setting style
    pb.set_style(ProgressStyle::default_bar()
        .template(&template)
        .unwrap()
        .progress_chars("#>-"));

    // wrap reader and return
    pb.wrap_read(reader)
}
