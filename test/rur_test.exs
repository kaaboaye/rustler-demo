defmodule RurTest do
  use ExUnit.Case
  doctest Rur

  test "greets the world" do
    assert Rur.hello() == :world
  end
end
