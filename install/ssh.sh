git config --global user.name "Victor Prokhorov"
git config --global user.email "mail@victorprokhorov.com"
ssh-keygen -t ed25519 -C "mail@victorprokhorov.com"
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519
cat ~/.ssh/id_ed25519.pub
