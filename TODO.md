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
	- [ ] Example header with all options
	- [ ] Logo
- [ ] Bibliography
	- [ ] Path to Citation Style file
	- [ ] Bibliography stuff works
	- [ ] Documentation done
- [x] Example file functionality
	- [x] Implement (?)
	- [x] Correct location for output path in CLI (?)
	- [x] Fix did you mean example command bug in CLI (?)
	- [x] Documentation done (?)
- [ ] Filters
	- [ ] Implementation done
	- [ ] Activate as list in header
	- [ ] Template filter activation via list
	- [ ] Documentation done
- [ ] Split description
	- [ ] Implement as filter
	- [ ] Documentation done
- [ ] Release binaries
	- [ ] Build binaries
	- [ ] Document installation


## Additional functionality

- [ ] Reveal.js integration
- [ ] Libre Office/Word output
- [ ] PDF export using Libre Office
- [ ] NeoVim Plugin
- [ ] metadata.rs
	- [ ] Remove Header/Metadata duplicate
	- [ ] Give Methods meaningful names
- 


- An empty or unset `template` field in the YAML header will now correctly fall back to the default Pandoc template.
- README improved (all header fields documented).
- Renamed header fields to more meaningful terms (`do_template` to `do_tera` and `template_context` to `tera_context`).
- Allow any YAML data type as value in the `tera_context` map.
