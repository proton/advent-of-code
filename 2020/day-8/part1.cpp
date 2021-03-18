#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <set>
#include <unistd.h>

using namespace std;

enum command_type { acc, jmp, nop };
struct command {
  command_type type;
  int argument;
};

int main() {
  vector<command> commands;
  set<uint> used_commands;

  ifstream file("input.txt");
  string line; 
  while (getline(file, line)) {
    command cmd;
    switch(line[0]) {
      case 'a': cmd.type = command_type::acc; break;
      case 'j': cmd.type = command_type::jmp; break;
      case 'n': cmd.type = command_type::nop; break;
    }
    cmd.argument = stoi(line.substr(4), nullptr);
    commands.push_back(cmd);
  }

  int accumulator = 0;
  uint i = 0;
  while(i < commands.size()) {
    const bool cmd_already_used = used_commands.find(i) != used_commands.end();
    if (cmd_already_used) {
      cout << accumulator << "\n";
      break;
    }
    used_commands.insert(i);

    auto cmd = commands[i];
    switch (cmd.type) {
      case command_type::acc:
        accumulator += cmd.argument;
        ++i;
        break;
      case command_type::jmp:
        i += cmd.argument;
        break;
      case command_type::nop:
        ++i;
using namespace std;
        break;
    }
  }
}