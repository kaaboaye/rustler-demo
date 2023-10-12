defmodule Deployment do
  def nif_data do
    %{priv_path: :code.priv_dir(:rur) |> IO.iodata_to_binary()}
  end
end
