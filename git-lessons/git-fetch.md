# Git Fetch

`git fetch` is a Git command that retrieves updates from a remote repository but does not integrate the changes into your local working copy. It's a way to see what changes are available in the remote repository without making any changes to your local branches. Here are some examples of using `git fetch`:

1. **Fetch Changes from the Default Remote (usually "origin"):**

   ```bash
   $ git fetch
   ```

   This command fetches changes from the default remote repository (`origin` in most cases). After running this command, your local repository will be aware of any changes made in the remote repository.

2. **Fetch Changes from a Specific Remote:**

   ```bash
   $ git fetch upstream
   ```

   This fetches changes from a remote named "upstream." You can replace "upstream" with the name of any remote you want to fetch changes from.

3. **Fetch a Specific Branch from a Remote:**

   ```bash
   $ git fetch origin feature-branch
   ```

   This fetches changes specifically for the "feature-branch" from the remote named "origin."

4. **Fetch All Branches from a Remote:**

   ```bash
   $ git fetch --all
   ```

   This fetches changes for all branches from the default remote repository. It ensures that your local repository is up-to-date with all branches in the remote repository.

5. **Fetch and Prune Remote Branches:**

   ```bash
   $ git fetch --prune
   ```

   This fetches changes and removes any remote-tracking branches that no longer exist on the remote. It helps keep your local repository in sync with the remote repository.

6. **Fetch Changes and Display Progress:**

   ```bash
   $ git fetch --progress
   ```

   This fetches changes and displays progress information. It's useful when you want to see the progress of the fetch operation.

7. **Fetch Changes from All Remotes:**

   ```bash
   $ git fetch --all
   ```

   This fetches changes from all remotes associated with your local repository.

After running `git fetch`, you can inspect the changes using other commands like `git log`, `git diff`, or `git status`. If you want to integrate the changes into your local branches, you can use `git merge` or `git rebase` after fetching.
