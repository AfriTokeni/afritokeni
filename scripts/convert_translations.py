#!/usr/bin/env python3
"""
Convert TypeScript translations to Rust
"""
import re
import json

# Read the TS file
with open('src/lib/services/translations.ts', 'r') as f:
    content = f.read()

# Extract the translations object
match = re.search(r'private static translations: Translations = \{(.*?)\};', content, re.DOTALL)
if not match:
    print("Could not find translations object")
    exit(1)

translations_str = match.group(1)

# Parse each translation entry
entries = []
current_key = None
current_langs = {}

for line in translations_str.split('\n'):
    line = line.strip()
    
    # Match key: {
    key_match = re.match(r'(\w+):\s*\{', line)
    if key_match:
        if current_key and current_langs:
            entries.append((current_key, current_langs))
        current_key = key_match.group(1)
        current_langs = {}
        continue
    
    # Match en: "text",
    lang_match = re.match(r'(en|lg|sw):\s*"(.+)",?', line)
    if lang_match:
        lang = lang_match.group(1)
        text = lang_match.group(2).replace('\\n', '\\n').replace('\\"', '\\"')
        current_langs[lang] = text
        continue
    
    # Match },
    if line == '},' and current_key and current_langs:
        entries.append((current_key, current_langs))
        current_key = None
        current_langs = {}

# Add last entry
if current_key and current_langs:
    entries.append((current_key, current_langs))

print(f"Found {len(entries)} translation entries")

# Generate Rust code
rust_code = []
for key, langs in entries:
    en = langs.get('en', key).replace('"', '\\"')
    lg = langs.get('lg', key).replace('"', '\\"')
    sw = langs.get('sw', key).replace('"', '\\"')
    
    rust_code.append(f'            ("{key}", Language::English) => "{en}",')
    rust_code.append(f'            ("{key}", Language::Luganda) => "{lg}",')
    rust_code.append(f'            ("{key}", Language::Swahili) => "{sw}",')
    rust_code.append('')

# Write to file
with open('src/satellite/src/translations_generated.txt', 'w') as f:
    f.write('\n'.join(rust_code))

print(f"Generated {len(rust_code)} lines of Rust code")
print("Output written to src/satellite/src/translations_generated.txt")
