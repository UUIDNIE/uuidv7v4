use uuid::Uuid;


fn main() {
    // Create a v7 UUID using the timestamp

    loop {
    let v7 = Uuid::now_v7().simple();
    let v4 = Uuid::new_v4().simple();

    let v7_str = v7.to_string();
    let v4_str = v4.to_string();


    let compound_uuid = format!("{}{}", v7_str, v4_str);

    println!("{}", compound_uuid);

    }
}

