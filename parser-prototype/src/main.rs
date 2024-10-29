use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./csv.pest"]
pub struct CSV;

fn main() {
    let input = "-273.15,42.0,3.14\n100,200,300\n";
    let parsed_file = CSV::parse(Rule::file, input);

    match parsed_file {
        Ok(records) => {
            for record in records {
                for inner_record in record.into_inner() {
                    let mut sum = 0.0;
                    let mut field_count = 0;

                    println!("New Record:");
                    for field in inner_record.into_inner() {
                        let field_value = field.as_str();
                        match field_value.parse::<f64>() {
                            Ok(number) => {
                                println!(" - Number: {}", number);
                                sum += number;
                                field_count += 1;
                            }
                            Err(_) => println!(" - Non-numeric field: {}", field_value),
                        }
                    }
                    println!(" Record sum: {}", sum);
                    if field_count > 0 {
                        println!(" Average: {:.2}", sum / field_count as f64);
                    }
                    println!("---------------------");
                }
            }
        }
        Err(e) => println!("Error parsing file: {:?}", e),
    }
}
