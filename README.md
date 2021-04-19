# rsmooth

Wrapper around [pandoc](https://pandoc.org) to create PDF's using [LaTeX](https://www.latex-project.org/). The main idea of rsmooth is to define all the needed informations in the [Front Matter](https://jekyllrb.com/docs/front-matter/) (a [YAML](https://en.wikipedia.org/wiki/YAML) header within the markdown document) thus no external configuration is needed.

To allow for even more flexibility it's possible t


## Usage

Take this simple file as a example (`example.md`):

```md

---
title: A Sample Document
author: John Doe
template: ~/templates/document.tex
---

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
```

To create a PDF based on this file call:

```shellscript
rsmooth example.md
```

This will create `example.pdf`. If you want to specify another name of the PDF use the `-o` flag. The look of the resulting document is defined by the [pandoc template](https://pandoc.org/MANUAL.html#templates) specified by the `template` field in the YAML head. The path to the template file can either be relative or absolute; rsmooth also supports environment variables (like `$HOME`) and tildes as an abbreviation for the home path ([shell expansion](https://tldp.org/LDP/Bash-Beginners-Guide/html/sect_03_04.html)).


## Available Options

By using the YAML header (front matter) you can alter the behavior of rsmooth and use additional features.

Note: The whole content of the 


### A word on paths

Rsmooth tries to resolve a variety of paths used in the configuration header. As they will get [shell expanded](https://tldp.org/LDP/Bash-Beginners-Guide/html/sect_03_04.html) you can use environment variables and tildes (`~`) for your home directory. Relative paths will be handled **relative to the input file's location**.


### Options

`template` Path to the template file. Learn more about these files in the [pandoc documentation](https://pandoc.org/MANUAL.html#templates).

`engine` Name of the LaTeX engine used to create the PDF document. This internally will set the [--pdf-engine](https://pandoc.org/MANUAL.html#option--pdf-engine) option of pandoc. This option defaults to `xelatex` (as this is what I'm working with).

`pandoc-options` Feed [additional options](https://pandoc.org/MANUAL.html#options) into the pandoc call. Defaults to empty.
