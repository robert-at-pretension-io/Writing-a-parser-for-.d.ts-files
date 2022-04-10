# Immediate Project goals
* Make a 'live' parsing editor that has watched test cases and a live editor that runs these tests against the current iteration of the code.

# Long Term Project goals
* using these parsed types, functions from various projects, come up with a live two-way mapping between a graphic visualization and the code.

# LONG TERM TODO:
1. Determine which nodes are 'primative'. Where this is defined as a type whose properties are composed types: string, number or arrays of strings and numbers, or anonymous typed objects.
2. Composed types are types that include properties with types: strings, numbers, arrays of strings and numbers and primative types or arrays of primative types. 
3. Add an option for adding new primative types (including those that are not in the domain-- externally imported from different libraries)
4. Populate the graph with the composed types, adding the primative types to each of the composed types according to their structure.
5. Add edges between composed types