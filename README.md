# Immediate Project goals
* Make a 'live' parsing editor that has watched test cases and a live editor that runs these tests against the current iteration of the code.
## Towards this end, let's attempt to implement the following workflow:
* select a parser of interest to work on
* select test cases
* open the parser in an editor (SEPARATE FROM TERMINAL FOR NOW)
* modify parser causes: 
    * recompilation
    * running the test cases
    * succinct report non-passing tests
    * repeat

## IMPLEMENTATION DETAILS
* Use tokio for file watching
* use pest to come up with grammars
* use wasmer for the wasm runtime (for running the parser compiled to wasm on the tests)
* the abstract syntax tree IS a graph. The names of the nodes should be the parts of the grammar -- the 'pairs'.

## Need to decide:
* Q: How will the grammar be transformed into an AST in a general way?
* A: There will be another abstraction layer specifying how the various components of the grammar are displayed.

# Long Term Project goals
* using these parsed types, functions from various projects, come up with a live two-way mapping between a graphic visualization and the code.

# LONG TERM TODO:
1. Determine which nodes are 'primative'. Where this is defined as a type whose properties are composed types: string, number or arrays of strings and numbers, or anonymous typed objects.
2. Composed types are types that include properties with types: strings, numbers, arrays of strings and numbers and primative types or arrays of primative types. 
3. Add an option for adding new primative types (including those that are not in the domain-- externally imported from different libraries)
4. Populate the graph with the composed types, adding the primative types to each of the composed types according to their structure.
5. Add edges between composed types