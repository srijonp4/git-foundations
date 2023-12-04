# Git Add

In Git, the `git add` command is used to stage changes for the next commit. It tells Git that you want to include updates to a particular file in the next commit. Here are some examples of using `git add`:

1. **Add a Specific File:**

   To stage changes for a specific file, use the following command:

   ```bash
   git add filename.txt
   ```

   Replace `filename.txt` with the actual name of the file you want to stage.

2. **Add All Changes:**

   To stage all changes (both modified and untracked files), you can use the following command:

   ```bash
   git add .
   ```

   This will add all changes in the current directory and its subdirectories.

3. **Add Changes Interactively:**

   Git provides an interactive mode for adding changes, which allows you to selectively stage parts of a file. Use the following command:

   ```bash
   git add -p
   ```

   This will prompt you with each change, and you can choose whether to stage or skip it.

4. **Add Changes by File Type:**

   If you want to add changes only to files of a specific type, you can use wildcards. For example, to add all `.txt` files:

   ```bash
   git add *.txt
   ```

   This adds all text files in the current directory.

5. **Add Changes in a Specific Directory:**

   To stage changes only in a specific directory, you can provide the directory path:

   ```bash
   git add path/to/your/directory/
   ```

6. **Add All Changes, Including Untracked Files:**

   If you want to add all changes, including untracked files, you can use the following command:

   ```bash
   git add -A
   ```

   This stages all changes, including modifications, deletions, and untracked files.

7. **Add Changes and Mark Conflicts as Resolved:**

   When resolving merge conflicts, after manually resolving conflicts in the files, you can use the following command to mark them as resolved:

   ```bash
   git add resolved-file.txt
   ```

   Replace `resolved-file.txt` with the file you have resolved.

Remember to commit your changes after using `git add` to stage them. You can use `git commit` to commit the staged changes to your local repository.
