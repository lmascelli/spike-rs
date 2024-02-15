from PySide6.QtGui import QColor, QTextCharFormat, QSyntaxHighlighter
from PySide6.QtWidgets import QApplication, QTextEdit
import re

class PythonHighlighter(QSyntaxHighlighter):
    KEYWORDS = ["and", "as", "assert", "break", "class", "continue", "def",
                "del", "elif", "else", "except", "exec", "finally", "for", "from",
                "global", "if", "import", "in", "is", "lambda", "not", "or",
                "pass", "print", "raise", "return", "try", "while", "with", "yield"]
    BUILTINS = ["abs", "divmod", "input", "open", "staticmethod", "all", "enumerate", "int", "ord", "str", "any",
                "eval", "isinstance", "pow", "sum", "basestring", "execfile", "issubclass", "print", "super",
                "bin", "file", "iter", "property", "tuple", "bool", "filter", "len", "range", "type", "bytearray",
                "float", "list", "raw_input", "unichr", "callable", "format", "locals", "reduce", "unicode",
                "chr", "frozenset", "long", "reload", "vars", "classmethod", "getattr", "map", "repr", "xrange",
                "cmp", "globals", "max", "reversed", "zip", "compile", "hasattr", "memoryview", "round",
                "complex", "hash", "min", "set", "delattr", "help", "next", "setattr", "dict", "hex", "object",
                "slice", "dir", "id", "oct", "sorted", "divmod", "input", "open", "staticmethod", "all", "exit",
                "quit", "bin", "bool", "memoryview", "exec", "list", "tuple", "format", "len", "property", "type",
                "frozenset", "locals", "range", "vars", "getattr", "map", "reversed", "zip", "globals", "max", "round",
                "compile", "hasattr", "print", "delattr", "hash", "oct", "setattr", "dict", "help", "open", "slice",
                "dir", "hex", "pow", "sorted", "id", "quit", "repr", "staticmethod", "issubclass", "iter", "reduce",
                "str", "sum", "import", "input", "raw_input", "intern"]

    def __init__(self, document):
        QSyntaxHighlighter.__init__(self, document)
        self.tri_single = (self.tri_single, self.tri_single)
        self.tri_double = (self.tri_double, self.tri_double)

        rules = []

        # Keyword, operator, and brace rules
        rules += [(r'\b%s\b' % w, 0, self.keywordFormat) for w in PythonHighlighter.KEYWORDS]
        rules += [(r'%s' % o, 0, self.operatorFormat)
                  for o in PythonHighlighter.OPERATORS]
        rules += [(r'%s' % b, 0, self.braceFormat)
                  for b in PythonHighlighter.BRACES]

        # All other rules
        rules += [
            # 'self'
            (r'\bself\b', 0, self.selfFormat),

            # Double-quoted string, possibly containing escape sequences
            (r'"[^"\\]*(\\.[^"\\]*)*"', 0, self.stringFormat),
            # Single-quoted string, possibly containing escape sequences
            (r"'[^'\\]*(\\.[^'\\]*)*'", 0, self.stringFormat),

            # 'def' followed by an identifier
            (r'\bdef\b\s*(\w+)', 1, self.defclassFormat),
            # 'class' followed by an identifier
            (r'\bclass\b\s*(\w+)', 1, self.defclassFormat),

            # From '#' until a newline
            (r'#[^\n]*', 0, self.commentFormat),

            # Numeric literals
            (r'\b[+-]?[0-9]+[lL]?\b', 0, self.numberFormat),
            (r'\b[+-]?0[xX][0-9A-Fa-f]+[lL]?\b', 0, self.numberFormat),
            (r'\b[+-]?[0-9]+(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?\b', 0, self.numberFormat),
        ]

        # Build a QRegExp for each pattern
        self.rules = [(QRegExp(pat), index, fmt) for (pat, index, fmt) in rules]

    def highlightBlock(self, text):
        """Apply syntax highlighting to the given block of text."""
        # Do other syntax formatting
        for expression, nth, format in self.rules:
            index = expression.indexIn(text, 0)

            while index >= 0:
                # We actually want the index of the nth match
                index = expression.pos(nth)
                length = len(expression.cap(nth))
                self.setFormat(index, length, format)
                index = expression.indexIn(text, index + length)

        self.setCurrentBlockState(0)

        # Do multi-line strings
        in_multiline = self.match_multiline(text, *self.tri_single)
        if not in_multiline:
            in_multiline = self.match_multiline(text, *self.tri_double)

    def match_multiline(self, text, delimiter, in_state, style):
        """Do highlighting of multi-line strings. ``delimiter`` should be a
        ``QRegExp`` for triple-single-quotes or triple-double-quotes, and
        ``in_state`` should be a unique integer to represent the corresponding
        state changes when inside those strings. Returns True if we're still
        inside a multi-line string when this function is finished.
        """
        # If inside triple-single quotes, start at 0
        if self.previousBlockState() == in_state:
            start = 0
            add = 0
        # Otherwise, look for the delimiter on this line
        else:
            start = delimiter.indexIn(text)
            # Move past this match
            add = delimiter.matchedLength()

        # As long as there's a delimiter match on this line...
        while start >= 0:
            # Look for the ending delimiter
            end = delimiter.indexIn(text, start + add)
            # Ending delimiter on this line?
            if end >= add:
                length = end - start + add + delimiter.matchedLength()
                self.setCurrentBlockState(0)
            # No; multi-line string
            else:
                self.setCurrentBlockState(in_state)
                length = text.length() - start + add
            # Apply formatting
            self.setFormat(start, length, style)
            # Look for the next match
            start = delimiter.indexIn(text, start + length)

        # Return True if still inside a multi-line string, False otherwise
        if self.currentBlockState() == in_state:
            return True
        else:
            return False

class PythonEditor(QTextEdit):
   def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.highlighter = PythonHighlighter(self.document())
