defmodule Solution do
  def run do
    lines = File.read!("input.txt")
      |> String.split("\n")
      |> Enum.map(&parse_line/1)

    IO.puts lines
  end

  defp parse_line(line) do
    re = ~r/^(?<parent>\S+ \S+) bags contain (?<contents>.*)\.$/
    caps = Regex.named_captures(re, line)
    caps["contents"] |> String.split(",")
  end
end

Solution.run

# (stdlib 3.13.2) :io.put_chars(:standard_io, :unicode, [[%{"contents" => "1 bright white bag, 2 muted yellow bags", "parent" => "light red"}, %{"contents" => "3 bright white bags, 4 muted yellow bags", "parent" => "dark orange"}, %{"contents" => "1 shiny gold bag", "parent" => "bright white"}, %{"contents" => "2 shiny gold bags, 9 faded blue bags", "parent" => "muted yellow"}, %{"contents" => "1 dark olive bag, 2 vibrant plum bags", "parent" => "shiny gold"}, %{"contents" => "3 faded blue bags, 4 dotted black bags", "parent" => "dark olive"}, %{"contents" => "5 faded blue bags, 6 dotted black bags", "parent" => "vibrant plum"}, %{"contents" => "no other bags", "parent" => "faded blue"}, %{"contents" => "no other bags", "parent" => "dotted black"}], 10])

# light red bags contain 1 bright white bag, 2 muted yellow bags.
# dark orange bags contain 3 bright white bags, 4 muted yellow bags.
# bright white bags contain 1 shiny gold bag.
# muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
# shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
# dark olive bags contain 3 faded blue bags, 4 dotted black bags.
# vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
# faded blue bags contain no other bags.
# dotted black bags contain no other bags.