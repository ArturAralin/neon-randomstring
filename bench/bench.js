const { generate } = require('randomstring');
const { generate: nativeGenerate } = require('../lib');

const len = 256;
const charset = '123абвгеabce😀';
const count = 500000;

const params = {
  length: len,
  charset,
};

console.time('Native');
for (let i = 0; i < count; i += 1) {
  nativeGenerate(params);
}
console.timeEnd('Native');


console.time('Original');
for (let i = 0; i < count; i += 1) {
  generate(params);
}
console.timeEnd('Original');
