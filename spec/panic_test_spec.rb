require 'panic_test'

describe "PanicTest" do

  it "panics" do
    expect(Object.new.panic).to be_nil
  end

end