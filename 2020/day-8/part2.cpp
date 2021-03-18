#include <iostream>
#include <fstream>
#include <vector>
#include <list>
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

  for(uint j = 0; j < commands.size(); ++j) {
    auto bugged_cmd = commands[j];
    command fixed_cmd;
    if (bugged_cmd.type == command_type::jmp) fixed_cmd.type = command_type::nop;
    else if (bugged_cmd.type == command_type::nop) fixed_cmd.type = command_type::jmp;
    else continue;

    fixed_cmd.argument = bugged_cmd.argument;
    auto fixed_commands = vector<command>(commands);
    fixed_commands[j] = fixed_cmd;

    set<uint> used_commands;
    int accumulator = 0;
    uint i = 0;
    while(i < fixed_commands.size()) {
      const bool cmd_already_used = used_commands.find(i) != used_commands.end();
      if (cmd_already_used) break;
      used_commands.insert(i);

      auto cmd = fixed_commands[i];
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
    if (i == fixed_commands.size()) 
      cout << accumulator << "\n";
  }
}