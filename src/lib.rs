#[allow(dead_code)]
struct Path {
    path : String,
}
#[allow(dead_code)]
fn path(path_name: &str) -> Path {
    Path{
        path: path_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_works() {
        let path_name = "tmp/a.txt";
        let p1 = super::path(path_name);
        assert_eq!(p1.path, path_name);
    }
}
