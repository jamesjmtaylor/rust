# Starcraft Broodwar Rust bot

This documents the steps that I took to build my own Starcraft bot in Rust. A list of references is provided at the end.

## Process

1. Create an [Ubuntu 22.04 partition](https://help.ubuntu.com/stable/ubuntu-help/disk-partitions.html.en) on my personal computer.  I did this because I didn't particularly want to start installing games on my work laptop, and BWAPI-C is best supported on Linux.
1. Install [Git](https://www.digitalocean.com/community/tutorials/how-to-install-git-on-ubuntu-22-04). I used `sudo apt install git`
1. Configure a new [GitHub SSH certificate](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)
1. Install [VS Code](https://itslinuxfoss.com/how-to-install-visual-studio-code-on-ubuntu-22-04/).
1. Install [cmake](https://cmake.org/).
1. Install [Rust](https://www.rust-lang.org/tools/install).  
1. Install [Wine](https://wine.htmlvalidator.com/install-wine-on-ubuntu-22.04.html).  I used `sudo apt install --install-recommends winehq-stable`
1. Download StarCraft 1.16.1 (I just did a google search, since Battlenet doesn't specifically have version 1.16.1).
1. Run the StarCraft.exe with Wine with `wine StarCraft.exe `
1. Install the latest [BWAPI](https://github.com/bwapi/bwapi/releases).  I downloaded and extracted the BWAPI.7z archive and placed it in my Starcraft directory.
1. Git clone and build [BWAPI-C](https://github.com/RnDome/bwapi-c):

```
Github/ $ git clone git@github.com:RnDome/bwapi-c.git && cd bwapi-c
Github/bwapi-c/ $ mkdir build && cd build
Github/bwapi-c/build $ cmake .. -DBWAPI_PATH=~/Downloads/BWAPI
Github/bwapi-c/build $ make
```

## Gotchas

1. When I first installed Rust I made the mistake of just using `sudo apt install rustc`.  This only gets you to rust version 1.61.0, but I eventually found that I needed at least 1.64.0.  I had to find and remove rustc (using `which rustc` to find the executable), and then reinstall using `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
1. When I first installed cmake I made the same mistake, and just used `sudo apt install cmake`.  Trying to subsequently run cmake caused the error 
```
CMake Error at CMakeLists.txt:2 (project):
  No CMAKE_CXX_COMPILER could be found.

  Tell CMake where to find the compiler by setting either the environment
  variable "CXX" or the CMake cache entry CMAKE_CXX_COMPILER to the full path
  to the compiler, or to the compiler name if it is in the PATH.
```
So I had to remove the installation with `sudo apt remove --purge --auto-remove cmake`.

## References

* [A bot for Starcraft in Rust](https://habr.com/en/post/436254/)
* [Tutorial for creating a StarCraft bot](https://sscaitournament.com/index.php?action=tutorial)
* [SSCAI map pack](https://sscaitournament.com/files/sscai_map_pack.zip)
* [BWAPI](https://github.com/bwapi/bwapi/)
* [BWAPI-C](https://github.com/RnDome/bwapi-c)

