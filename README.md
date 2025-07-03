# hj (Hyper Jujutsu)

> Working in Progress. 🚧

Fast, opinionated version control experience. Powered by [jj-vcs](https://github.com/jj-vcs/jj).

## Goals

- Most of jj actions can be done with a single command, with reasonable default arguments.
- Try my best to make jj compatible with git remote actions (pull, push, ...). Some bookmark actions are annoying, I make them implicit.
- Integrate GitHub workflow into jj, (powered by gh cli).
- Bring designable [starship](https://starship.rs/) prompt to jj.
- Final goal: make a freshman user able to use jj **without any git knowledge**.

MAKE **VERSION CONTROL** GREAT AGAIN!

## Planned Features
- [x] Clone a repo
  - [x] shorthand for github
  - [ ] shorthand for your own github repo (powered by gh cli)
- [x] Download a repo (powered by [degit-rs](https://github.com/psnszsn/degit-rs))
- [x] Initialize a repo
  - [x] Create a new repo on GitHub
  - [x] Add remote to local repo
  - [x] Create a default branch
  - [ ] Download a .gitignore
- [x] Commit specified files
  - [x] `--push` option
- [x] Amend changes
- [x] Reset changes
- [x] Push to remote
  - [x] If no branch specified, push the closest bookmark
  - [x] (for a multi-worker branch) Pull before Push `--pull`
  - [x] (for a multi-branch repo) Update trunk `--upbase`
  - [ ] `--change` and `--pr` option
- [x] Pull from Remote
- [x] Update Trunk from a branch (`upbase`)
- [ ] Starship Prompt
- [ ] Github PR Create (other PR actions via gh cli)

## Nice ideas that may be added in `hj`

- [Find the closest bookmark in history to advance it forward](https://github.com/jj-vcs/jj/discussions/5568)
- [Select all my branches with the goal of rebasing them all on the trunk](https://github.com/jj-vcs/jj/discussions/4974)
- [`gh pr create`](https://github.com/jj-vcs/jj/discussions/6279)
- [jj configurations](https://github.com/jj-vcs/jj/discussions/5812)

