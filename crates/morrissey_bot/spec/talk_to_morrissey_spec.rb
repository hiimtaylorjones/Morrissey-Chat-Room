require "morrissey_bot"

describe "MorrisseyBot" do
  it "can respond to stuff he doesn't know" do
    expect(MorrisseyBot.translate("What's your favorite Smiths' song?")).to eq("I dont' have much to say about this.")
  end

  it "can respond to the right phrase" do
    expect(MorrisseyBot.translate("I don't like some people")).to eq("In my life, why do I smile At people who I'd much rather kick in the eye?")
  end

  it "can say hello" do
    expect(MorrisseyBot.hello).to eq("Hello from morrissey_bot!")
  end
end
