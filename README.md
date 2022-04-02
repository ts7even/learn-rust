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

## Make Rust Initialized
* make sure your folder is a snake case name (learn_rust)
* cargo init (Creates rust enviorment)
* cargo run (compile and run)
* ./target/debug/learn_rust (runs the compiled file)
* cargo build (compiles but does not run)
* cargo build --release (builds package for release 'optimized')


## Importing functions from other file to call 
make sure you save all before running
* from print.rs file
```

pub fn run() {
    //print to console
    println!("Hello from the print.rs file");

    // To print 'Number 1' you have to do it like this
    println!("Number {}", 1);

}

```
* from main.rs file

```
mod print;
// mod imports the print.rs file

fn main() {
    //calls and runs the print.rs file
    print::run();
}

```

## Primative Types
/* 
Primative Types ---
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory, larger the number the more of bits)
FLoats: f32, f64
Boolean (bool)
Characters (char) - 1 character not a string
Tuples - lists 
Arrays - fixed length
Vectors - growing length
Rust is a Statis type language, 
*/

// Rust is  astatically typed language, which means that it must know the types of all variables at compiple time. 
// Howerverm the compiler can usally infer what type we want to use base on the value and how we use it. 
