# defmodule Aoc do
#   def parse_line(line, counters) do
#     m = Regex.named_captures(~r/(?<parent>.+?) bags contain (?<children>.*)\./, line)
#     Regex.scan(~r/(\d*) (\w+ \w+) bag/, m["children"])

#     IO.puts m["parent"]
#     IO.puts 1
#     h = Map.put_new(h, "bbb", 4)
#   end
# end

default_counter_fn = fn(_counters) -> 0 end
counters = Map.new
{:ok, content} = File.read("input.txt")
counters = content
  |> String.split("\n")
  # |> Enum.map(&Aoc.parse_line/1)
  |> Enum.reduce(counters, fn(line, counters) ->
    
    m = Regex.named_captures(~r/(?<parent>.+?) bags contain (?<children>.*)\./, line)
    parent = m["parent"]
    children = Regex.scan(~r/(\d*) (\w+ \w+) bag/, m["children"])
      |> Enum.map(&List.last/1)

    # h = Map.put_new(h, "bbb", 4)

    Map.put(
      counters,
      parent,
      fn(counters) -> children |> Enum.map(fn(key) -> Map.get(counters, key, default_counter_fn).(counters) end) |> Enum.sum end
    )
  end)

counters = Map.put(counters, "shiny gold", fn(_counters) -> 1 end)
# counters = Map.put(counters, "shiny gold", 3331)

IO.puts inspect(counters)

cnt = counters |> Map.values |> Enum.map(fn(children_count) -> children_count.(counters) end) |> Enum.sum

IO.puts inspect(cnt)