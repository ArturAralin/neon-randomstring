const numbers = '0123456789';
const letters = 'abcdefghijklmnopqrstuvwxyz';
const uppercaseLetters = letters.toUpperCase();
const hex = `${numbers}abcdef`;

module.exports = {
  alphanumeric: [numbers, letters, uppercaseLetters].join(''),
  alphabetic: [letters, uppercaseLetters].join(''),
  numeric: numbers,
  hex,
};
