const { assert } = require('console')
const fs = require('fs')
const path = require('path')

const icons = fs.readFileSync('./icons-input.c', 'utf-8')

const width = Number(icons.match(/#define .*_FRAME_WIDTH (\d+)/)[1])
const height = Number(icons.match(/#define .*_FRAME_HEIGHT (\d+)/)[1])

assert(width === height, `Icons must be square-sized, got icons ${width}x${height}`)

const size = width;
const struct_name = `Icons${size}`;
const filename = `icons${size}.rs`;
const filepath = path.join('src', 'icons', filename);

const padding = Math.max(0, 8 - size)
const data_width = Math.min(8, size)
const byte_regexp = new RegExp(`\\d{${data_width}}`, 'g');

console.log(`Pad zero: ${padding}; Data width in byte: ${data_width}; Match bytes by: ${byte_regexp}`)

const icons_data = [...icons.matchAll(/(?<icon>\{[^\{\}]*\})/igm)]
    .map(matches => matches.groups.icon.replaceAll(/[\{\}\s]/g, '')
        .split(',')
        .map(byte => {
            if (byte === '0xff000000') {
                return '1'
            } else if (byte === '0x00000000') {
                return '0'
            } else {
                throw new Error(`Invalid data byte: ${byte}. Should be '0xff000000' or '0x00000000'`)
            }
        })
        .join('')
        .replaceAll(byte_regexp, (match) => `0b${match}${'0'.repeat(padding)},`)
        .split(',')
        .filter(row => !!row.length)
    );

const indent = ind => ' '.repeat(4 * ind)
const drawComment = (ind, data) => `${indent(ind)}/*${'*'.repeat(size + padding * 2 + 1)}
${data.map(row => `${indent(ind)} *${' '.repeat(padding)}${row.replaceAll(/0b/g, '').replaceAll(/\d/g, m => !!Number(m) ? '#' : ' ')}*`).join('\n')}
${indent(ind)} ${'*'.repeat(size + padding * 2 + 1)}*/`

const result = `
use crate::make_icon_set;

make_icon_set! {
    pub ${struct_name}: ${size} {
${icons_data.map(data => `${drawComment(2, data)}\n${indent(2)}ICON_NAME: method_name = &[\n${data.map(row => `${indent(3)}${row}`).join(',\n')}\n${indent(2)}]`).join(',\n\n')}
    }
}
`.trim();

if (fs.existsSync(filepath)) {
    console.log(`Icons with same size already exists. Renaming old file...`);
    const fileDir = path.dirname(filepath)
    const filenameWoExt = path.basename(filename, '.rs')
    fs.renameSync(filepath, path.join(fileDir, `${filenameWoExt}_${new Date().toISOString().split('.')[0]}.old.rs`))
}

fs.writeFileSync(filepath, result, 'utf-8')
