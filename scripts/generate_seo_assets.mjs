import fs from "node:fs";
import zlib from "node:zlib";

const font = {
  " ": ["00000", "00000", "00000", "00000", "00000", "00000", "00000"],
  "-": ["00000", "00000", "00000", "11111", "00000", "00000", "00000"],
  ":": ["00000", "01100", "01100", "00000", "01100", "01100", "00000"],
  A: ["01110", "10001", "10001", "11111", "10001", "10001", "10001"],
  B: ["11110", "10001", "10001", "11110", "10001", "10001", "11110"],
  C: ["01111", "10000", "10000", "10000", "10000", "10000", "01111"],
  D: ["11110", "10001", "10001", "10001", "10001", "10001", "11110"],
  E: ["11111", "10000", "10000", "11110", "10000", "10000", "11111"],
  F: ["11111", "10000", "10000", "11110", "10000", "10000", "10000"],
  G: ["01111", "10000", "10000", "10011", "10001", "10001", "01111"],
  H: ["10001", "10001", "10001", "11111", "10001", "10001", "10001"],
  I: ["11111", "00100", "00100", "00100", "00100", "00100", "11111"],
  K: ["10001", "10010", "10100", "11000", "10100", "10010", "10001"],
  L: ["10000", "10000", "10000", "10000", "10000", "10000", "11111"],
  M: ["10001", "11011", "10101", "10101", "10001", "10001", "10001"],
  N: ["10001", "11001", "10101", "10011", "10001", "10001", "10001"],
  O: ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
  P: ["11110", "10001", "10001", "11110", "10000", "10000", "10000"],
  R: ["11110", "10001", "10001", "11110", "10100", "10010", "10001"],
  S: ["01111", "10000", "10000", "01110", "00001", "00001", "11110"],
  T: ["11111", "00100", "00100", "00100", "00100", "00100", "00100"],
  U: ["10001", "10001", "10001", "10001", "10001", "10001", "01110"],
  V: ["10001", "10001", "10001", "10001", "10001", "01010", "00100"],
  W: ["10001", "10001", "10001", "10101", "10101", "10101", "01010"],
  Y: ["10001", "10001", "01010", "00100", "00100", "00100", "00100"],
};

