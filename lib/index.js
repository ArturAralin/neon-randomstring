var addon = require('../native');
const { generate } = require('randomstring');

const len = 128;
const alphabet = 'alphanumeric';
const charset = '123абвгеabce😀';
const count = 100000;

const result = addon.randomString(len, alphabet, charset);
console.log(result);

const td = new TextDecoder('utf-8');
const t = td.decode(result)
console.log(t);

// return;

console.time('!')
for (let i = 0; i < count; i++) {
  const rr = addon.randomString(len, alphabet, charset);
  new TextDecoder('utf-8').decode(rr);

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