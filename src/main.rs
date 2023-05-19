#[macro_use]
extern crate clap;
#[macro_use]
extern crate simple_error;

use xmltree::Element;
use std::error::Error;
use std::fs;
use clap::App;

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn main() {
  run().unwrap();
}

fn run() -> BoxResult<i32> {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let input_file = matches.value_of("INPUT").unwrap();
  let xml_content = fs::read_to_string(input_file).unwrap();
  let coverage = extract_coverage(xml_content.as_str()).unwrap();
  if coverage.instruction_covered > 0 || coverage.instruction_missed > 0 {
    println!("instruction: {:.2}", coverage.instruction_covered as f32 / (coverage.instruction_missed + coverage.instruction_covered) as f32);
  }
  if coverage.branch_covered > 0 || coverage.branch_missed > 0 {
    println!("branch: {:.2}", coverage.branch_covered as f32 / (coverage.branch_missed + coverage.branch_covered) as f32);
  }
  if coverage.line_covered > 0 || coverage.line_missed > 0 {
    println!("line: {:.2}", coverage.line_covered as f32 / (coverage.line_missed + coverage.line_covered) as f32);
  }
  if coverage.complexity_covered > 0 || coverage.complexity_missed > 0 {
    println!("complexity: {:.2}", coverage.complexity_covered as f32 / (coverage.complexity_missed + coverage.complexity_covered) as f32);
  }
  if coverage.method_covered > 0 || coverage.method_missed > 0 {
    println!("method: {:.2}", coverage.method_covered as f32 / (coverage.method_missed + coverage.method_covered) as f32);
  }
  if coverage.class_covered > 0 || coverage.class_missed > 0 {
    println!("class: {:.2}", coverage.class_covered as f32 / (coverage.class_missed + coverage.class_covered) as f32);
  }
  Ok(0)
}

fn extract_coverage(xml_data: &str) -> BoxResult<Coverage> {
  let mut coverage = Coverage::new();

  let coverage_result = Element::parse(xml_data.as_bytes()).unwrap();
  let children = coverage_result.children;
  for child in children {
    let element = child.as_element().unwrap();
    if element.name == "counter" {
      let attribute_map = &element.attributes;
      let counter_type = attribute_map.get("type").expect("malformed");

      let covered: i64 = attribute_map.get("covered").unwrap().parse().unwrap();
      let missed: i64 = attribute_map.get("missed").expect("malformed").parse().unwrap();
      match counter_type.as_str() {
        "INSTRUCTION" => {
          coverage.instruction_covered = covered;
          coverage.instruction_missed = missed;
        }
        "BRANCH" => {
          coverage.branch_covered = covered;
          coverage.branch_missed = missed;
        }
        "LINE" => {
          coverage.line_covered = covered;
          coverage.line_missed = missed;
        }
        "COMPLEXITY" => {
          coverage.complexity_covered = covered;
          coverage.complexity_missed = missed;
        }
        "METHOD" => {
          coverage.method_covered = covered;
          coverage.method_missed = missed;
        }
        "CLASS" => {
          coverage.class_covered = covered;
          coverage.class_missed = missed;
        }
        _ => bail!("unknown counter type {:?}", counter_type),
      };
    }
  }
  Ok(coverage)
}

struct Coverage {
  branch_covered: i64,
  branch_missed: i64,
  class_covered: i64,
  class_missed: i64,
  complexity_covered: i64,
  complexity_missed: i64,
  instruction_covered: i64,
  instruction_missed: i64,
  line_covered: i64,
  line_missed: i64,
  method_covered: i64,
  method_missed: i64,
}

impl Coverage {
  pub fn new() -> Coverage {
    Coverage {
      branch_covered: 0,
      branch_missed: 0,
      class_covered: 0,
      class_missed: 0,
      complexity_covered: 0,
      complexity_missed: 0,
      instruction_covered: 0,
      instruction_missed: 0,
      line_covered: 0,
      line_missed: 0,
      method_covered: 0,
      method_missed: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_xml() {
    let data: &'static str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?><!DOCTYPE report PUBLIC "-//JACOCO//DTD Report 1.1//EN" "report.dtd">
<report name="test-service">
    <sessioninfo id="object-21061aa9" start="1683895529957" dump="1683895566710"/>
    <counter type="INSTRUCTION" missed="1" covered="2"/>
    <counter type="BRANCH" missed="3" covered="4"/>
    <counter type="LINE" missed="5" covered="6"/>
    <counter type="COMPLEXITY" missed="7" covered="8"/>
    <counter type="METHOD" missed="9" covered="10"/>
    <counter type="CLASS" missed="11" covered="12"/>
</report>
"##;

    let coverage = extract_coverage(&data).unwrap();
    println!("type = instruction covered = {:#?} missed = {:#?}",
             coverage.instruction_missed, coverage.instruction_missed);
  }
}
