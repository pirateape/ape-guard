# typed: false
# frozen_string_literal: true

# ApeGuard — One-command security posture assessment
# Homebrew formula: install with:
#   brew install --formula path/to/apeguard.rb
# Or add to a custom tap:
#   brew tap apeguard/homebrew-tap
#   brew install apeguard

class Apeguard < Formula
  desc "One-command security posture assessment — layered scans, Zero Trust mapping, multi-audience reports"
  homepage "https://github.com/apeguard/cli"
  url "https://github.com/apeguard/cli/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "REPLACE_ME_WITH_ACTUAL_SHA256"
  license "EL-2.0"
  head "https://github.com/apeguard/cli.git", branch: "main"

  depends_on "rust" => :build
  depends_on "gitleaks" => :recommended
  depends_on "semgrep" => :recommended
  depends_on "trivy" => :recommended
  depends_on "nuclei" => :optional

  def install
    system "cargo", "install", *std_cargo_args
    bash_completion.install "completions/apeguard.bash" if Dir.exist?("completions")
    fish_completion.install "completions/apeguard.fish" if Dir.exist?("completions")
    zsh_completion.install "completions/_apeguard" if Dir.exist?("completions")
  end

  test do
    output = shell_output("#{bin}/apeguard version 2>&1")
    assert_match "ApeGuard v", output
    assert_match "License: EL-2.0", output
  end
end
