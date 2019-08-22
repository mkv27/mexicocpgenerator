use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::env::args;

fn main() {
    let limit: i32 = match args().nth(1) {
        Some(x) => x.parse().unwrap(),
        None => 4
    };
    let file = File::open("cps.txt").unwrap();
    let mut count = 0;
    let mut created_count = 0;
    let mut urban_count = 0;
    let mut pc_vec:Vec<String> = Vec::new();
    let mut current_pc:String = String::from("");

    for line in BufReader::new(file).lines() {
        
        count += 1;
        if count == 1 {continue;}
        else if limit > 0 && count > limit {break;}

        let split_value = line.unwrap();
        let split = split_value.split("|");
        let line_vec: Vec<&str> = split.collect();
        let zone_type = line_vec[13];
        let postal_code = line_vec[0];
        
        if zone_type != "Urbano" {continue;}

        urban_count+=1;

        if current_pc == postal_code {
            pc_vec.push(create_json(&line_vec));
        }else{
            if current_pc != "" {
                create_file(&pc_vec, &mut created_count, current_pc)
            }

            current_pc = postal_code.to_string();
            pc_vec.clear();
            pc_vec.push(create_json(&line_vec));
        }
        
    }
    println!("Total cp from txt file: {}",count-1);
    println!("Total urban: {}",urban_count);
    println!("Total files created: {}",created_count);
}

fn create_file(vec: &Vec<String>, created_count: &mut i32, postal_code:String) {
    let file_name = format!("./postal/{}.json",postal_code);
    let mut ofile = File::create(file_name).expect("unable to create file");
    let json = create_file_content(&vec);
    ofile.write_all(json.as_bytes()).expect("unable to write");
    *created_count += 1;
}

fn create_file_content(vec: &Vec<String>) -> String {
    return format!("[{}]",vec.join(","));
}

fn create_json(vec: &Vec<&str>) -> String {
    return format!("{{\
        \"postal_code\":\"{postal_code}\",\
        \"place_name\":\"{place_name}\",\
        \"place_type\":\"{place_type}\",\
        \"county\":\"{county}\",\
        \"state\":\"{state}\",\
        \"city\":\"{city}\"\
        }}",
        postal_code = vec[0],
        place_name = vec[1],
        place_type = vec[2],
        county = vec[3],
        state = vec[4],
        city = vec[5]
    );
}