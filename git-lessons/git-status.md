# Git Status

`git status` is a Git command that provides information about the current state of your working directory and staging area. It shows you which changes are staged, which are not yet staged, and which files are untracked.

Here's a breakdown of the `git status` command with examples:

1. **Untracked files:**

   When you create new files in your working directory that Git is not currently tracking, they are considered untracked. The `git status` command will show these files in the "Untracked files" section.

   ```bash
   $ git status
   On branch master
   Untracked files:
     (use "git add <file>..." to include in what will be committed)

           newfile.txt

   nothing added to commit but untracked files present (use "git add" to track)
   ```

2. **Changes not staged for commit:**

   If you have modified existing files but haven't added those changes to the staging area, `git status` will display them in the "Changes not staged for commit" section.

   ```bash
   $ git status
   On branch master
   Changes not staged for commit:
     (use "git add <file>..." to update what will be committed)
     (use "git restore <file>..." to discard changes in working directory)

           modified:   myfile.txt

   no changes added to commit (use "git add" and/or "git commit -a")
   ```

3. **Changes to be committed:**

   After you use `git add` to stage changes, the files will appear in the "Changes to be committed" section.

   ```bash
   $ git status
   On branch master
   Changes to be committed:
     (use "git restore --staged <file>..." to unstage)

           modified:   myfile.txt
   ```

4. **Working tree clean:**

   If there are no untracked files or changes not staged for commit, `git status` will indicate that the working directory is clean.

   ```bash
   $ git status
   On branch master
   nothing to commit, working tree clean
   ```

These are the main scenarios you might encounter when using `git status`. It helps you understand the current state of your repository and what actions you need to take next.
