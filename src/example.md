---
title: Document Title
author: John Doe
# template: ~/templates/document.tex
engine: pdflatex # defaults to xelatex
do_tera: True
tera_context:
	foo: A string variable
	list:
		- Eggs
		- Bread
		- Butter
---

# Introduction

This is a example document showing some of rsmooth's functionality. You can set your own pandoc template by setting the `template` field.

# Templating with Tera

By setting `do_tera` to `True` this file will passed to the Tera template engine. Use the `tera_context` to give additional information to Tera. For example: {{ foo }}, or render a list:

{% for item in list %}
- {{ item }}
{% endfor %}

Learn more about the Tera syntax in the [Documentation](https://tera.netlify.app/docs/#templates)
