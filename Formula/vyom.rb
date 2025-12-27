class Vyom < Formula
  desc "A minimalist, transparent music player for the terminal"
  homepage "https://github.com/MrSyr3x/vyom"
  url "https://github.com/MrSyr3x/vyom/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "<REPLACE_WITH_SHA256>"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Simple test to verify version
    assert_match "vyom 0.1.0", shell_output("#{bin}/vyom --version")
  end
end
