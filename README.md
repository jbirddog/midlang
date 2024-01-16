# midlang

_Language frontends can target this._

`midlang` is a compiler toolchain that provides the middle and backend components for a traditional compiler. While `midlang` itself does not have a traditional textual representation, `json_lang` exists as a [narrow waist](https://www.oilshell.org/cross-ref.html?tag=narrow-waist#narrow-waist). Rust frontends can directly leverage the crates in this repository.  

## External Frontends

| Name | Notes |
|----|----|
| [Supreme Spoon](https://github.com/jbirddog/supreme-spoon) | Python based frontend that builds `json_lang` from BPMN diagrams parsed by [SpiffWorkflow](https://github.com/sartography/SpiffWorkflow). |

## Backends

### QBE

The `qbe` backend takes a list of `middlang` modules and lowers them to its `lower language` to build a list of `CompUnit`s. These compilation units are then used to build QBE IL files, which are turned into assembly files and finally compiled to object files. The `lower language` (`lower_lang.rs`) and IL generator (`il.rs`) are built up as needed and currently support:

- [x] QBE types
- [ ] Subtyping
- [ ] Constants
   - [X] Integer compile-time constants
   - [ ] Float compile-time constants
   - [ ] Dynamic constants
- [ ] Linkage
   - [x] Export
   - [ ] Thread
   - [ ] Section
- [ ] Aggregate Types
- [x] Data
- [x] Functions
- [x] Labels
- [ ] Control flow
   - [x] jmp
   - [x] jnz
   - [x] ret
   - [ ] hlt
- [ ] Instructions 
