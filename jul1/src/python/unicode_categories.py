import requests
import zipfile
import io
from xml.etree import ElementTree as ET

def main():
    url = "https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.all.flat.zip"
    response = requests.get(url)
    response.raise_for_status()

    zip_file = zipfile.ZipFile(io.BytesIO(response.content))
    xml_content = zip_file.read("ucd.all.flat.xml").decode("utf-8")

    root = ET.fromstring(xml_content)
    categories = {}
    for char_element in root.findall(".//{http://www.unicode.org/ns/2003/ucd/1.0}char"):
        codepoint_str = char_element.get("cp")
        if codepoint_str is not None:
            codepoint = int(codepoint_str, 16)
            category = char_element.get("gc")
            if category not in categories:
                categories[category] = []
            categories[category].append(codepoint)

    rust_code = """
// Generated from Unicode data
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GeneralCategory {
    """ + ",\n    ".join([f"{category}" for category in categories]) + """
}

pub fn general_category(c: char) -> GeneralCategory {
    let codepoint = c as u32;
    """

    for category, codepoints in categories.items():
        ranges = to_ranges(codepoints)
        rust_code += f"    if "
        for i, (start, end) in enumerate(ranges):
            if i > 0:
                rust_code += " || "
            rust_code += f"({start}..={end}).contains(&codepoint)"
        rust_code += f" {{\n        return GeneralCategory::{category};\n    }}\n"

    rust_code += """
    GeneralCategory::Cn
}

pub fn chars_in_general_category(category: GeneralCategory) -> Vec<char> {
    match category {
"""

    for category, codepoints in categories.items():
        rust_code += f"        GeneralCategory::{category} => {{\n"
        rust_code += f"            let mut result = Vec::new();\n"
        for codepoint in codepoints:
            rust_code += f"            if let Some(c) = std::char::from_u32({codepoint}) {{\n"
            rust_code += f"                result.push(c);\n"
            rust_code += f"            }}\n"
        rust_code += f"            result\n"
        rust_code += f"        }},\n"

    rust_code += """
    }
}

fn to_ranges(codepoints: Vec<u32>) -> Vec<(u32, u32)> {
    let mut ranges = Vec::new();
    if codepoints.is_empty() {
        return ranges;
    }

    let mut start = codepoints[0];
    let mut end = codepoints[0];
    for &codepoint in codepoints.iter().skip(1) {
        if codepoint == end + 1 {
            end = codepoint;
        } else {
            ranges.push((start, end));
            start = codepoint;
            end = codepoint;
        }
    }
    ranges.push((start, end));
    ranges
}
"""

    with open("../unicode_general_category.rs", "w") as f:
        f.write(rust_code)


def to_ranges(codepoints):
    ranges = []
    if not codepoints:
        return ranges

    start = codepoints[0]
    end = codepoints[0]
    for codepoint in codepoints[1:]:
        if codepoint == end + 1:
            end = codepoint
        else:
            ranges.append((start, end))
            start = codepoint
            end = codepoint
    ranges.append((start, end))
    return ranges


if __name__ == "__main__":
    main()