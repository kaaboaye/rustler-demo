defmodule Rur do
  @moduledoc """
  Documentation for `Rur`.
  """

  def path do
    Path.join(:code.priv_dir(:rur), "mieszkania_wroclaw.csv")
  end

  def elixir_load do
    path()
    |> File.stream!()
    |> CSV.decode()
    |> Enum.to_list()
  end

  def mem do
    :erlang.garbage_collect()
    prev = :erlang.memory(:total)
    data = elixir_load()
    :erlang.garbage_collect()
    current = :erlang.memory(:total)
    diff = current - prev
    IO.puts("Memory used: #{format_bytes(diff)}")
    length(data)
  end

  def format_bytes(bytes) when bytes < 1024, do: "#{bytes} B"
  def format_bytes(bytes) when bytes < 1_048_576, do: "#{Float.round(bytes / 1024, 2)} KB"

  def format_bytes(bytes) when bytes < 1_073_741_824,
    do: "#{Float.round(bytes / 1_048_576, 2)} MB"

  def format_bytes(bytes), do: "#{Float.round(bytes / 1_073_741_824, 2)} GB"
end
