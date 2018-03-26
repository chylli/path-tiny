use regex::Regex;

pub struct Path {
    path : String,
}

pub fn path(path_name: &str) -> Path {
    Path{
        path: path_name.to_string()
    }
}

impl Path {
    pub fn children(&self, sub_path: &str) -> Self {
        let mut owned_string = self.path.to_owned();
        let concat_string = owned_string + "/" + sub_path;
        path( &concat_string )
    }
}


fn canon_path(path_name: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^/\.\.|//").unwrap();
    }
    let mut result =  path_name.to_string();
    while RE.is_match(&result){
        result = RE.replace_all(&result, "/").to_string();
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
    }

    #[test]
    fn canon_path_works(){
        assert_eq!(super::canon_path("/../../hello"), "/hello");
        assert_eq!(super::canon_path("///hello"), "/hello");
        assert_eq!(super::canon_path("//../hello"), "/hello");
    }

    #[test]
    fn children_works() {
        let p1 = super::path("/tmp");
        let p2 = p1.children("abc");
        assert_eq!(p2.path, "/tmp/abc");
    }
}
