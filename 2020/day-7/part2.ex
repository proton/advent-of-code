default_counter_fn = fn(_counters) -> 1 end
counters = Map.new
{:ok, content} = File.read("input.txt")
counters = content
  |> String.split("\n")
  |> Enum.reduce(counters, fn(line, counters) ->
    
    m = Regex.named_captures(~r/(?<parent>.+?) bags contain (?<children>.*)\./, line)
    parent = m["parent"]
    children = Regex.scan(~r/(\d*) (\w+ \w+) bag/, m["children"])

    get_fn = fn(counters) ->
      cnt = children |> Enum.map(fn([_, n, child]) ->
        String.to_integer(n) * Map.get(counters, child, default_counter_fn).(counters)
      end) |> Enum.sum
      cnt + 1
    end

    Map.put(counters, parent, get_fn)
  end)

cnt = counters["shiny gold"].(counters) - 1

IO.puts cnt