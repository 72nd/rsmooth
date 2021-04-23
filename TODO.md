# TODO's

## Until 1.0.0 

- [x] Empty `template` field behavior. (v.0.2.0)
- [x] Tera Templating(v.0.2.0)
	- [x] Debug context (-)
	- [x] Rename header fields (v.0.2.0)
	- [x] Update documentation (v.0.2.0)
	- [x] Allow more complex template_context values (v.0.2.0)
	- [x] Update documentation (v.0.2.0)
- [ ] Documentation
	- [x] Finish `README.md` (v.0.2.0)
	- [x] Document available environment variables (?)
	- [ ] Example header with all options
	- [ ] Logo
- [x] Bibliography
	- [x] Fix relative imports in front matter header for bibliography(?)
	- [x] Fix depreciated citeproc filter (?)
	- [x] Path to Citation Style file (?)
	- [x] Bibliography stuff works (?)
	- [x] Documentation done (?)
- [x] Example file functionality
	- [x] Implement (v.0.2.1)
	- [x] Correct location for output path in CLI (v.0.2.1)
	- [x] Fix did you mean example command bug in CLI (v.0.2.1)
	- [x] Documentation done (v.0.2.1)
- [-] Filters (discarded)
	- [-] Implementation done (discarded)
	- [-] Activate as list in header (discarded)
	- [-] Template filter activation via list (discarded)
	- [-] Implement path expansion (discarded)
	- [-] Documentation done (discarded)
- [x] Relative paths in documents
	- [x] Fix the handling for relative resource links in documents (?)
	- [x] Remove unused expand_paths filter (?)
- [ ] Filters/Split description
	- [x] Move template module to tera module in root (?)
	- [ ] Implement embedded Lua filter structure
	- [ ] Implement as filter
	- [ ] Documentation done
- [ ] Release binaries
	- [ ] Build binaries
	- [ ] Document installation
- [ ] Other
	- [x] metadata.rs: Give Methods meaningful names (?)
	- [-] File ending for temporary file (discarded)


## Additional functionality

- [ ] Reveal.js integration
- [ ] Libre Office/Word output
- [ ] PDF export using Libre Office
- [ ] NeoVim Plugin
- [ ] metadata.rs
	- [ ] Remove Header/Metadata duplicate
- [ ] Load templates from URL's
- [ ] Wordcount

Lot of improvements and fixes:

- Bibliography
	- Resolve relative paths to citation style files.
	- Switched to the current citeproc filter.
- Relative Paths in input documents
	- Fix the relative linking of files within the document by setting pandoc's `--ressource-path` to the correct folder.
	- Fix the relative linking of bibliography files in the frontmatter header.
- Documentation (README):
	- Available environment variables documented.
	- Bibliography.
- Internal
	- Tidy up metadata.rs (method names).
	- Unused `expand_paths`filter removed.
	- Tera functionality lives now as module in the source root.
	- Unused `filters` module removed.
