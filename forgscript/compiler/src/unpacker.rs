use std::env::temp_dir;
use std::fs::File;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::str::from_utf8;
// use std::io;

#[link_section = "fscriptcode"]
#[used]
static mut SCRIPT: &str = ">..v\r\n>..v.";

fn write_bin(filename: &String, bin: &[u8]) {
    let mut f = File::create(filename).expect("Could not create file");
    f.write_all(bin).expect("Could not write to file");
    f.sync_all().expect("Could not sync file");
}

fn get_template() -> &'static [u8] {
    let template = include_bytes!("lib\\forgscript.exe");
    return template;
}
fn main() -> io::Result<()> {
    write_bin(&String::from("tmp\\forgscript.exe"), get_template());
    write_bin(&String::from("tmp\\print.fgs"), unsafe {
        SCRIPT.as_bytes()
    });
    let mut child = Command::new("cmd")
        .args(["/C", "tmp\\forgscript.exe", "tmp\\print.fgs"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?;
    assert!(child.wait().expect("Failed to wait on child").success());
    // println!("{:?}", output);
    Ok(())
}
