# rsmooth

Wrapper around [pandoc](https://pandoc.org) to create PDF's using [LaTeX](https://www.latex-project.org/). The main idea of rsmooth is to define all the needed informations in the [Front Matter](https://jekyllrb.com/docs/front-matter/) (a [YAML](https://en.wikipedia.org/wiki/YAML) header within the markdown document) thus no external configuration is needed.

To allow for even more flexibility it's possible to run the content of your input file trough [Terra](https://tera.netlify.app/) (a templating language very similar to [Jinja2](https://jinja.palletsprojects.com/en/2.11.x/)). This allows you to tweak the content of your pandoc input.


## Usage

### PDF Export

Take this simple file as a example (`example.md`):

```markdown

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


### Example File

The application can create an example markdown file showcasing some of the functionality of rsmooth.

```shell script
rsmooth example-file -o example.md
```

Saves the content to the `example.md` file. Calling without the `-o` flag will output the example file to the STDOUT.


## Available Options

By using the YAML header (front matter) you can alter the behavior of rsmooth and use additional features.

Note: The whole content of the YAML header will be also available to pandoc and can thus be used in the template files using the `$VAR_NAME$` syntax.


### A word on paths

Rsmooth tries to resolve a variety of paths used in the configuration header. As they will get [shell expanded](https://tldp.org/LDP/Bash-Beginners-Guide/html/sect_03_04.html) you can use environment variables and tildes (`~`) for your home directory. Relative paths will be handled **relative to the input file's location**.


### Template

**Field Name:** `template`

**Description:** Path to the template file. Learn more about these files in the [pandoc documentation](https://pandoc.org/MANUAL.html#templates). If no template is given the default template of pandoc will be used.

**Default:** None.


### PDF Engine

`engine` Name of the LaTeX engine used to create the PDF document. This internally will set the [--pdf-engine](https://pandoc.org/MANUAL.html#option--pdf-engine) option of pandoc. This option defaults to `xelatex` (as this is what I'm working with).


### Pandoc Options

**Field Name:** `pandoc-options`

**Description:** Feed [additional options](https://pandoc.org/MANUAL.html#options) into the pandoc call. You can use this the same way as you passing command line options to a pandoc call.

**Default:** None.


### Apply input to Tera

**Field Name:** `do_tera`

**Description:** States whether the markdown input should be passed trough the [Terra](https://tera.netlify.app/) template engine. This allows you some additional flexibility over your input which cannot be achieved by tweaking the pandoc template file. Especially useful to split the content of your document into multiple markdown files and including them using the `{% include "section_01.md" %}` syntax of Tera. You can pass information to Tera using the [Tera Context](#tera-context) field. You can learn more about the syntax of Tera in it's [Documentation](https://tera.netlify.app/docs/#templates).

**Default:** `False`.


### Tera Context

**Field Name:** `tera_context`

**Description:** Sometimes it can be useful to pass some additional information to the Tera engine (context). This can be done by giving this field a map (dict) of strings to any type supported by YAML (learn more about maps in YAML [here](https://stackoverflow.com/a/34328811)).

**Default:** None.


## Environment Variables

rsmooth assumes the pandoc executable is callable with the `pandoc` command. You can use the environment variable `PANDOC_CMD` to alter this.
