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

---

### 6. mixed reset using commit hash

The `git reset` command is used to reset the current state of the repository to a specified commit. When you provide a commit hash, it moves the branch pointer and potentially modifies the staging area and working directory. In your example:

```bash
$ git reset 1f10c11766a18f50c38568f59342975bde965cf0
```

This command resets the branch (assuming you are on a branch) to the specified commit `1f10c11766a18f50c38568f59342975bde965cf0`.

Here are some possible scenarios based on the different modes of `git reset`:

### 1. Soft reset:

```bash
$ git reset --soft 1f10c11
```

This will move the branch pointer to commit `1f10c11`, keeping all changes staged. You can then make further modifications or commit the changes again.

### 2. Mixed reset (default behavior):

```bash
$ git reset 1f10c11
```

This is similar to the soft reset, but it also unstages the changes. The changes will be present in the working directory, and you can choose to re-stage and commit them.

### 3. Hard reset:

```bash
$ git reset --hard 1f10c11
```

This will move the branch pointer to commit `1f10c11` and discard any changes in the staging area and working directory. Be cautious with this option, as it can lead to irreversible data loss.

### 4. Reset to a specific commit:

```bash
$ git reset 1f10c11
```

This is a mixed reset by default. It moves the branch pointer to commit `1f10c11`, unstages changes, and leaves the modified files in the working directory.

Before performing a hard reset, especially with the `--hard` option, make sure you have a backup or are certain that you want to discard all changes after the specified commit.
