# rsmooth

Wrapper around [pandoc](https://pandoc.org) to create PDF's using [LaTeX](https://www.latex-project.org/). The main idea of rsmooth is to define all the needed informations in the [Front Matter](https://jekyllrb.com/docs/front-matter/) (a [YAML](https://en.wikipedia.org/wiki/YAML) header within the markdown document) thus no external configuration is needed.


## Usage

Take this simple file as a example (`example.md`):

```markdown

---
title: A Sample Document
author: John Doe
template: ~/templates/document.tex
---

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
```

To create a PDF based on this file call:

```shellscript
rsmooth example.md
```

This will create `example.pdf`. If you want to specify another name of the PDF use the `-o` flag. The look of the resulting document is defined by the [pandoc template](https://pandoc.org/MANUAL.html#templates) specified by the `template` field in the YAML head. The path to the template file can either be relative or absolute; rsmooth also supports environment variables (like `$HOME`) and tildes as an abbreviation for the home path ([shell expansion](https://tldp.org/LDP/Bash-Beginners-Guide/html/sect_03_04.html)).
