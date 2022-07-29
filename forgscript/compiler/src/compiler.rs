use std::fs::File;
use std::io::prelude::*;

// Create a file inside of `std::env::temp_dir()`.

fn write_bin(filename: &String, bin: &[u8]) {
    let mut f = File::create(filename).expect("Could not create file");
    f.write_all(bin).expect("Could not write to file");
    f.sync_all().expect("Could not sync file");
}

fn get_template() -> &'static [u8] {
    let template = include_bytes!("lib\\template.exe");
    return template;
}

fn find_vector(vec: &Vec<u8>, subvector: &Vec<u8>) -> Option<usize> {
    fn check_subvector(i: usize, vec: &Vec<u8>, subvector: &Vec<u8>) -> bool {
        for (j, _) in subvector.iter().enumerate() {
            if (vec[i + j] != subvector[j]) {
                return false;
            }
        }
        return true;
    }

    for (i, _) in vec.iter().enumerate() {
        if (check_subvector(i, vec, subvector)) {
            return Some(i);
        }
    }
    None
}

fn replace_vector(vec: &mut Vec<u8>, find: &Vec<u8>, replace: Vec<u8>) -> Option<Vec<u8>> {
    let start = find_vector(&vec, &find)?;
    let end = start + find.len();
    let removed: Vec<u8> = vec.splice(start..end, replace).collect();
    return Some(vec.to_vec());
}

fn main() {
    // let output = Command::new("cmd").args(["/C", "dir"]).output().expect("failed to execute process");
    // println!("{}", match std::str::from_utf8(&output.stdout) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence")
    // });
    let template = get_template();
    write_bin(&String::from("s.exe"), &template);
    let replaced = replace_vector(&mut template.to_vec(), &b"+..v\r\n>..v".to_vec(), b"-..v\r\n>..v........".to_vec()).unwrap();
    write_bin(&String::from("e.exe"), &replaced);
    // println!(
    //     "vec!{:?}",
    //     replaced
    // );
    // fill_template(get_template(), String::from("+..v\r\n>..v"));
    // println!("{:?}", path)
}
