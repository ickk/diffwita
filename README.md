This is a proof of concept.

Diffwita
---

A (WIP) tool to turn a git tree into a tutorial. It's like a simple form of
[*literate programming*](https://en.wikipedia.org/wiki/Literate_programming).

The way it works is you simply include markdown in your commit messages that
describes the changes in that particular commit. Diffwita then turns it into a
lovely document.

![example image](image.png?raw=true)

***Why?***
- It's kind of silly.
- Lets tutorials include a single source of truth for both the text of a
  tutorial and the project the tutorial is building.
- The audience can be sure that if they follow your tutorial, it will build
  because you can't make a mistake or leave something out when the diff itself
  constitutes the tutorial.
- Readers can browse the change-log and directly relate the change with the
  relevant section of the tutorial since the commit message includes the
  tutorial.

**To do:**
- automatically build a table of contents, linkify headings, separate into
  multiple pages.
- create a markdown block that lets you specify where a particular file will
  appear in the markdown.
- use libgit2 instead of calling out to the system's git install.
- probably should rewrite a bunch of it to take advantage of the templating
  library I used instead of messing around so much with the markdown
  processor.
- make links to commit SHAs actually resolve properly.
- syntax highlighting.

**Notes:**
- You almost certainly want to configure the repository with:
  ```sh
  git config --local commit.cleanup scissors
  ```
  which prevents git from removing your headings; `#` is a comment to git by
  default, and gets removed from your commit messages.
- If you want to create a section in your document with only text but no code
  changes, the command you need is:
  ```sh
  git commit --allow-empty
  ```
