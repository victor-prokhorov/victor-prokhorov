TODO: split into separate file by tech, git, linux, ts, rs, etc.
TMUX
C b - tab, then C+0 or C+1 to switch
C+l - toggle between windows
C % - split v
C+o - toggle between splits
simply C+d to exit
git add -A - add all file to the repo not only current folder as git add . do
DEBUG_PRINT_LIMIT=10000 yarn test
LINUX
du -cksh * | sort -rn | head
find . -name 'node_modules' -type d -exec rm -r '{}' \;
find . -iname '*pattern*' - case insensitive search file containing pattern
grep -Ril 'pattern' . - search pattern from current folder
(need exclude for node_modules)
grep -Rl 'console.log' src/ | xargs -n1 -I {} sh -c "sed -i '/console.log/d' {}" -- remove console logs
kill $(lsof -ti:6017) -- kill process
VIM
:g/^\s*$/d - Remove all blank lines
:ls - list buffers
:b0 - checkout buf 0
:Buffers via fzf give nice preview and can be C+p / C+n
:bd[elete] - close buffer
C+6 - toggle between 2 last buffers
:bp[revious]
:bn[ext]
S' - surround with ' in visual mode
cst<tag - change tag to tag
cstt - change surrounding tag by tag
C + z - run in background or :sus[pend]
fg - run in foreground
jobs - list background jobs
C + w, v
C + w, w
:split :sp <file> :vsp <file>
:on[ly]
:tabnew
:tabonly :tabo[nly]
gt gT (tab)
:wqa
:e <file>
:%s/<text>/<text>/gc
m<letter> - save mark
'<letter> - move to mark
"1p - paste last deletion
"<letter><action>
"a3dd - delete 3 lines to named buffer a
"ap - paste from buffer a
GITLAB
on merge request remove branch and squash commits
YARN
yarn build
yarn link
PACKAGES
npx npm-check-updates -u @your_org/*
GIT
git checkout --track origin/<branch> - checkout new branch from remote
git reset --hard main
git log --oneline remote1/branch..remote2/branch
git add --patch - NEED git-perl git-email
git add -p

git checkout --ours -- <file>
git checkout --theirs -- <file>
git branch --contains <hash>
diff --brief --recursive <dir> <dir>
cp -TRv <div> <div>
git push -d <remove=origin> <branch>
git branch -d <branch>
git revert <commit-hash-to-revert>
git revert -m 1 <merge-commit-hash>
git commit --no-verify
git push --no-verify
git log <branch> -n <number>
git merge feature main - checkout feature and merge main oneliner
git checkout feature git rebase main - linear history but rewrite history
git push --force origin main
git fetch <remote>
git branch -v -a
git switch -c <branch> <remote>/<branch>
git switch -c <non_existing_branch>
is the same as but avoid unclear 'checkout' that used for many things
git checkout -b <non_existing_branch>
stack:
https://www.conventionalcommits.org/en/v1.0.0/
https://commitizen-tools.github.io/commitizen/
https://www.npmjs.com/package/lint-staged
https://www.npmjs.com/package/husky
https://jestjs.io/
https://testing-library.com/
https://www.cypress.io/
https://www.elastic.co/observability/application-performance-monitoring
https://12factor.net/config
https://www.drycode.io/
https://kentcdodds.com/blog/common-mistakes-with-react-testing-library
