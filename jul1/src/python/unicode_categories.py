import requests
import zipfile
import os
import xml.etree.ElementTree as ET
from collections import defaultdict

# Step 1: Download the Unicode XML file
url = 'https://www.unicode.org/Public/UCD/latest/ucdxml/ucd.all.flat.zip'
response = requests.get(url)
zip_path = 'ucd.all.flat.zip'

with open(zip_path, 'wb') as f:
    f.write(response.content)

# Step 2: Unzip the downloaded file
with zipfile.ZipFile(zip_path, 'r') as zip_ref:
    zip_ref.extractall('.')

# Step 3: Parse the XML file
xml_path = 'ucd.all.flat.xml'
tree = ET.parse(xml_path)
root = tree.getroot()

# Define the namespace
ns = {'ns': 'http://www.unicode.org/ns/2003/ucd/1.0'}

# Step 4: Extract character ranges by general category
categories = defaultdict(list)

for char in root.findall('.//ns:char', ns):
    cp = char.get('cp')
    first_cp = char.get('first-cp')
    last_cp = char.get('last-cp')
    gc = char.get('gc')

    if not gc:
        continue

    if first_cp and last_cp:
        categories[gc].append((int(first_cp, 16), int(last_cp, 16)))
    elif cp:
        code_point = int(cp, 16)
        categories[gc].append((code_point, code_point))

# Step 5: Generate Rust code
rust_code = """
#[derive(Debug, PartialEq, Eq)]
pub enum GeneralCategory {{
    {}
}}

pub fn get_general_category(c: char) -> Option<GeneralCategory> {{
    let code_point = c as u32;
    match code_point {{
        {}
        _ => None,
    }}
}}
"""

category_variants = []
match_arms = []

for category, ranges in categories.items():
    variant = category.upper()
    category_variants.append(variant)

    for start, end in ranges:
        if start == end:
            match_arms.append(f"{start} => Some(GeneralCategory::{variant}),")
        else:
            match_arms.append(f"{start}..={end} => Some(GeneralCategory::{variant}),")

category_variants_code = ",\n    ".join(category_variants)
match_arms_code = "\n        ".join(match_arms)

rust_code = rust_code.format(category_variants_code, match_arms_code)

# Write the generated Rust code to a file
output_path = '../unicode_general_category.rs'
with open(output_path, 'w') as f:
    f.write(rust_code)

print(f"Rust code has been generated and written to {output_path}")

# Clean up
os.remove(zip_path)
os.remove(xml_path)