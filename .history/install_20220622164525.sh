apt-get update && apt-get install -y

sudo apt-get update
sudo apt-get upgrade
sudo apt-get install cmake pkg-config libtool libtool-bin unzip



sudo apt-get install 



RUN apt-get update && apt-get install -y \
  bzr \
  cvs \
  git \
  mercurial \
  subversion \
  && rm -rf /var/lib/apt/lists/*




curl -fsSL https://deb.nodesource.com/setup_current.x | sudo -E bash -
sudo apt-get install -y nodejs



https://github.com/junegunn/fzf
git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
~/.fzf/install







git clone https://github.com/neovim/neovim
cd neovim













make CMAKE_BUILD_TYPE=RelWithDebInfo
sudo make install

sudo apt install cmake unzip libtool libtool-bin  gettext
gettext
