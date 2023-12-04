# git reset

`git reset` is a Git command that is used to reset the current HEAD to a specified state. It can be used to unstage changes, undo commits, or move the HEAD to a different commit. There are different modes of `git reset` that allow you to accomplish different tasks.

Here are some examples of how to use `git reset`:

### 1. Unstage changes:

If you have added changes to the staging area using `git add` but want to unstage them, you can use `git reset` with the `--soft` option:

```bash
# Assume you've added changes to the staging area
$ git add file1.txt file2.txt

# Unstage the changes
$ git reset --soft
```

This will unstage the changes but leave the modified files in your working directory.

### 2. Discard changes in the working directory:

If you want to discard changes both in the staging area and the working directory, you can use `git reset` with the `--hard` option:

```bash
# Assume you've made changes to the working directory
$ git add file1.txt
$ git reset --hard
```

This will unstage changes and discard changes in the working directory.

### 3. Move HEAD to a previous commit:

To move the HEAD to a previous commit, effectively undoing commits, you can use `git reset` with a specific commit hash:

```bash
# Assume you want to go back to the commit with hash abc123
$ git reset --hard abc123
```

Be cautious when using `--hard` as it discards changes irreversibly.

### 4. Soft reset for undoing commits:

If you want to undo the last few commits but keep the changes staged, you can use `git reset` with the `--soft` option:

```bash
# Assume you want to undo the last 2 commits
$ git reset --soft HEAD~2
```

This will move the HEAD pointer to the specified commit, and the changes from the undone commits will be in the staging area.

### 5. Mixed reset for undoing commits:

If you want to undo the last few commits and unstage the changes, you can use `git reset` with the `--mixed` option (which is the default behavior if no option is specified):

```bash
# Assume you want to undo the last 2 commits
$ git reset HEAD~2
```

This will move the HEAD pointer to the specified commit and unstage the changes, but keep the changes in the working directory.

Always use caution when using `git reset` with the `--hard` option, as it can lead to irreversible data loss. It's a good practice to create a backup or commit your changes before performing a hard reset if you are unsure.
