# git branch

The `git branch` command is used to list, create, or delete branches in a Git repository. Here are some examples of using `git branch` with detailed explanations:

### 1. **List Branches:**

To list all branches in your repository:

```bash
$ git branch
* master
  feature-branch
  another-feature
```

This example shows three branches: `master`, `feature-branch`, and `another-feature`. The current branch is indicated by the `*` symbol.

### 2. **Create a New Branch:**

To create a new branch:

```bash
$ git branch new-feature
```

This creates a new branch named `new-feature`. However, it does not switch to that branch. You would typically use `git checkout` or `git switch` to switch to the new branch.

### 3. **Switch to a Branch:**

To switch to an existing branch:

```bash
$ git checkout feature-branch
```

Or, using the newer `git switch` command:

```bash
$ git switch feature-branch
```

This switches your working directory to the specified branch.

### 4. **Create and Switch to a New Branch (Shortcut):**

To create and switch to a new branch in one command:

```bash
$ git checkout -b new-feature
```

Or, using the newer `git switch`:

```bash
$ git switch -c new-feature
```

This is a shortcut that combines branch creation and switching.

### 5. **Delete a Branch:**

To delete a branch:

```bash
$ git branch -d another-feature
```

This deletes the branch `another-feature`. The `-d` flag stands for delete.

### 6. **Force Delete a Branch:**

If a branch has unmerged changes, you may need to force delete it:

```bash
$ git branch -D another-feature
```

The `-D` flag stands for force delete.

### 7. **List Remote Branches:**

To list remote branches:

```bash
$ git branch -r
  origin/master
  origin/feature-branch
```

This shows branches on the remote repository (e.g., `origin`).

### 8. **List Local and Remote Branches:**

To list both local and remote branches:

```bash
$ git branch -a
* master
  feature-branch
  another-feature
  remotes/origin/master
  remotes/origin/feature-branch
```

This displays both local and remote branches.

These are some common use cases of the `git branch` command. It's a fundamental tool for managing branches in a Git repository, which is crucial for organizing and collaborating on software development projects. Adjust the examples based on your specific needs and workflow.
