const fs = require('fs');

const subStringCount = (string, substring) => string.split(substring).length - 1;

const isValid = (line) => {
  const [_, min, max, letter, password] = line.match(/^(\d+)-(\d+) (\S): (\S+)/);
  const cnt = subStringCount(password, letter);
  return +min <= cnt && cnt <= +max
}

const path = 'input.txt';
const lines = fs.readFileSync(path).toString().split('\n');
const validPasswordsCount = lines.filter(isValid).length;

console.log(validPasswordsCount)