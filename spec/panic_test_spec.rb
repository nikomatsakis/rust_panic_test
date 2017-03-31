require 'panic_test'

describe "PanicTest" do

  it "panics" do
    # This is calling the panic method we defined in src/lib.rs
    expect(Object.new.panic).to be_nil
    expect(Object.new.panic).to be_nil
  end

end