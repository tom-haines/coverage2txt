use xmltree::Element;

fn main() {
  println!("hello world");
}

fn extract_coverage(xml_data: &str) -> Vec<Coverage> {
  let mut result: Vec<Coverage> = Vec::new();

  let coverage_result = Element::parse(xml_data.as_bytes()).unwrap();
  let children = coverage_result.children;
  for child in children {
    let element = child.as_element().unwrap();
    if element.name == "counter" {
      let attribute_map = &element.attributes;
      let counter_type = attribute_map.get("type").expect("malformed");
      let covered = attribute_map.get("covered").expect("malformed");
      let missed = attribute_map.get("missed").expect("malformed");
      result.push(Coverage::new(counter_type.to_string(), covered.to_string(), missed.to_string()));
    }
  }
  result
}

struct Coverage {
  counter_type: String,
  missed: String,
  covered: String,
}

impl Coverage {
  pub fn new(counter_type: String, missed: String, covered: String) -> Coverage {
    Coverage {
      counter_type,
      covered,
      missed,
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
</report>
"##;

    let coverages = extract_coverage(&data);
    for coverage in coverages {
      println!("type = {:#?} covered = {:#?} missed = {:#?}",
               coverage.counter_type, coverage.covered, coverage.missed);
    }
  }

}
