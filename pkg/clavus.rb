class Clavus < Formula
    desc "In vetusta bibliotheca, clavum lateris aperiebat fores librarii occultum, ubi sapientia et arcana conservabantur."
    homepage "https://github.com/peter-mbx/clavus"
    url "https://github.com/peter-mbx/clavus/archive/0.0.1.tar.gz"
    sha256 "bb64672b75d2d459063820b65364798ab9569e14e7ee04a1d5ce514c3c5c18b4"
    license "MIT"
  
    depends_on "rust" => :build
  
    def install
      system "cargo", "build", "--release"
      bin.install "target/release/clavus"
    end
  end
