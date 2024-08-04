import requests
import zipfile
from io import BytesIO
from xml.etree import ElementTree as ET
from collections import defaultdict


def download_and_parse_unicode_data():
    url = "https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.all.flat.zip"
    response = requests.get(url)
    response.raise_for_status()

    with zipfile.ZipFile(BytesIO(response.content)) as zf:
        with zf.open("ucd.all.flat.xml") as xml_file:
            tree = ET.parse(xml_file)

    root = tree.getroot()
    char_data = defaultdict(list)
    for char_element in root.findall(".//{http://www.unicode.org/ns/2003/ucd/1.0}char"):
        cp_str = char_element.get("cp")
        if cp_str is not None:
            cp = int(cp_str, 16)
            gc = char_element.get("gc")
            char_data[gc].append(cp)

    return char_data


def convert_to_ranges(code_points):
    ranges = []
    if code_points:
        start = end = code_points[0]
        for point in code_points[1:]:
            if point == end + 1:
                end = point
            else:
                ranges.append((start, end))
                start = end = point
        ranges.append((start, end))
    return ranges


def generate_rust_code(char_data):
    categories = sorted(char_data.keys())
    rust_code = """
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GeneralCategory {
    """ + ",\n    ".join(categories) + """
}

pub fn char_ranges_in_general_category(category: GeneralCategory) -> &'static [(char, char)] {
    match category {
"""

    for category in categories:
        ranges = convert_to_ranges(sorted(char_data[category]))
        rust_code += f"        GeneralCategory::{category} => &[\n"
        for start, end in ranges:
            if start == end:
                rust_code += f"            ('\\u{{{start:04X}}}', '\\u{{{end:04X}}}'),\n"
            else:
                rust_code += f"            ('\\u{{{start:04X}}}', '\\u{{{end:04X}}}'),\n"
        rust_code += "        ],\n"

    rust_code += """    }
}

pub fn chars_in_general_category(category: GeneralCategory) -> &'static [char] {
    static CHAR_CACHE: std::sync::OnceLock<std::collections::HashMap<GeneralCategory, Vec<char>>> = std::sync::OnceLock::new();

    CHAR_CACHE.get_or_init(|| {
        let mut cache = std::collections::HashMap::new();
        for category in [""" + ", ".join(f"GeneralCategory::{cat}" for cat in categories) + """] {
            let ranges = char_ranges_in_general_category(category);
            let chars: Vec<char> = ranges.iter().flat_map(|&(start, end)| (start as u32..=end as u32).filter_map(std::char::from_u32)).collect();
            cache.insert(category, chars);
        }
        cache
    }).get(&category).unwrap()
}

pub fn general_category_for_char(c: char) -> GeneralCategory {
    match c as u32 {
"""

    all_ranges = []
    for category, points in char_data.items():
        ranges = convert_to_ranges(sorted(points))
        for start, end in ranges:
            all_ranges.append((start, end, category))

    all_ranges.sort()

    for start, end, category in all_ranges:
        if start == end:
            rust_code += f"        0x{start:04X} => GeneralCategory::{category},\n"
        else:
            rust_code += f"        0x{start:04X}..=0x{end:04X} => GeneralCategory::{category},\n"

    rust_code += """        _ => panic!("Unknown general category for char: {}", c),
    }
}
"""

    return rust_code


def main():
    char_data = download_and_parse_unicode_data()
    rust_code = generate_rust_code(char_data)

    with open("../unicode_categories.rs", "w") as f:
        f.write(rust_code)

    print("Rust code has been generated and saved to ../unicode_categories.rs")


if __name__ == "__main__":
    main()