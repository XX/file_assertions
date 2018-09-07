#[macro_export]
macro_rules! delete_file {
    ($path:tt) => {
        ::std::fs::remove_file($path).expect(&format!("File: {:?} can not delete.", $path));
    };
}

#[macro_export]
macro_rules! clear_dir {
    ($path:tt) => {
        if ::std::fs::metadata($path).is_ok() {
            ::std::fs::remove_dir_all($path).expect(&format!("Can't remove {:?} dir.", $path));
        }
        ::std::fs::create_dir_all($path).expect(&format!("Can't create {:?} dir.", $path));
    };
}

#[macro_export]
macro_rules! create_file {
    ($path:tt, $content:tt) => {{
        use std::fs;
        use std::io::Write;
        use std::path::Path;

        let path = Path::new($path);
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).expect(&format!("Can't create dir: {:?}", dir));
        }
        let mut file = fs::File::create(path).expect(&format!("File: {:?} can not create.", path));
        file.write_all($content.as_bytes())
            .expect(&format!("File: {:?} wrong content write.", path));
    }};
}

#[macro_export]
macro_rules! assert_content {
    ($path:tt, $content:tt) => {{
        use std::fs;
        use std::io::Read;
        use std::path::Path;

        let path = Path::new($path);
        let mut file = fs::File::open(path).expect(&format!("File: {:?} can not open.", path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("File: {:?} wrong content reading.", path));

        assert_eq!($content, contents);
    }};
}

