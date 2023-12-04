# Git Merge

The `git merge` command in Git is used to integrate changes from one branch into another. It combines the changes made on different branches. Here are some examples to illustrate the use of `git merge`:

### Example 1: Basic Merge

Suppose you have a branch called `feature-branch` with some changes that you want to merge into the `master` branch:

1. Switch to the target branch (e.g., `master`):

   ```bash
   $ git checkout master
   ```

2. Merge changes from the source branch (e.g., `feature-branch`):

   ```bash
   $ git merge feature-branch
   ```

   This command merges the changes from `feature-branch` into the current branch (`master`).

### Example 2: Fast-Forward Merge

If there are no new changes in the target branch since the source branch was created, Git performs a fast-forward merge. This means that the target branch pointer simply moves forward to the same commit as the source branch:

1. Switch to the target branch:

   ```bash
   $ git checkout master
   ```

2. Merge changes from the source branch:

   ```bash
   $ git merge feature-branch
   ```

   If there are no conflicting changes, Git performs a fast-forward merge.

### Example 3: Merge Conflict

If changes in the source and target branches overlap and Git cannot automatically resolve the differences, a merge conflict occurs. Here's an example:

1. Switch to the target branch:

   ```bash
   $ git checkout master
   ```

2. Make changes in the target branch:

   ```bash
   # Modify the same file that was modified in the source branch
   $ echo "This is a conflicting change." >> myfile.txt
   $ git add myfile.txt
   $ git commit -m "Conflicting change in master"
   ```

3. Attempt to merge the source branch:

   ```bash
   $ git merge feature-branch
   ```

   Git will indicate a conflict, and you must manually resolve it before completing the merge.

### Example 4: Merge from Remote Repository

If you're working with a remote repository, you might need to fetch changes from the remote before merging:

1. Fetch changes from the remote:

   ```bash
   $ git fetch origin
   ```

2. Merge changes from a remote branch:

   ```bash
   $ git merge origin/feature-branch
   ```

   This example merges changes from a remote branch (`origin/feature-branch`) into the current branch.

These examples cover basic usage scenarios of the `git merge` command. It's important to handle conflicts carefully and review changes before completing a merge, especially in collaborative projects.
