#![cfg_attr(not(test), no_std)]

/// Context API for providing generalized setup, and cleanup
///
/// this is useful for building APIs that use RAII-like patterns
/// but with fixed scopes
pub trait Context: Sized {
    type Ctx;
    fn setup_context(&mut self) -> Option<Self::Ctx>;
    fn cleanup_context(&mut self, _: Self::Ctx);

    fn with_ref<T>(mut self, f: impl Fn(&Self::Ctx) -> T) -> Option<T> {
        let context = self.setup_context()?;
        let ret = f(&context);
        self.cleanup_context(context);
        Some(ret)
    }
    fn with_mut_ref<T>(mut self, f: impl Fn(&mut Self::Ctx) -> T) -> Option<T> {
        let mut context = self.setup_context()?;
        let ret = f(&mut context);
        self.cleanup_context(context);
        Some(ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
}
