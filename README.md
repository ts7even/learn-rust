# Learn Rust

# Git Good 

cd to where you want your project to be 

* git clone https://github.com/ts7even/eda-project2.git

## Git Push & Pull 
* git init
* git add . 
* git status (What branch you are on)
* git commit -m "First Commit"
* git push -u origin master 
Make sure you git fetch then git pull before you start coding on other devices. 
Also Create a developer branch, and then a branch with your name. Total 3 Branches, or one branch per person? 
look up tickets. 

## Create Git Branches for Team Members
* git status 'shows you what branch you are on'
* git branch (branch-name) (and or specific revision)
* git checkout (branch-name) - switches to diffrent branch
* git switch (branch-name - prefered way to switch to diffrent branch)
* git push -u origin (branch-name)

## Git Merge Merge Branches into master branch. Git rebase rewinds the latest merge
* git master (go into the master branch)
* git merge --squash feature (summarize all commits into one commit as latest commit into master branch)
* git commit -m "Branch merged into Master"
* git push 