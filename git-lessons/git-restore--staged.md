# Git restore staged

The `git restore --staged` command is used to unstage changes that have been previously added to the staging area. This can be useful if you have added changes to the staging area using `git add` but later decide that you don't want to include those changes in the next commit.

Here's how you can use `git restore --staged` with examples:

### Example 1: Unstaging Changes

Suppose you have modified a file and added it to the staging area, but you want to unstage it:

```bash
# Make some changes to a file
$ echo "Some changes" > myfile.txt

# Add the changes to the staging area
$ git add myfile.txt

# Check the status
$ git status
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)

        modified:   myfile.txt

# Unstage the changes
$ git restore --staged myfile.txt

# Check the status again
$ git status
On branch master
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)

        modified:   myfile.txt
```

### Example 2: Unstaging All Changes

You can also use `git restore --staged .` to unstage all changes:

```bash
# Add all changes to the staging area
$ git add .

# Check the status
$ git status
On branch master
Changes to be committed:
  (use "git restore --staged <file>..." to unstage)

        modified:   file1.txt
        modified:   file2.txt

# Unstage all changes
$ git restore --staged .

# Check the status again
$ git status
On branch master
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)

        modified:   file1.txt
        modified:   file2.txt
```

In both examples, the `git restore --staged` command is used to unstage changes, making them again unstaged modifications in your working directory. Remember to replace `myfile.txt`, `file1.txt`, and `file2.txt` with the actual names of your files.
