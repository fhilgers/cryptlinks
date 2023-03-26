#!/usr/bin/env python

"""
Pandoc filter to convert all level 2+ headings to paragraphs with
emphasized text.
"""

from pandocfilters import toJSONFilter, Emph, Para, Image

import subprocess
import json


#expanded = { "pandoc-api-version": [1, 22, 2, 1], "meta": {}, "blocks": [{"t": "Plain", "c": [{"t": key, "c": value}]}]}
#unconvert = subprocess.run(["pandoc", "-f", "json"], text=True, stdout=subprocess.PIPE, input=json.dumps(expanded))
# right {"t": "Image", "c": [["", [], []], [{"t": "Str", "c": "image"}], ["./out.png", ""]]}]]}
# wrong {"t": "Image", "c": [["", [], []], [], ["./qrcode.png", ""]]}
# {"t":"Span","c":[["",[],[["encrypted","true"]]],[{"t":"Str","c":"abc"}]]}
#{"t": "Span", "c": [['', [], [['encrypted', 'true']]], [{'t': 'Str', 'c': 'abc'}, {'t': 'Space'}, {'t': 'Emph', 'c': [{'t': 'Str', 'c': 'xyz'}]}, {'t': 'Space'}, {'t': 'Str', 'c': 'abc'}, {'t': 'Space'}, {'t': 'Str', 'c': 'fyz'}, {'t': 'Space'}, {'t': 'Image', 'c': [['', [], []], [{'t': 'Str', 'c': 'image'}], ['./out.png', '']]}]]}

# {"pandoc-api-version": [1, 22, 2, 1], "meta": {}, "blocks": [{"t": "Plain", "c": [{"t": "Span", "c": [["", [], [["encrypted", "true"]]], [{"t": "Str", "c": "abc"}, {"t": "Space"}, {"t": "Emph", "c": [{"t": "Str", "c": "xyz"}]}, {"t": "Space"}, {"t": "Str", "c": "abc"}, {"t": "Space"}, {"t": "Str", "c": "fyz"}, {"t": "Space"}, {"t": "Image", "c": [["", [], []], [{"t": "Str", "c": "image"}], ["./out.png", ""]]}]]}]}]}

count = 0


def create_qrcode(text, password, path):
    return subprocess.run(["../../target/release/cryptlinks", "qrcode", "--password", password, "--output", path, "--link", "https://cryptlinks.fly.dev/"],
                          text=True, stdout=subprocess.PIPE, input=text)


def create_image(path):
    return Image(("", ["encrypted"], []), [], [path, ""])


def behead(key, value, format, meta):
    global count
    path = f"/tmp/qrcode{count}.svg"
    try:
        attrs = value[0][1]
    except:
        attrs = []

    match [key, 'encrypted' in attrs]:
        case ['Code', True]:
            count += 1
            create_qrcode(value[1], "password", path)
            return create_image(path)

        case ['CodeBlock', True]:
            count += 1
            create_qrcode(value[1], "password", path)
            return Para([create_image(path)])

        case _:
            return
      

if __name__ == "__main__":
    toJSONFilter(behead)

