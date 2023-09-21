# ctxapi


## Usage

```rust
use ctxapi::Context;
struct Dummy;
impl Context for Dummy {
    type Ctx = ();
    fn setup_context(&mut self) -> Option<Self::Ctx> {
        log::error!("Dummy: context setup");
        Some(())
    }
    fn cleanup_context(&mut self, _: Self::Ctx) {
        log::error!("Dummy: context cleanup");
    }
}

struct Create<T: AsRef<std::path::Path>>(T);
impl<T: AsRef<std::path::Path>> Context for Create<T> {
    type Ctx = std::fs::File;
    fn setup_context(&mut self) -> Option<Self::Ctx> {
        std::fs::File::create(self.0.as_ref()).ok()
    }
    fn cleanup_context(&mut self, _: Self::Ctx) {
        // file will be dropped automatically
    }
}

struct Read<T: AsRef<std::path::Path>>(T);
impl<T: AsRef<std::path::Path>> Context for Read<T> {
    type Ctx = std::fs::File;
    fn setup_context(&mut self) -> Option<Self::Ctx> {
        std::fs::File::open(self.0.as_ref()).ok()
    }
    fn cleanup_context(&mut self, _: Self::Ctx) {
        // file will be dropped automatically
    }
}

#[test]
fn run_test() {
    _ = env_logger::Builder::new().init();
    Dummy.with_ref(|_| {
        println!("running with context");
    });
    Create("zmnbjfieu_test").with_mut_ref(|file| {
        use std::io::Write;
        _ = file.write(b"hello, test file!");
        println!("{:?}", file);
    });
    Read("zmnbjfieu_test").with_mut_ref(|file| {
        println!("{:?}", file);
        use std::io::Read;
        let mut src = String::new();
        _ = file.read_to_string(&mut src);
        println!("src: {src}");
    });
    _ = std::fs::remove_file("zmnbjfieu_test");
}
```
