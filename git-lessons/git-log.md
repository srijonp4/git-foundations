# Git Log

The `git log` command is used to display the commit history of a Git repository. By default, it shows a list of commits in reverse chronological order, starting with the most recent commit. Here's how you can use `git log` with some examples:

1. **Basic `git log` usage:**

   ```bash
   $ git log
   ```

   This command displays the commit history with details like the commit hash, author, date, and commit message.

2. **Limiting the number of commits displayed:**

   You can limit the number of commits shown in the log using the `-n` option:

   ```bash
   $ git log -n 5
   ```

   This shows only the latest 5 commits.

3. **Displaying a concise log:**

   To display a more condensed version of the log, you can use the `--oneline` option:

   ```bash
   $ git log --oneline
   ```

   This shows each commit on a single line, with only the first few characters of the commit hash and the commit message.

4. **Showing a graphical representation of the commit history:**

   Use the `--graph` option to display a text-based graph of the commit history:

   ```bash
   $ git log --graph
   ```

   This can be useful to visualize branching and merging in your repository.

5. **Viewing commit details:**

   To see the detailed changes introduced in each commit, you can use the `-p` (or `--patch`) option:

   ```bash
   $ git log -p
   ```

   This shows the full diff of each commit.

6. **Displaying commits for a specific branch:**

   You can view the commit history for a specific branch by specifying the branch name:

   ```bash
   $ git log master
   ```

   Replace `master` with the name of the branch you are interested in.

7. **Filtering commits by author:**

   To filter commits by a specific author, use the `--author` option:

   ```bash
   $ git log --author="John Doe"
   ```

   Replace "John Doe" with the actual author's name.

These are just a few examples of how you can use `git log` to explore the commit history of a Git repository. The command has various other options and flags that you can use to customize the output based on your needs.
