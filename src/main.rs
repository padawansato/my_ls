use std::fs;

fn main() {
    println!("`ls .`");
    // read_dirメソッド ディレクトリの内容を読み込む。返り値は`io::Result<Vec<Path>>`。より正確には`Result<ReadDir>`
    match fs::read_dir(".") {
        Err(e) => println!("! {:?}", e.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }
}
