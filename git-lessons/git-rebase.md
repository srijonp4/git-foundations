# git rebase

Video Explanations 🔽

- https://www.youtube.com/watch?v=CRlGDDprdOQ
- https://www.youtube.com/watch?v=f1wnYdLEpgI

---

`git rebase` is a Git command used to integrate changes from one branch into another by moving or combining a sequence of commits to a new base commit. This process results in a cleaner, more linear commit history. It's important to note that rebasing rewrites commit history, so it should be used carefully, especially on branches that are shared with others.

Here's an explanation of `git rebase` with an example:

### Example Scenario:

1. **Create a Feature Branch:**

   Assume you have a feature branch named `feature-branch` that you want to rebase onto the latest changes in the `main` branch.

   ```bash
   # Create and switch to the feature branch
   git checkout -b feature-branch
   ```

   Now, you make some commits on the `feature-branch`.

   ```
   A---B---C  (main)
            \
             D---E---F  (feature-branch)
   ```

2. **Switch to the Main Branch:**

   ```bash
   git checkout main
   ```

3. **Fetch the Latest Changes from Remote:**

   Ensure your local `main` branch is up-to-date with the remote repository.

   ```bash
   git fetch origin main
   ```

4. **Rebase the Feature Branch onto Main:**

   ```bash
   git checkout feature-branch
   git rebase main
   ```

   This command moves the changes made in the `feature-branch` onto the tip of the `main` branch.

   ```
   A---B---C---D'---E'---F'  (main)
                  |
                  (feature-branch)
   ```

   The commits `D`, `E`, and `F` from the `feature-branch` are now applied on top of the latest commit in the `main` branch (`C`). The original commits `D`, `E`, and `F` are replaced by new commits `D'`, `E'`, and `F'`.

5. Now checkout to the main branch

   ```bash
   $ git checkout main
   ```

6. Uptodate the main branch with upstream remote

   ```bash
   $ git pull upstream main
   ```

7. rebase the changes in main

   ```sh
   $ git rebase feature_branch
   ```

### Additional Notes:

- If conflicts occur during the rebase process, Git will pause and ask you to resolve them before continuing.

- After rebasing, your branch is effectively a new branch with new commit hashes. Never rebase commits that you have already pushed to a shared repository.

- If you have already shared the branch with others, it's generally safer to use `git merge` instead of `git rebase` to avoid disrupting their work.

Remember to use `git rebase` judiciously, especially in collaborative environments, to avoid confusion and conflicts for other team members.
