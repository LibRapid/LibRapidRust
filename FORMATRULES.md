Formatting rules for contributing
========
If you want to contribute to LibRapid, be sure to follow these rules to have consistency in the code.
_____
The key words “MUST”, “MUST NOT”, “REQUIRED”, “SHALL”, “SHALL NOT”, “SHOULD”, “SHOULD NOT”, “RECOMMENDED”, “MAY”, and “OPTIONAL” in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).
_____
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
When explicitly delcaring a type, the colon `:` is places right after the name of the constant or variable.

§4 Operators
----
Arithmetic and bitwise operators `+`,`-`,`*`,`/`,`%`,`&`,`^`,`|`,`<<`,`>>` MUST have one trailing and leading space.

Logic operators `>`,`>=`,`<`,`<=`,`==`,`!=`,`&&`,`||` MUST have one trailing and leading space.

One SHOULD split up statements using `&&`,`||`, when one line is becoming too long (subjective to the editor). This MUST be done having a line break instead of a trailing space, while the right hand side MUST begin on the same column as the left hand side.

§5 Others
----
Other characters and substrings, such as `->` MUST have one trailing and one leading space.