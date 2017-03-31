require 'panic_test'

describe "PanicTest" do

  it "panics" do
    # This is calling the panic method we defined in src/lib.rs
    expect { Object.new.panic }.to raise_error(RuntimeError, "Panicked in Rust")
    expect { Object.new.panic }.to raise_error(RuntimeError, "Panicked in Rust")
  end

end