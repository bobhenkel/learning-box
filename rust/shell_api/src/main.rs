use std::process::Command;


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> std::string::String {
    let output = Command::new("docker")
                     .arg("run")
                     .arg("--rm")
                     .arg("alpine")
                     .arg("date")
                     .output()
                     .expect("failed to execute process");

println!("status: {}", output.status);
println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
let string_json = String::from_utf8_lossy(&output.stdout);
string_json.to_string()
//let return_value = serde_json::from_str(&string_json).unwrap();
//JsonValue(return_value)
//assert!(output.status.success());
    //"Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
