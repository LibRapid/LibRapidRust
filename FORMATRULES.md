Formatting rules for contributing
========
If you want to contribute to LibRapid, be sure to follow these rules to have consistency in the code.

§1 Curly braces
------
A opening curly brace `{` MUST be placed on the same line as its statement with a leading space, except
* the statement inside the curly braces is only one line, in which case the statement and the curly braces `{` and `}` MAY all be placed on the same line with one space before and after the statement.

A closing curly brace `}` MUST be placed on a following line on the same column as the beginning of the beforegoing statement, except the rule for `{` is applied.

§2 Tabs and spaces
------
Tabs MUST be 4 spaces wide and MUST be applied where:
* A statement is inside curly braces and on a separate line,
* or a definition has occured inside curly braces.

Spaces SHOULD be applied where:
* Multiple types are declared in following lines to bring them onto the same column or
* multiple definitions after a `=` on following lines take place to bring them onto the same column.

§3 Types
-----
Types MAY be inferred, even though a explicit type declaration is preffered.