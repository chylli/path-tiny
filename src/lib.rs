extern crate regex;
use regex::Regex;

struct Path {
    path : String,
}

fn path(path_name: &str) -> Path {
    Path{
        path: path_name.to_string()
    }
}

fn canon_path(path_name: &str) -> String {
    // remove /../..
    let re1 = Regex::new(r"//").unwrap();
    let re2 = Regex::new(r"^/\.\.").unwrap();
    let mut result =  path_name.to_string();
    while re1.is_match(&result) || re2.is_match(&result){
        let string1 = re1.replace_all(&result, "/").to_string();
        result = re2.replace_all(&string1,"/").to_string();
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
