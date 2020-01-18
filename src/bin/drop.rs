struct File(u32);
static mut FID: u32 = 0;

impl File {
    fn new() -> File {
        unsafe {
            FID += 1;
            println!("Creating file {}", FID);
            File(FID)
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        println!("Dropping file {}", self.0);
    }
}

fn main() {
    let _file1 = File::new();
    let _file2 = File::new();
    let _file3 = File::new();
    drop(_file2);
}
