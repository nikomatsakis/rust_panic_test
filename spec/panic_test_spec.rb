require 'panic_test'

describe "PanicTest" do

  it "panics" do
    # This is calling the methods we defined in src/lib.rs
    puts "BEFORE RAISE"
    # expect { Object.new.rust_raise }.to raise_error(RuntimeError, "Panicked in Rust")
    puts "AFTER RAISE"
    expect(Object.new.panic).to be_nil
  end

end
