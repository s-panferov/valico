use std::old_io::fs;
use std::old_io::fs::PathExtensions;
use serialize::json;
use valico::json_schema;

fn visit_specs<F>(dir: &Path, cb: F) where F: Fn(&Path, json::Json) {
    let contents = fs::readdir(dir).ok().unwrap();
    for entry in contents.iter() {
        if entry.is_file() {
            let mut file = fs::File::open(&entry).ok().unwrap();
            let json: json::Json = file.read_to_string().ok().unwrap().parse().unwrap();
            cb(&entry, json);
        }
    }
}

#[test]
fn test_suite() {
    let json_v4_schema: json::Json = fs::File::open(&Path::new("tests/schema/schema.json")).ok().unwrap()
        .read_to_string().ok().unwrap().parse().unwrap();

    visit_specs(&Path::new("tests/schema/JSON-Schema-Test-Suite/tests/draft4"), |&: path, spec_set: json::Json| {
        let mut failures: Vec<(String, String)> = vec![];

        let spec_set = spec_set.as_array().unwrap();
        for spec in spec_set.iter() {
            let spec = spec.as_object().unwrap();
            let mut scope = json_schema::Scope::new();

            scope.compile(json_v4_schema.clone()).ok().unwrap();

            let schema = scope.compile_and_return(spec.get("schema").unwrap().clone()).ok().unwrap();
            let tests = spec.get("tests").unwrap().as_array().unwrap();
            
            for test in tests.iter() {
                let test = test.as_object().unwrap();
                let description = test.get("description").unwrap().as_string().unwrap();
                let data = test.get("data").unwrap();
                let valid = test.get("valid").unwrap().as_boolean().unwrap();

                let state = schema.validate(&data);

                if state.is_valid() == valid {
                    println!("Spec OK in {:?}: {}", path, description);
                } else {
                    failures.push((path.filename_str().unwrap().to_string(), description.to_string()))
                }
            }
        }

        let exceptions: Vec<(String, String)> = vec![
            ("maxLength.json".to_string(), "two supplementary Unicode code points is long enough".to_string()),
            ("minLength.json".to_string(), "one supplementary Unicode code point is not long enough".to_string()),
            ("refRemote.json".to_string(), "remote ref invalid".to_string()),
            ("refRemote.json".to_string(), "remote fragment invalid".to_string()),
            ("refRemote.json".to_string(), "ref within ref invalid".to_string()),
            ("refRemote.json".to_string(), "changed scope ref invalid".to_string()),
        ];

        for failure in failures.iter() {
            if !exceptions.as_slice().contains(failure) {
                panic!("Failure: \"{}\" in {}", failure.1, failure.0);
            }
        }
    })
}