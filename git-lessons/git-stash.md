# git stash

`git stash` is a Git command that allows you to temporarily save changes that are not ready to be committed yet. This is useful when you need to switch to a different branch or work on something else without committing your current changes. The stashed changes can later be reapplied to your working directory.

Here are some examples of using `git stash`:

### Example 1: Stash Changes

```bash
# Save your changes to a stash
$ git stash
```

This command will stash your changes and revert your working directory to the state of the last commit.

### Example 2: Stash with a Message

```bash
# Stash changes with a descriptive message
$ git stash save "WIP: Working on a new feature"
```

This is helpful for adding context to your stashes.

### Example 3: Stash Untracked Files

```bash
# Stash changes, including untracked files
$ git stash -u
```

The `-u` or `--include-untracked` option stashes untracked files as well.

### Example 4: Stash Multiple Changes

```bash
# Stash changes with multiple files
$ git stash push -m "Working on multiple features" file1.txt file2.txt
```

You can specify the files you want to stash.

### Example 5: List Stashes

```bash
# List all stashes
$ git stash list
```

This will show a list of all your stashes.

### Example 6: Apply Stash

```bash
# Apply the latest stash
$ git stash apply
```

This will apply the changes from the latest stash to your working directory. If you have multiple stashes, you can specify which one to apply using `git stash apply stash@{n}`.

### Example 7: Apply and Drop Stash

```bash
# Apply the latest stash and remove it from the stash list
$ git stash pop
```

This is equivalent to applying the stash and then dropping it.

### Example 8: Drop Stash

```bash
# Remove the latest stash without applying it
$ git stash drop
```

### Example 9: Clear All Stashes

```bash
# Remove all stashes
$ git stash clear
```

This will remove all stashes from the stash list.

These examples demonstrate various use cases for `git stash`. It's a powerful tool for managing changes when you need to switch contexts or branches temporarily.
