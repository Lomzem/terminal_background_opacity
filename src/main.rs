use serde_json::Number;

use std::{
    fs::File,
    io::{BufReader, Write},
};

fn main() {
    // const JSON_SETTINGS_FILE: &str = "C:/Users/Lomzem/AppData/Local/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json";
    const JSON_SETTINGS_FILE: &str = "/mnt/c/Users/Lomzem/AppData/Local/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json";

    let arg = std::env::args().nth(1);

    if arg.is_none() {
        println!("ERROR: Must pass in wanted opacity level");
        return;
    }

    let mut opacity: f64 = arg
        .unwrap()
        .parse()
        .expect("Unable to cast argument into a number between 0-100");

    if opacity > 100.0 {
        opacity = 100.0;
    }

    opacity /= 100.0;

    let file = File::open(JSON_SETTINGS_FILE).expect("Unable to open terminal settings file");
    let reader = BufReader::new(file);

    let mut json: serde_json::Value = serde_json::from_reader(reader).unwrap();

    if let serde_json::Value::Number(ref mut bk_opacity) =
        &mut json["profiles"]["defaults"]["backgroundImageOpacity"]
    {
        *bk_opacity = Number::from_f64(opacity).unwrap();
    }

    let mut file = File::create(JSON_SETTINGS_FILE).expect("Unable to open terminal settings file");
    file.write_all(json.to_string().as_bytes()).unwrap();
}
