# Pull Request in GIT

A "pull request" is a feature in version control systems, such as Git and GitHub, that allows you to propose changes from a branch in one repository to the branch of another repository. In the context of Git and GitHub, the term "pull request" is commonly used, while in other systems, it might be referred to as a "merge request" or something similar.

Here is a detailed explanation of how to create a pull request and why it's essential to use different branches for each pull request:

### How to Create a Pull Request:

1. **Fork the Repository:**

   If you don't have write access to the original repository, you need to fork it on platforms like GitHub. Forking creates a copy of the repository under your GitHub account.

2. **Clone the Forked Repository:**

   Clone the forked repository to your local machine:

   ```bash
   $ git clone https://github.com/yourusername/repository.git
   ```

3. **Create a New Branch:**

   Create a new branch for your feature or bug fix:

   ```bash
   $ git checkout -b feature-branch
   ```

4. **Make Changes:**

   Make your changes and commit them to the new branch:

   ```bash
   $ git add .
   $ git commit -m "Your commit message"
   ```

5. **Push Changes to Your Fork:**

   Push the changes to your forked repository on GitHub:

   ```bash
   $ git push origin feature-branch
   ```

6. **Create Pull Request:**

   Open your forked repository on GitHub and navigate to the branch you just pushed. Click on the "Compare & pull request" button.

7. **Fill in Pull Request Details:**

   Provide a title and description for your pull request. Review the changes and make sure everything looks good.

8. **Create Pull Request:**

   Click the "Create pull request" button to submit your changes. This notifies the maintainers of the original repository that you have changes you'd like them to review.

### Why Use Different Branches for Each Pull Request:

1. **Isolation of Changes:**

   Creating a new branch for each feature or bug fix isolates your changes from the main development branch. This isolation makes it easier to manage and review changes independently.

2. **Parallel Development:**

   Different developers can work on different features concurrently by creating separate branches. This allows for parallel development without conflicts.

3. **Ease of Code Review:**

   Pull requests are often associated with code reviews. Having changes in separate branches allows for focused and targeted code reviews, making it easier to understand and approve/reject changes.

4. **Maintainability:**

   Each branch represents a specific set of changes with a clear purpose. This makes it easier to maintain a clean and organized project history.

By following these practices, you contribute to a collaborative and organized development workflow, especially when working with others on a shared codebase. It facilitates communication and ensures that changes are well-tested and reviewed before they are merged into the main branch.
