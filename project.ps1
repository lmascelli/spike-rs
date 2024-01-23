function Script:Init
{
  cargo add hdf5-sys --features=static,threadsafe
  cargo add hdf5
}

Clear-Host

cargo build
