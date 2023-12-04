# Merging Commits

## Method 1 : Reset then Commit

**Problem Statement:**

I have made several commits, and I want to combine them into a single commit. I also want to reset to the state before these commits and then commit all the changes at once.

**Solution:**

1. **Reset to Previous Commit:**

   - Identify the commit hash you want to reset to using `git log`.
   - Run `git reset --mixed <commit-hash>` to move your branch pointer and working directory back to that commit, discarding the commits made afterward.

2. **Stage and Commit All Changes:**
   - Use `git add .` to stage all changes.
   - Commit the changes using `git commit -m "Your combined commit message"`.

**Step-by-Step:**

```bash
# Step 1: Identify the commit hash you want to reset to
git log

# Step 2: Reset to the previous commit
git reset --mixed <commit-hash>

# Step 3: Stage all changes
git add .

# Step 4: Commit the changes with a combined commit message
git commit -m "Your combined commit message"
```

This will result in a single commit that includes all the changes made in the previous commits.

---

## Method 2 : Squashing Commits

Squashing commits using `git rebase -i <hash>` allows you to combine multiple commits into a single, more cohesive commit. This is useful for cleaning up your commit history before pushing changes to a shared repository. Here's a step-by-step explanation with an example:

### Step 1: Identify the Commit Hash

First, find the commit hash to which you want to start the interactive rebase. You can use `git log` to view the commit history and find the hash of the commit just before the series of commits you want to squash.

```bash
git log
```

### Step 2: Start Interactive Rebase

Run the following command, replacing `<hash>` with the commit hash you identified:

```bash
git rebase -i <hash>
```

This opens an interactive rebase session, presenting you with a list of commits and options.

### Step 3: Mark Commits for Squashing

The interactive rebase list will look something like this:

```bash
pick 1a2b3c4 Your commit message 1
pick 5d6e7f8 Your commit message 2
pick 9g0h1i2 Your commit message 3
```

Change the word `pick` to `squash` (or simply `s`) for the commits you want to squash. For example:

```bash
pick 1a2b3c4 Your commit message 1
squash 5d6e7f8 Your commit message 2
squash 9g0h1i2 Your commit message 3
```

### Step 4: Save and Close the Editor

Save the changes and close the editor.

### Step 5: Edit the Combined Commit Message

After saving, another editor will open, allowing you to edit the commit message for the squashed commit. Keep the commit messages you want or modify them as needed. Save and close the editor.

### Step 6: Complete the Rebase

Git will apply the changes and complete the rebase. If there are any conflicts during the rebase, Git will pause and allow you to resolve them.

### Step 7: Verify Changes

After the rebase is complete, use `git log` to verify that the commits have been squashed into a single commit.

```bash
git log
```

### Step 8: Force Push Changes (Optional)

If you've already pushed the commits to a remote repository, you'll need to force push the changes since you've rewritten history.

```bash
git push origin <branch> --force
```

**Important Note:** Be cautious with force pushing, especially if you're working in a shared repository. Rewriting history can cause issues for collaborators.

That's it! You've successfully squashed commits using `git rebase -i <hash>`.
