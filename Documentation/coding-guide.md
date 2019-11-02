# C++ Coding Style
For C++ files (*.cpp and *.h) we use clang-format to ensure code
styling. After changing or adding any Cpp or H file you need to reformat
the code before merging into the master branch. This script will ensure
that all native code files adhere to the coding style guidelines.

## Using clang-format in CLion
You can configure clang-format in CLion IDE simply by clicking the
`Indent` button on the right bottom location. Then, select `Enable
ClangFormat with clangd server`. Done!

If you want to reformat code automatically before commit, you can easily
enable it in `Settings>Version Control>Commit Dialog`. Just enable
`Reformat Code`.

## Coding Rules
1. We use `PascalCase` to name all our class names.
2. We use `camelCase` to name all our function names.
3. We use `snake_case` to name all our local variables and fields. For
   constant variables and fields we use `SNAKE_CASE`. 