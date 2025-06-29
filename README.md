# hj (Hyper Jujutsu)

> Working in Progress. 🚧

Fast, opinionated version control experience. Powered by [jj-vcs](https://github.com/jj-vcs/jj).

## Goals

- Most of actions can be done with a single command, with reasonable default arguments.
- Try my best to make jj compatible with git remote actions (pull, push, ...). Some bookmark actions are trivial, I make them implicit.
- Integrate GitHub workflow into jj, (powered by gh cli).
- Bring designable [starship](https://starship.rs/) prompt to jj.
- Final goal: make a freshman user able to use jj **without any git knowledge**.

MAKE **VERSION CONTROL** GREAT AGAIN!

## Planned Features
- [ ] Clone a repo
  - [ ] shorthand for github
  - [ ] shorthand for your own repo (powered by gh cli)
- [ ] Initialize a repo
  - [ ] Create a new repo on GitHub
  - [ ] Add remote to local repo
  - [x] Create a default branch
  - [ ] Download a .gitignore
- [x] Commit Specified Files
- [x] Amend Commit Specified Files
- [x] Reset Last Commit
- [ ] Push to Remote
- [ ] Pull from Remote
