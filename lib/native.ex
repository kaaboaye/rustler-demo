defmodule Native do
  use Rustler,
    otp_app: :rur,
    crate: :native,
    load_data_fun: {Deployment, :nif_data}

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def nearest_home(_longitude, _latitude), do: :erlang.nif_error(:nif_not_loaded)
end
