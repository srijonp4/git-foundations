# Fetch Upstream

## TLDR:

---

## ALT:

Fetching upstream in Git involves retrieving the latest changes from the original repository (the upstream repository) that your local repository is forked from. This is commonly done when you want to keep your forked repository up-to-date with the changes made in the original repository. Here's a step-by-step guide:

### Steps to Fetch Upstream Changes:

1. **Add Upstream Remote:**

   If you haven't added the upstream remote, you need to do this first. Navigate to your local repository in the terminal and run:

   ```bash
   $ git remote add upstream https://github.com/original/repository.git
   ```

   Replace the URL with the URL of the original repository.

2. **Fetch Upstream Changes:**

   Now, fetch the latest changes from the upstream repository:

   ```bash
   $ git fetch upstream
   ```

   This command fetches the changes from the upstream repository but does not merge them into your local branch.

3. **Merge or Rebase:**

   After fetching upstream changes, you have a couple of options to incorporate those changes into your local branch:

   - **Merge:**

     ```bash
     $ git merge upstream/main
     ```

     Replace `main` with the branch name you want to merge into. This creates a new merge commit.

   - **Rebase:**

     ```bash
     $ git rebase upstream/main
     ```

     This reapplies your local commits on top of the upstream changes. It provides a cleaner history but can lead to conflicts that need resolution.

4. **Push Changes to Your Fork:**

   After merging or rebasing, push the changes to your forked repository:

   ```bash
   $ git push origin main
   ```

   Replace `main` with your branch name.

### Why Fetch Upstream:

1. **Stay Updated:**

   Fetching upstream changes helps you keep your local repository up-to-date with the latest changes made by others in the original repository.

2. **Contribute to Original Repository:**

   If you plan to contribute back to the original repository, having your fork updated makes it easier to create meaningful and conflict-free pull requests.

3. **Resolve Conflicts Locally:**

   By fetching changes regularly, you can identify and resolve conflicts locally, making it easier to manage the integration of new features and bug fixes.

Remember to adjust branch names and URLs according to your specific repository and branch configurations. Regularly fetching upstream changes is a good practice, especially when working on long-term projects or contributing to open-source repositories.
