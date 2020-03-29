const DEFAULT_CHARSET = require('./charset');
// eslint-disable-next-line import/no-unresolved
const native = require('../native.node');

const matchCharset = (charset) => {
  if (DEFAULT_CHARSET[charset]) {
    return DEFAULT_CHARSET[charset];
  }

  return charset;
};

const applyCapitalization = (type, str) => {
  switch (type) {
    case 'uppercase':
      return str.toUpperCase();
    case 'lowercase':
      return str.toLowerCase();
    default:
      throw new Error(`Unknown capitalization type ${type}`);
  }
};

const generate = (opts) => {
  const {
    length = 32,
    charset = 'alphanumeric',
    capitalization = null,
  } = opts;

  const chars = matchCharset(charset);
  const result = native.randomString(length, chars);

  if (capitalization) {
    return applyCapitalization(capitalization, result);
  }

  return result;
};

module.exports = {
  generate,
};
