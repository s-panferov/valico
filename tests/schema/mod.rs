use std::fs;
use std::path;
use std::io::Read;
use serialize::json;
use valico::json_schema;

fn visit_specs<F>(dir: &path::Path, cb: F) where F: Fn(&path::Path, json::Json) {
    let contents = fs::read_dir(dir).ok().unwrap();
    for entry in contents {
        let path = entry.unwrap().path();
        match fs::File::open(&path) {
            Err(_) => continue,
            Ok(mut file) => {
                let metadata = file.metadata().unwrap();
                if metadata.is_file() {
                    let mut content = String::new();
                    file.read_to_string(&mut content).ok().unwrap();
                    let json: json::Json = content.parse().unwrap();
                    cb(&path, json);
                }
            }
        }
    }
}

#[test]
fn test_suite() {
    let mut content = String::new();

    fs::File::open(&path::Path::new("tests/schema/schema.json")).ok().unwrap()
        .read_to_string(&mut content).ok().unwrap();

    let json_v4_schema: json::Json = content.parse().unwrap();

    visit_specs(&path::Path::new("tests/schema/JSON-Schema-Test-Suite/tests/draft4"), |path, spec_set: json::Json| {
        let spec_set = spec_set.as_array().unwrap();

        let exceptions: Vec<(String, String)> = vec![
            ("maxLength.json".to_string(), "two supplementary Unicode code points is long enough".to_string()),
            ("minLength.json".to_string(), "one supplementary Unicode code point is not long enough".to_string()),
            ("refRemote.json".to_string(), "remote ref invalid".to_string()),
            ("refRemote.json".to_string(), "remote fragment invalid".to_string()),
            ("refRemote.json".to_string(), "ref within ref invalid".to_string()),
            ("refRemote.json".to_string(), "changed scope ref invalid".to_string()),
        ];

        for spec in spec_set.iter() {
            let spec = spec.as_object().unwrap();
            let mut scope = json_schema::Scope::new();

            scope.compile(json_v4_schema.clone(), true).ok().unwrap();

            let schema = match scope.compile_and_return(spec.get("schema").unwrap().clone(), false) {
                Ok(schema) => schema,
                Err(err) => panic!("Error in schema {} {}: {:?}",
                    path.file_name().unwrap().to_str().unwrap(),
                    spec.get("description").unwrap().as_string().unwrap(),
                    err
                )
            };

            let tests = spec.get("tests").unwrap().as_array().unwrap();

            for test in tests.iter() {
                let test = test.as_object().unwrap();
                let description = test.get("description").unwrap().as_string().unwrap();
                let data = test.get("data").unwrap();
                let valid = test.get("valid").unwrap().as_boolean().unwrap();

                let state = schema.validate(&data);

                if state.is_valid() != valid {
                    if !&exceptions[..].contains(&(path.file_name().unwrap().to_str().unwrap().to_string(), description.to_string())) {
                        panic!("Failure: \"{}\" in {}",
                            path.file_name().unwrap().to_str().unwrap(),
                            description.to_string());
                    }
                } else {
                    println!("test json_schema::test_suite -> {} .. ok", description);
                }
            }
        }
    })
}