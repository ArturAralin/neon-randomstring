var addon = require('../native');
const { generate } = require('randomstring');

const len = 256;
const charset = '123абвгеabce😀';
const count = 1000000;

const result = addon.randomString(len, charset);
console.log(result);

// return;

console.time('!')
for (let i = 0; i < count; i++) {
  const rr = addon.randomString(len, charset);
}
console.timeEnd('!')


console.time('!!')
for (let i = 0; i < count; i++) {
  generate({
    length: len,
    charset,
  })
}
console.timeEnd('!!')