# Update the local branch which is behind some commits

**Problem Statement:**

My `main` branch is two commits ahead, and my `dev` branch is behind. How do I update my `dev` branch to be up-to-date with the changes in `main`?

**Solution:**

**Q: How do I start the process?**
A: First, checkout the `dev` branch.

```bash
git checkout dev
```

**Q: What's the next step for merging?**
A: To merge the changes from `main` into `dev`, use the following command:

```bash
git merge main
```

**Q: How about if I want to rebase instead of merging?**
A: If you prefer rebasing, after checking out `dev`, run:

```bash
git rebase main
```

**Q: What does rebasing do?**
A: Rebasing moves your `dev` branch to the tip of the `main` branch, applying your local changes on top.

**Q: What should I do if conflicts occur during rebase?**
A: Git will pause and allow you to resolve conflicts before continuing the rebase.

**Q: Is there anything else I need to remember?**
A: Before merging or rebasing, make sure to commit or stash any changes on your `dev` branch to avoid conflicts.
