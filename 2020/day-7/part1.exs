defmodule Solution do
  def run do
    lines = File.read!("input.txt")
      |> String.split("\n")
      |> Enum.map(&parse_line/1)

    IO.puts lines
  end

  defp parse_line(line) do
    line
  end
end

Solution.run
