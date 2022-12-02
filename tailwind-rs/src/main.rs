use serde_json::Value;

fn main() {
    plain();
    parse_jsml();
}

fn plain() {
    let plain = include_str!("convert.plain");
    let mut out = String::new();
    for line in plain.split("\n") {
        let mut sides = line.split("\t").map(|s| s.to_string()).collect::<Vec<String>>();
        if sides[..2].iter().any(|s| *s == String::new()) {
            continue;
        }
        
        println!("{:?}", sides);
        sides[0] = fix_var_name(&sides[0]);
        //println!("{}", output_format(&sides));
        out = [out,output_format(&sides)].join("\n").trim_start().to_string()
    }
    std::fs::write("plain.out", out).unwrap();
}

fn fix_var_name(s: &str)->String {
    s.replace("-", "_").replace(".", "_").replace("/", "__")
}

fn output_format(s: &[String]) -> String {
    format!("pub const {}: T = \"{}\";", s[0], s[1])
}

fn parse_jsml() {
    let jsml = include_str!("convertjson.json");
    let parsed = serde_json::from_str::<Value>(jsml).unwrap();
    let mut out = String::new();

    let Value::Array(a) = parsed else {
        return;
    };

    for x in a {
        let Value::Array(b) = x else {
            return
        };

        let mut s = b
            .into_iter()
            .map(|l| {
                if let Value::String(ll) = l {
                    ll
                } else {
                    "ERROR".to_string()
                }
            })
            .collect::<Vec<String>>();

        s[0] = fix_var_name(&s[0]);

        out = [out,output_format(&s)].join("\n").trim_start().to_string();
    }

    std::fs::write("json.out", out).unwrap();
}
