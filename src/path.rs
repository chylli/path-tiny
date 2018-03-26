use regex::Regex;

pub struct Path {
    path : String,
}

pub fn path(path_name: &str) -> Path {
    Path{
        path: path_name.to_string()
    }
}

fn canon_path(path_name: &str) -> String {
    let re1 = Regex::new(r"^/\.\.|//").unwrap();
    let mut result =  path_name.to_string();
    while re1.is_match(&result){
        result = re1.replace_all(&result, "/").to_string();
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn path_works() {
        let path_name = "tmp/a.txt";
        let p1 = super::path(path_name);
        assert_eq!(p1.path, path_name);
        assert_eq!(super::canon_path("/../../hello"), "/hello");
        assert_eq!(super::canon_path("///hello"), "/hello");
        assert_eq!(super::canon_path("//../hello"), "/hello");
    }
}
