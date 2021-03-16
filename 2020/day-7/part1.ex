default_counter_fn = fn(_counters) -> 0 end
counters = Map.new
{:ok, content} = File.read("input.txt")
counters = content
  |> String.split("\n")
  |> Enum.reduce(counters, fn(line, counters) ->
    
    m = Regex.named_captures(~r/(?<parent>.+?) bags contain (?<children>.*)\./, line)
    parent = m["parent"]
    children = Regex.scan(~r/(\d*) (\w+ \w+) bag/, m["children"])
      |> Enum.map(&List.last/1)

    Map.put(
      counters,
      parent,
      fn(counters) -> children |> Enum.map(fn(key) -> Map.get(counters, key, default_counter_fn).(counters) end) |> Enum.count(fn(cnt) -> cnt > 0 end) end
    )
  end)

counters = Map.put(counters, "shiny gold", fn(_counters) -> 1 end)

cnt = counters |> Map.values |> Enum.map(fn(children_count) -> children_count.(counters) end) |> Enum.count(fn(cnt) -> cnt > 0 end)
cnt = cnt - 1

IO.puts cnt