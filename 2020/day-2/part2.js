const fs = require('fs');

const isValid = (line) => {
  const [_, pos1, pos2, letter, password] = line.match(/^(\d+)-(\d+) (\S): (\S+)/);
  return (password[pos1 - 1] === letter) ^ (password[pos2 - 1] === letter);
}

const path = 'input.txt';
const lines = fs.readFileSync(path).toString().split('\n');
const validPasswordsCount = lines.filter(isValid).length;

console.log(validPasswordsCount);