function crc32(buffer) {
  if (!crc32.table) {
    crc32.table = new Uint32Array(256);
    for (let index = 0; index < 256; index += 1) {
      let value = index;
      for (let bit = 0; bit < 8; bit += 1) {
        value = value & 1 ? 0xedb88320 ^ (value >>> 1) : value >>> 1;
      }
      crc32.table[index] = value >>> 0;
    }
  }

  let value = 0xffffffff;
  for (const byte of buffer) {
    value = crc32.table[(value ^ byte) & 0xff] ^ (value >>> 8);
  }
  return (value ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const typeBuffer = Buffer.from(type, "ascii");
  const length = Buffer.alloc(4);
  const checksum = Buffer.alloc(4);
  length.writeUInt32BE(data.length);
  checksum.writeUInt32BE(crc32(Buffer.concat([typeBuffer, data])));
  return Buffer.concat([length, typeBuffer, data, checksum]);
}

function png(width, height, rgba) {
  const raw = Buffer.alloc((width * 4 + 1) * height);
  for (let y = 0; y < height; y += 1) {
    const row = y * (width * 4 + 1);
    raw[row] = 0;
    rgba.copy(raw, row + 1, y * width * 4, (y + 1) * width * 4);
  }

  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;

  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk("IHDR", ihdr),
    chunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

function color(hex) {
  return [
    Number.parseInt(hex.slice(1, 3), 16),
    Number.parseInt(hex.slice(3, 5), 16),
    Number.parseInt(hex.slice(5, 7), 16),
    255,
  ];
}

function canvas(width, height, background) {
  const data = Buffer.alloc(width * height * 4);
  const [red, green, blue, alpha] = color(background);
  for (let index = 0; index < data.length; index += 4) {
    data[index] = red;
    data[index + 1] = green;
    data[index + 2] = blue;
    data[index + 3] = alpha;
  }
  return { data, height, width };
}

function pixel(target, x, y, rgba) {
  if (x < 0 || y < 0 || x >= target.width || y >= target.height) {
    return;
  }
  const index = (y * target.width + x) * 4;
  target.data[index] = rgba[0];
  target.data[index + 1] = rgba[1];
  target.data[index + 2] = rgba[2];
  target.data[index + 3] = rgba[3];
}

function rect(target, x, y, width, height, rgba) {
  for (let yy = Math.max(0, y); yy < Math.min(target.height, y + height); yy += 1) {
    for (let xx = Math.max(0, x); xx < Math.min(target.width, x + width); xx += 1) {
      pixel(target, xx, yy, rgba);
    }
  }
}

function roundRect(target, x, y, width, height, radius, rgba) {
  for (let yy = y; yy < y + height; yy += 1) {
    for (let xx = x; xx < x + width; xx += 1) {
      const dx = xx < x + radius ? x + radius - xx : xx >= x + width - radius ? xx - (x + width - radius - 1) : 0;
      const dy = yy < y + radius ? y + radius - yy : yy >= y + height - radius ? yy - (y + height - radius - 1) : 0;
      if (dx * dx + dy * dy <= radius * radius) {
        pixel(target, xx, yy, rgba);
      }
    }
  }
}

function line(target, x0, y0, x1, y1, thickness, rgba) {
  const dx = x1 - x0;
  const dy = y1 - y0;
  const steps = Math.max(Math.abs(dx), Math.abs(dy));
  for (let step = 0; step <= steps; step += 1) {
    const x = Math.round(x0 + (dx * step) / steps);
    const y = Math.round(y0 + (dy * step) / steps);
    rect(target, x - Math.floor(thickness / 2), y - Math.floor(thickness / 2), thickness, thickness, rgba);
  }
}

function text(target, value, x, y, scale, rgba) {
  let cursor = x;
  for (const char of value.toUpperCase()) {
    const glyph = font[char] ?? font[" "];
    for (let gy = 0; gy < glyph.length; gy += 1) {
      for (let gx = 0; gx < glyph[gy].length; gx += 1) {
        if (glyph[gy][gx] === "1") {
          rect(target, cursor + gx * scale, y + gy * scale, scale, scale, rgba);
        }
      }
    }
    cursor += char === " " ? scale * 4 : scale * 6;
  }
}

const ink = color("#26342f");
const cream = color("#f7f4ef");
const gold = color("#e9c46a");
const white = color("#fffdfa");
const darkText = color("#202124");
const muted = color("#60646c");
const soft = color("#c8d0cc");

const og = canvas(1200, 630, "#f7f4ef");
roundRect(og, 72, 72, 1056, 486, 28, ink);
roundRect(og, 118, 118, 108, 108, 20, gold);
line(og, 145, 172, 168, 195, 16, ink);
line(og, 164, 195, 212, 147, 16, ink);
text(og, "OPT-OUT DESK", 270, 144, 9, cream);
text(og, "DATA BROKER OPT-OUT CHECKLIST", 272, 228, 5, soft);

[
  ["SEARCH", "FIND LISTINGS"],
  ["REMOVE", "TRACK STEPS"],
  ["RECHECK", "REPEAT LATER"],
].forEach(([title, subtitle], index) => {
  const x = 118 + index * 332;
  roundRect(og, x, 302, 300, 136, 16, white);
  text(og, title, x + 32, 340, 6, darkText);
  text(og, subtitle, x + 32, 394, 3, muted);
});
text(og, "BROWSER LOCALSTORAGE ONLY", 118, 498, 5, soft);
fs.writeFileSync("og-image.png", png(og.width, og.height, og.data));

const touch = canvas(180, 180, "#26342f");
roundRect(touch, 34, 34, 112, 112, 18, gold);
line(touch, 62, 91, 78, 107, 12, ink);
line(touch, 76, 107, 120, 63, 12, ink);
fs.writeFileSync("apple-touch-icon.png", png(touch.width, touch.height, touch.data));
