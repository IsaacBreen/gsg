import requests
import zipfile
import io
import xml.etree.ElementTree as ET
from collections import defaultdict

def download_and_extract_zip(url):
    response = requests.get(url)
    with zipfile.ZipFile(io.BytesIO(response.content)) as zip_file:
        xml_content = zip_file.read(zip_file.namelist()[0])
    return xml_content

def parse_xml(xml_content):
    root = ET.fromstring(xml_content)
    char_groups = defaultdict(list)

    for group in root.findall('.//{http://www.unicode.org/ns/2003/ucd/1.0}group'):
        gc = group.get('gc')
        for char in group.findall('{http://www.unicode.org/ns/2003/ucd/1.0}char'):
            if char.get('cp') is None:
                print(f"Warning: Ignoring character with no codepoint: {char.text}")
                continue
            cp = int(char.get('cp'), 16)
            char_groups[gc].append(cp)

    return char_groups

def generate_rust_code(char_groups):
    rust_code = """
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnicodeCategory {
"""

    # Generate enum variants
    for gc in sorted(key for key in char_groups.keys() if key is not None):
        rust_code += f"    {gc},\n"

    rust_code += "}\n\n"

    # Generate lazy_static initialization
    rust_code += """
use lazy_static::lazy_static;

lazy_static! {
    static ref UNICODE_CATEGORIES: HashMap<UnicodeCategory, Vec<char>> = {
        let mut m = HashMap::new();
"""

    for gc, chars in char_groups.items():
        rust_code += f"        m.insert(UnicodeCategory::{gc}, vec![\n"
        for chunk in [chars[i:i + 10] for i in range(0, len(chars), 10)]:
            rust_code += "            " + ", ".join("'\\u{{{0:04X}}}'".format(c) for c in chunk) + ",\n"
        rust_code += "        ]);\n"

    rust_code += """
        m
    };
}

pub fn get_chars_in_category(category: UnicodeCategory) -> &'static [char] {
    UNICODE_CATEGORIES.get(&category).map(|v| v.as_slice()).unwrap_or(&[])
}
"""

    return rust_code

def main():
    url = "https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.all.grouped.zip"
    xml_content = download_and_extract_zip(url)
    char_groups = parse_xml(xml_content)
    rust_code = generate_rust_code(char_groups)

    with open("../unicode_categories.rs", "w", encoding="utf-8") as f:
        f.write(rust_code)

    print("Rust code has been generated and saved to 'unicode_categories.rs'")

if __name__ == "__main__":
    main()