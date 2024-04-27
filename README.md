# SUI Development Move Formatter

This formatter exists because Mystenlabs has not yet created a formatter for Move code that uses the most up-to-date development features. This formatter aims to support:

- [x] Nested Use
- [x] Public Package
- [x] Postfix Abilities
- [x] Struct Type Visibility
- [x] Dot Call
- [x] Positional Fields
- [x] LetMut
- [x] Block Labels
- [ ] Move 2024 Paths
- [ ] No Parens Cast
- [x] Enum definitions
- [x] Match statements
- [ ] Clever Assertions

# Local installation for Visual Studio Code

`yarn`

Compile the binary: `yarn build`

Build the vsce (Visual Studio Code Extensions) package: `yarn package`

Debug run the extension: `F5` output is visible in `OUTPUT` (next to the terminal) when selecting `Log (Extension Host)` at the top right

Install the extension: `yarn install:local`
