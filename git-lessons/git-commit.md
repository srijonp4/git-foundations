# Git Commit

The `git commit` command is used to save your changes to the local Git repository. It is a crucial step in the Git workflow. Here's how you can use `git commit` with examples:

### Basic Commit

To commit changes, you typically follow these steps:

1. **Stage Changes:**
   Use `git add` to stage the changes you want to commit. This tells Git which modifications you want to include in the commit.

   ```bash
   $ git add file1.txt file2.txt
   ```

2. **Commit Changes:**
   After staging changes, use `git commit` to create a new commit. This opens a text editor for you to write a commit message.

   ```bash
   $ git commit
   ```

   The text editor will open, allowing you to enter a commit message. Save and close the editor to complete the commit.

### Commit with Message Inline

You can also provide the commit message inline using the `-m` option:

```bash
$ git commit -m "Your commit message here"
```

### Amending the Last Commit

If you forgot to include something in your last commit or need to change the commit message, you can use the `--amend` option:

```bash
$ git commit --amend
```

This allows you to modify the last commit. The text editor will open for you to make changes.

### Committing All Changes at Once

If you want to commit all changes (both staged and unstaged), you can use the `-a` option:

```bash
$ git commit -a -m "Committing all changes"
```

This automatically stages and commits all modifications to tracked files.

### Committing with Author Information

You can specify the author information for a commit using the `--author` option:

```bash
$ git commit --author="Author Name <author@example.com>"
```

### Examples Summary:

1. **Basic Commit:**

   ```bash
   $ git add file1.txt file2.txt
   $ git commit
   ```

2. **Commit with Message Inline:**

   ```bash
   $ git commit -m "Your commit message here"
   ```

3. **Amend the Last Commit:**

   ```bash
   $ git commit --amend
   ```

4. **Committing All Changes at Once:**

   ```bash
   $ git commit -a -m "Committing all changes"
   ```

5. **Committing with Author Information:**
   ```bash
   $ git commit --author="Author Name <author@example.com>"
   ```

Remember to replace file names, commit messages, and author information with your specific details. The commit message is essential for explaining the purpose of the commit.
