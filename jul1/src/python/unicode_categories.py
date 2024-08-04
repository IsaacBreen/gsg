import requests
import zipfile
from io import BytesIO
from xml.etree import ElementTree as ET
from collections import defaultdict

# Download and extract the Unicode data
url = "https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.all.flat.zip"
response = requests.get(url)
response.raise_for_status()

with zipfile.ZipFile(BytesIO(response.content)) as zf:
    with zf.open("ucd.all.flat.xml") as xml_file:
        tree = ET.parse(xml_file)

# Parse the XML data
root = tree.getroot()
char_data = defaultdict(list)
general_categories = set()

for char_element in root.findall(".//{http://www.unicode.org/ns/2003/ucd/1.0}char"):
    cp_str = char_element.get("cp")
    if cp_str is not None:  # Add this check
        cp = int(cp_str, 16)
        gc = char_element.get("gc")
        char_data[gc].append(cp)
        general_categories.add(gc)

# Convert code points to ranges for each category
def convert_to_ranges(code_points):
    ranges = []
    if code_points:
        start = code_points[0]
        end = start
        for code_point in code_points[1:]:
            if code_point == end + 1:
                end = code_point
            else:
                ranges.append((start, end))
                start = code_point
                end = start
        ranges.append((start, end))
    return ranges

for category in char_data:
    char_data[category] = convert_to_ranges(sorted(char_data[category]))

# Generate the Rust code
rust_code = """
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GeneralCategory {
"""

for category in sorted(general_categories):
    rust_code += f"    {category},\n"

rust_code += """
}

pub fn chars_in_general_category(category: GeneralCategory) -> Vec<char> {
    let unicode_ranges = create_unicode_ranges();
    let category_str = match category {
"""

for category in sorted(general_categories):
    rust_code += f"        GeneralCategory::{category} => \"{category}\",\n"

rust_code += """
    };

    let mut chars = Vec::new();
    if let Some(ranges) = unicode_ranges.get(category_str) {
        for &(start, end) in ranges {
            for cp in start..=end {
                if let Some(c) = std::char::from_u32(cp as u32) {
                    chars.push(c);
                }
            }
        }
    }
    chars
}

fn create_unicode_ranges() -> HashMap<&'static str, Vec<(u32, u32)>> {
    let mut unicode_ranges: HashMap<&'static str, Vec<(u32, u32)>> = HashMap::new();

"""

for category, ranges in char_data.items():
    rust_code += f"    unicode_ranges.insert(\"{category}\", vec![\n"
    for start, end in ranges:
        rust_code += f"        (0x{start:04X}, 0x{end:04X}),\n"
    rust_code += "    ]);\n\n"

rust_code += "    unicode_ranges\n"
rust_code += "}\n"

# Save the generated Rust code to a file
with open("generated_unicode_ranges.rs", "w") as file:
    file.write(rust_code)

print("Rust code has been generated and saved to generated_unicode_ranges.rs")