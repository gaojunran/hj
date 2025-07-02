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
- [x] Clone a repo
  - [x] shorthand for github
  - [ ] shorthand for your own repo (powered by gh cli)
- [x] Download a repo (powered by [degit-rs](https://github.com/psnszsn/degit-rs))
- [x] Initialize a repo
  - [x] Create a new repo on GitHub
  - [x] Add remote to local repo
  - [x] Create a default branch
  - [ ] Download a .gitignore
- [x] Commit Specified Files
  - [ ] Commit and Push
- [x] Amend Commit
- [x] Reset Commit
- [x] Push to Remote
- [x] Pull from Remote


