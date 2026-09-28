/**
 * Prady Interactive In-Browser Compiler & Runtime Engine (v0.1.0)
 * Lexer, Parser, AST Builder, Diagnostic Bag, and Tree-Walking Evaluator
 */

(function (window) {
  // --- Token Kinds ---
  const TokenKind = {
    // Keywords
    Fn: 'Fn',
    Let: 'Let',
    Mut: 'Mut',
    Return: 'Return',
    If: 'If',
    Else: 'Else',
    While: 'While',
    For: 'For',
    In: 'In',
    Architecture: 'Architecture',
    Layer: 'Layer',
    Spec: 'Spec',
    Contract: 'Contract',
    Cannot: 'Cannot',
    Import: 'Import',
    Class: 'Class',
    Interface: 'Interface',
    Struct: 'Struct',
    Enum: 'Enum',
    Match: 'Match',
    True: 'True',
    False: 'False',
    Null: 'Null',

    // Literals & Identifiers
    Ident: 'Ident',
    IntLit: 'IntLit',
    FloatLit: 'FloatLit',
    StringLit: 'StringLit',

    // Symbols & Operators
    Arrow: 'Arrow',           // ->
    FatArrow: 'FatArrow',     // =>
    EqEq: 'EqEq',             // ==
    NotEq: 'NotEq',           // !=
    LtEq: 'LtEq',             // <=
    GtEq: 'GtEq',             // >=
    PlusEq: 'PlusEq',         // +=
    MinusEq: 'MinusEq',       // -=
    StarEq: 'StarEq',         // *=
    SlashEq: 'SlashEq',       // /=
    AndAnd: 'AndAnd',         // &&
    OrOr: 'OrOr',             // ||
    Eq: 'Eq',                 // =
    Plus: 'Plus',             // +
    Minus: 'Minus',           // -
    Star: 'Star',             // *
    Slash: 'Slash',           // /
    Percent: 'Percent',       // %
    Lt: 'Lt',                 // <
    Gt: 'Gt',                 // >
    Bang: 'Bang',             // !
    Colon: 'Colon',           // :
    Semicolon: 'Semicolon',   // ;
    Comma: 'Comma',           // ,
    Dot: 'Dot',               // .
    Question: 'Question',     // ?
    At: 'At',                 // @

    // Delimiters
    LBrace: 'LBrace',         // {
    RBrace: 'RBrace',         // }
    LParen: 'LParen',         // (
    RParen: 'RParen',         // )
    LBracket: 'LBracket',     // [
    RBracket: 'RBracket',     // ]

    Comment: 'Comment',
    Eof: 'Eof',
  };

  const KEYWORDS = {
    fn: TokenKind.Fn,
    let: TokenKind.Let,
    mut: TokenKind.Mut,
    return: TokenKind.Return,
    if: TokenKind.If,
    else: TokenKind.Else,
    while: TokenKind.While,
    for: TokenKind.For,
    in: TokenKind.In,
    architecture: TokenKind.Architecture,
    layer: TokenKind.Layer,
    spec: TokenKind.Spec,
    contract: TokenKind.Contract,
    cannot: TokenKind.Cannot,
    import: TokenKind.Import,
    class: TokenKind.Class,
    interface: TokenKind.Interface,
    struct: TokenKind.Struct,
    enum: TokenKind.Enum,
    match: TokenKind.Match,
    true: TokenKind.True,
    false: TokenKind.False,
    null: TokenKind.Null,
  };

  // --- Lexer ---
  class Lexer {
    constructor(source) {
      this.source = source;
      this.pos = 0;
      this.line = 1;
      this.col = 1;
      this.tokens = [];
      this.errors = [];
    }

    peek(offset = 0) {
      const idx = this.pos + offset;
      return idx < this.source.length ? this.source[idx] : null;
    }

    advance() {
      const ch = this.peek();
      if (ch === null) return null;
      this.pos++;
      if (ch === '\n') {
        this.line++;
        this.col = 1;
      } else {
        this.col++;
      }
      return ch;
    }

    tokenize() {
      while (this.pos < this.source.length) {
        const ch = this.peek();

        if (ch === ' ' || ch === '\t' || ch === '\r' || ch === '\n') {
          this.advance();
          continue;
        }

        // Single-line or Multi-line comments
        if (ch === '/' && this.peek(1) === '/') {
          const startCol = this.col;
          const startLine = this.line;
          let text = '';
          while (this.peek() !== null && this.peek() !== '\n') {
            text += this.advance();
          }
          this.tokens.push({
            kind: TokenKind.Comment,
            text,
            line: startLine,
            col: startCol,
          });
          continue;
        }

        if (ch === '/' && this.peek(1) === '*') {
          const startCol = this.col;
          const startLine = this.line;
          let text = this.advance() + this.advance();
          while (this.peek() !== null) {
            if (this.peek() === '*' && this.peek(1) === '/') {
              text += this.advance() + this.advance();
              break;
            }
            text += this.advance();
          }
          this.tokens.push({
            kind: TokenKind.Comment,
            text,
            line: startLine,
            col: startCol,
          });
          continue;
        }

        const startLine = this.line;
        const startCol = this.col;

        // Strings
        if (ch === '"' || ch === "'") {
          const quote = this.advance();
          let text = '';
          while (this.peek() !== null && this.peek() !== quote) {
            if (this.peek() === '\\') {
              this.advance();
              const esc = this.advance();
              if (esc === 'n') text += '\n';
              else if (esc === 't') text += '\t';
              else if (esc === 'r') text += '\r';
              else text += esc;
            } else {
              text += this.advance();
            }
          }
          if (this.peek() === quote) {
            this.advance();
          } else {
            this.errors.push({
              line: startLine,
              col: startCol,
              message: 'Unterminated string literal',
            });
          }
          this.tokens.push({
            kind: TokenKind.StringLit,
            text,
            raw: `"${text}"`,
            line: startLine,
            col: startCol,
          });
          continue;
        }

        // Numbers
        if (/[0-9]/.test(ch)) {
          let numStr = '';
          let isFloat = false;
          while (this.peek() !== null && /[0-9_]/.test(this.peek())) {
            numStr += this.advance();
          }
          if (this.peek() === '.' && /[0-9]/.test(this.peek(1))) {
            isFloat = true;
            numStr += this.advance(); // .
            while (this.peek() !== null && /[0-9_]/.test(this.peek())) {
              numStr += this.advance();
            }
          }
          this.tokens.push({
            kind: isFloat ? TokenKind.FloatLit : TokenKind.IntLit,
            text: numStr,
            value: isFloat ? parseFloat(numStr.replace(/_/g, '')) : parseInt(numStr.replace(/_/g, ''), 10),
            line: startLine,
            col: startCol,
          });
          continue;
        }

        // Identifiers and keywords
        if (/[a-zA-Z_]/.test(ch)) {
          let ident = '';
          while (this.peek() !== null && /[a-zA-Z0-9_]/.test(this.peek())) {
            ident += this.advance();
          }
          const kind = KEYWORDS[ident] || TokenKind.Ident;
          this.tokens.push({
            kind,
            text: ident,
            line: startLine,
            col: startCol,
          });
          continue;
        }

        // Two-character symbols
        const twoChar = ch + (this.peek(1) || '');
        if (twoChar === '->') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.Arrow, text: '->', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '=>') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.FatArrow, text: '=>', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '==') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.EqEq, text: '==', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '!=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.NotEq, text: '!=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '<=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.LtEq, text: '<=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '>=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.GtEq, text: '>=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '+=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.PlusEq, text: '+=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '-=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.MinusEq, text: '-=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '*=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.StarEq, text: '*=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '/=') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.SlashEq, text: '/=', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '&&') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.AndAnd, text: '&&', line: startLine, col: startCol });
          continue;
        }
        if (twoChar === '||') {
          this.advance(); this.advance();
          this.tokens.push({ kind: TokenKind.OrOr, text: '||', line: startLine, col: startCol });
          continue;
        }

        // Single-character symbols
        this.advance();
        const singleKinds = {
          '{': TokenKind.LBrace,
          '}': TokenKind.RBrace,
          '(': TokenKind.LParen,
          ')': TokenKind.RParen,
          '[': TokenKind.LBracket,
          ']': TokenKind.RBracket,
          ':': TokenKind.Colon,
          ';': TokenKind.Semicolon,
          ',': TokenKind.Comma,
          '.': TokenKind.Dot,
          '?': TokenKind.Question,
          '@': TokenKind.At,
          '+': TokenKind.Plus,
          '-': TokenKind.Minus,
          '*': TokenKind.Star,
          '/': TokenKind.Slash,
          '%': TokenKind.Percent,
          '=': TokenKind.Eq,
          '<': TokenKind.Lt,
          '>': TokenKind.Gt,
          '!': TokenKind.Bang,
        };

        const kind = singleKinds[ch];
        if (kind) {
          this.tokens.push({ kind, text: ch, line: startLine, col: startCol });
        } else {
          this.errors.push({
            line: startLine,
            col: startCol,
            message: `Unexpected character '${ch}'`,
          });
        }
      }

      this.tokens.push({ kind: TokenKind.Eof, text: '', line: this.line, col: this.col });
      return { tokens: this.tokens, errors: this.errors };
    }
  }

  // --- Parser ---
  class Parser {
    constructor(tokens, sourceLines) {
      this.tokens = tokens.filter((t) => t.kind !== TokenKind.Comment);
      this.pos = 0;
      this.sourceLines = sourceLines;
      this.diagnostics = [];
    }

    current() {
      return this.pos < this.tokens.length
        ? this.tokens[this.pos]
        : { kind: TokenKind.Eof, text: '', line: 1, col: 1 };
    }

    peek(offset = 1) {
      const idx = this.pos + offset;
      return idx < this.tokens.length
        ? this.tokens[idx]
        : { kind: TokenKind.Eof, text: '', line: 1, col: 1 };
    }

    check(kind) {
      return this.current().kind === kind;
    }

    match(...kinds) {
      for (const k of kinds) {
        if (this.check(k)) {
          return this.advance();
        }
      }
      return null;
    }

    advance() {
      const tok = this.current();
      if (!this.check(TokenKind.Eof)) {
        this.pos++;
      }
      return tok;
    }

    consume(kind, errMsg) {
      if (this.check(kind)) {
        return this.advance();
      }
      const tok = this.current();
      this.diagnostics.push({
        level: 'error',
        line: tok.line,
        col: tok.col,
        message: `${errMsg} (found '${tok.text || 'EOF'}')`,
      });
      return null;
    }

    parseProgram() {
      const items = [];
      while (!this.check(TokenKind.Eof)) {
        try {
          const item = this.parseItem();
          if (item) items.push(item);
          else this.advance();
        } catch (e) {
          this.synchronize();
        }
      }
      return { type: 'Program', items, diagnostics: this.diagnostics };
    }

    synchronize() {
      this.advance();
      while (!this.check(TokenKind.Eof)) {
        if (this.tokens[this.pos - 1]?.kind === TokenKind.Semicolon) return;
        if (
          [
            TokenKind.Fn,
            TokenKind.Let,
            TokenKind.Architecture,
            TokenKind.Class,
            TokenKind.Interface,
            TokenKind.Enum,
            TokenKind.Return,
          ].includes(this.current().kind)
        ) {
          return;
        }
        this.advance();
      }
    }

    parseItem() {
      if (this.check(TokenKind.Fn)) return this.parseFunction();
      if (this.check(TokenKind.Architecture)) return this.parseArchitecture();
      if (this.check(TokenKind.Interface)) return this.parseInterface();
      if (this.check(TokenKind.Class)) return this.parseClass();
      if (this.check(TokenKind.Enum)) return this.parseEnum();
      if (this.check(TokenKind.Let)) return this.parseLetStatement();
      return this.parseStatement();
    }

    parseFunction() {
      const fnTok = this.consume(TokenKind.Fn, 'Expected fn');
      const nameTok = this.consume(TokenKind.Ident, 'Expected function name');
      this.consume(TokenKind.LParen, "Expected '(' after function name");

      const params = [];
      while (!this.check(TokenKind.RParen) && !this.check(TokenKind.Eof)) {
        const paramName = this.consume(TokenKind.Ident, 'Expected parameter name');
        let paramType = 'Any';
        if (this.match(TokenKind.Colon)) {
          paramType = this.parseType();
        }
        if (paramName) params.push({ name: paramName.text, type: paramType });
        if (!this.match(TokenKind.Comma)) break;
      }
      this.consume(TokenKind.RParen, "Expected ')' after parameters");

      let returnType = 'Void';
      if (this.match(TokenKind.Arrow)) {
        returnType = this.parseType();
      }

      const body = this.parseBlock();
      return {
        type: 'Function',
        name: nameTok ? nameTok.text : '<anonymous>',
        params,
        returnType,
        body,
        line: fnTok.line,
        col: fnTok.col,
      };
    }

    parseArchitecture() {
      const archTok = this.consume(TokenKind.Architecture, 'Expected architecture');
      const nameTok = this.consume(TokenKind.Ident, 'Expected architecture name');
      this.consume(TokenKind.LBrace, "Expected '{'");

      const layers = [];
      const rules = [];

      while (!this.check(TokenKind.RBrace) && !this.check(TokenKind.Eof)) {
        if (this.match(TokenKind.Layer)) {
          const layerName = this.consume(TokenKind.Ident, 'Expected layer name');
          this.match(TokenKind.Semicolon);
          if (layerName) layers.push(layerName.text);
        } else if (this.check(TokenKind.Ident)) {
          const from = this.advance().text;
          if (this.match(TokenKind.Arrow)) {
            const to = this.consume(TokenKind.Ident, 'Expected destination layer');
            this.match(TokenKind.Semicolon);
            if (to) rules.push({ type: 'allow', from, to: to.text });
          } else if (this.match(TokenKind.Cannot)) {
            this.consume(TokenKind.Import, "Expected 'import' after cannot");
            const forbidden = this.consume(TokenKind.Ident, 'Expected target layer');
            this.match(TokenKind.Semicolon);
            if (forbidden) rules.push({ type: 'deny', from, to: forbidden.text });
          } else {
            this.advance();
          }
        } else {
          this.advance();
        }
      }
      this.consume(TokenKind.RBrace, "Expected '}'");

      return {
        type: 'Architecture',
        name: nameTok ? nameTok.text : 'system',
        layers,
        rules,
        line: archTok.line,
        col: archTok.col,
      };
    }

    parseInterface() {
      const tok = this.consume(TokenKind.Interface, 'Expected interface');
      const nameTok = this.consume(TokenKind.Ident, 'Expected interface name');
      this.consume(TokenKind.LBrace, "Expected '{'");
      const methods = [];
      while (!this.check(TokenKind.RBrace) && !this.check(TokenKind.Eof)) {
        if (this.match(TokenKind.Fn)) {
          const mName = this.consume(TokenKind.Ident, 'Expected method name');
          this.consume(TokenKind.LParen, "Expected '('");
          // skip args
          while (!this.check(TokenKind.RParen) && !this.check(TokenKind.Eof)) this.advance();
          this.match(TokenKind.RParen);
          let ret = 'Void';
          if (this.match(TokenKind.Arrow)) ret = this.parseType();
          this.match(TokenKind.Semicolon);
          if (mName) methods.push({ name: mName.text, returnType: ret });
        } else {
          this.advance();
        }
      }
      this.consume(TokenKind.RBrace, "Expected '}'");
      return { type: 'Interface', name: nameTok ? nameTok.text : 'Interface', methods, line: tok.line };
    }

    parseClass() {
      const tok = this.consume(TokenKind.Class, 'Expected class');
      const nameTok = this.consume(TokenKind.Ident, 'Expected class name');
      const body = this.parseBlock();
      return { type: 'Class', name: nameTok ? nameTok.text : 'Class', body, line: tok.line };
    }

    parseEnum() {
      const tok = this.consume(TokenKind.Enum, 'Expected enum');
      const nameTok = this.consume(TokenKind.Ident, 'Expected enum name');
      this.consume(TokenKind.LBrace, "Expected '{'");
      const variants = [];
      while (!this.check(TokenKind.RBrace) && !this.check(TokenKind.Eof)) {
        if (this.check(TokenKind.Ident)) {
          const v = this.advance().text;
          if (this.match(TokenKind.LParen)) {
            while (!this.check(TokenKind.RParen) && !this.check(TokenKind.Eof)) this.advance();
            this.match(TokenKind.RParen);
          }
          variants.push(v);
          this.match(TokenKind.Comma);
        } else {
          this.advance();
        }
      }
      this.consume(TokenKind.RBrace, "Expected '}'");
      return { type: 'Enum', name: nameTok ? nameTok.text : 'Enum', variants, line: tok.line };
    }

    parseBlock() {
      this.consume(TokenKind.LBrace, "Expected '{'");
      const statements = [];
      while (!this.check(TokenKind.RBrace) && !this.check(TokenKind.Eof)) {
        const stmt = this.parseStatement();
        if (stmt) statements.push(stmt);
      }
      this.consume(TokenKind.RBrace, "Expected '}'");
      return { type: 'Block', statements };
    }

    parseStatement() {
      if (this.check(TokenKind.Let)) return this.parseLetStatement();
      if (this.check(TokenKind.Return)) return this.parseReturnStatement();
      if (this.check(TokenKind.If)) return this.parseIfStatement();
      if (this.check(TokenKind.While)) return this.parseWhileStatement();
      if (this.check(TokenKind.LBrace)) return this.parseBlock();

      // Expression or Assignment statement
      const expr = this.parseExpression();
      if (this.match(TokenKind.PlusEq)) {
        const value = this.parseExpression();
        this.match(TokenKind.Semicolon);
        return { type: 'AssignOp', op: '+=', target: expr, value };
      }
      if (this.match(TokenKind.MinusEq)) {
        const value = this.parseExpression();
        this.match(TokenKind.Semicolon);
        return { type: 'AssignOp', op: '-=', target: expr, value };
      }
      if (this.match(TokenKind.Eq)) {
        const value = this.parseExpression();
        this.match(TokenKind.Semicolon);
        return { type: 'Assign', target: expr, value };
      }

      this.match(TokenKind.Semicolon);
      return { type: 'ExprStmt', expr };
    }

    parseLetStatement() {
      this.consume(TokenKind.Let, 'Expected let');
      const isMut = !!this.match(TokenKind.Mut);
      const nameTok = this.consume(TokenKind.Ident, 'Expected variable name');

      let varType = null;
      if (this.match(TokenKind.Colon)) {
        varType = this.parseType();
      }

      let init = null;
      if (this.match(TokenKind.Eq)) {
        init = this.parseExpression();
      }
      this.match(TokenKind.Semicolon);

      return {
        type: 'Let',
        name: nameTok ? nameTok.text : '_',
        isMut,
        varType,
        init,
      };
    }

    parseReturnStatement() {
      const retTok = this.consume(TokenKind.Return, 'Expected return');
      let value = null;
      if (!this.check(TokenKind.Semicolon) && !this.check(TokenKind.RBrace)) {
        value = this.parseExpression();
      }
      this.match(TokenKind.Semicolon);
      return { type: 'Return', value, line: retTok.line };
    }

    parseIfStatement() {
      this.consume(TokenKind.If, 'Expected if');
      const condition = this.parseExpression();
      const thenBranch = this.parseBlock();
      let elseBranch = null;
      if (this.match(TokenKind.Else)) {
        if (this.check(TokenKind.If)) {
          elseBranch = this.parseIfStatement();
        } else {
          elseBranch = this.parseBlock();
        }
      }
      return { type: 'If', condition, thenBranch, elseBranch };
    }

    parseWhileStatement() {
      this.consume(TokenKind.While, 'Expected while');
      const condition = this.parseExpression();
      const body = this.parseBlock();
      return { type: 'While', condition, body };
    }

    parseType() {
      let base = this.consume(TokenKind.Ident, 'Expected type identifier')?.text || 'Any';
      if (this.match(TokenKind.Lt)) {
        const args = [];
        while (!this.check(TokenKind.Gt) && !this.check(TokenKind.Eof)) {
          args.push(this.parseType());
          if (!this.match(TokenKind.Comma)) break;
        }
        this.match(TokenKind.Gt);
        return `${base}<${args.join(', ')}>`;
      }
      return base;
    }

    // Expressions (Pratt / Precedence Climbing)
    parseExpression() {
      return this.parseLogicalOr();
    }

    parseLogicalOr() {
      let left = this.parseLogicalAnd();
      while (this.match(TokenKind.OrOr)) {
        const right = this.parseLogicalAnd();
        left = { type: 'Binary', op: '||', left, right };
      }
      return left;
    }

    parseLogicalAnd() {
      let left = this.parseEquality();
      while (this.match(TokenKind.AndAnd)) {
        const right = this.parseEquality();
        left = { type: 'Binary', op: '&&', left, right };
      }
      return left;
    }

    parseEquality() {
      let left = this.parseComparison();
      while (true) {
        if (this.match(TokenKind.EqEq)) {
          left = { type: 'Binary', op: '==', left, right: this.parseComparison() };
        } else if (this.match(TokenKind.NotEq)) {
          left = { type: 'Binary', op: '!=', left, right: this.parseComparison() };
        } else break;
      }
      return left;
    }

    parseComparison() {
      let left = this.parseTerm();
      while (true) {
        if (this.match(TokenKind.Lt)) {
          left = { type: 'Binary', op: '<', left, right: this.parseTerm() };
        } else if (this.match(TokenKind.LtEq)) {
          left = { type: 'Binary', op: '<=', left, right: this.parseTerm() };
        } else if (this.match(TokenKind.Gt)) {
          left = { type: 'Binary', op: '>', left, right: this.parseTerm() };
        } else if (this.match(TokenKind.GtEq)) {
          left = { type: 'Binary', op: '>=', left, right: this.parseTerm() };
        } else break;
      }
      return left;
    }

    parseTerm() {
      let left = this.parseFactor();
      while (true) {
        if (this.match(TokenKind.Plus)) {
          left = { type: 'Binary', op: '+', left, right: this.parseFactor() };
        } else if (this.match(TokenKind.Minus)) {
          left = { type: 'Binary', op: '-', left, right: this.parseFactor() };
        } else break;
      }
      return left;
    }

    parseFactor() {
      let left = this.parseUnary();
      while (true) {
        if (this.match(TokenKind.Star)) {
          left = { type: 'Binary', op: '*', left, right: this.parseUnary() };
        } else if (this.match(TokenKind.Slash)) {
          left = { type: 'Binary', op: '/', left, right: this.parseUnary() };
        } else if (this.match(TokenKind.Percent)) {
          left = { type: 'Binary', op: '%', left, right: this.parseUnary() };
        } else break;
      }
      return left;
    }

    parseUnary() {
      if (this.match(TokenKind.Bang)) {
        return { type: 'Unary', op: '!', expr: this.parseUnary() };
      }
      if (this.match(TokenKind.Minus)) {
        return { type: 'Unary', op: '-', expr: this.parseUnary() };
      }
      return this.parseCallOrMember();
    }

    parseCallOrMember() {
      let expr = this.parsePrimary();

      while (true) {
        if (this.match(TokenKind.LParen)) {
          const args = [];
          while (!this.check(TokenKind.RParen) && !this.check(TokenKind.Eof)) {
            args.push(this.parseExpression());
            if (!this.match(TokenKind.Comma)) break;
          }
          this.consume(TokenKind.RParen, "Expected ')' after call arguments");
          expr = { type: 'Call', callee: expr, args };
        } else if (this.match(TokenKind.Dot)) {
          const member = this.consume(TokenKind.Ident, 'Expected property or method name');
          expr = { type: 'Member', object: expr, member: member ? member.text : '' };
        } else {
          break;
        }
      }
      return expr;
    }

    parsePrimary() {
      if (this.match(TokenKind.True)) return { type: 'Literal', value: true };
      if (this.match(TokenKind.False)) return { type: 'Literal', value: false };
      if (this.match(TokenKind.Null)) return { type: 'Literal', value: null };

      if (this.check(TokenKind.IntLit)) {
        const tok = this.advance();
        return { type: 'Literal', value: tok.value !== undefined ? tok.value : parseInt(tok.text, 10) };
      }
      if (this.check(TokenKind.FloatLit)) {
        const tok = this.advance();
        return { type: 'Literal', value: tok.value !== undefined ? tok.value : parseFloat(tok.text) };
      }
      if (this.check(TokenKind.StringLit)) {
        const tok = this.advance();
        return { type: 'Literal', value: tok.text };
      }

      if (this.check(TokenKind.Ident)) {
        const tok = this.advance();
        return { type: 'Identifier', name: tok.text };
      }

      if (this.match(TokenKind.LParen)) {
        const expr = this.parseExpression();
        this.consume(TokenKind.RParen, "Expected ')'");
        return expr;
      }

      const tok = this.current();
      this.diagnostics.push({
        level: 'error',
        line: tok.line,
        col: tok.col,
        message: `Unexpected token '${tok.text || 'EOF'}'`,
      });
      this.advance();
      return { type: 'Error' };
    }
  }

  // --- Runtime Evaluator ---
  class Environment {
    constructor(parent = null) {
      this.parent = parent;
      this.vars = new Map();
      this.mutability = new Map();
    }

    define(name, value, isMut = false) {
      this.vars.set(name, value);
      this.mutability.set(name, isMut);
    }

    assign(name, value) {
      if (this.vars.has(name)) {
        if (!this.mutability.get(name)) {
          throw new Error(`Cannot reassign immutable variable '${name}'. Declare with 'let mut' to allow mutation.`);
        }
        this.vars.set(name, value);
        return;
      }
      if (this.parent) {
        this.parent.assign(name, value);
        return;
      }
      throw new Error(`Variable '${name}' is not defined.`);
    }

    get(name) {
      if (this.vars.has(name)) return this.vars.get(name);
      if (this.parent) return this.parent.get(name);
      throw new Error(`Undefined identifier '${name}'`);
    }
  }

  class Evaluator {
    constructor(ast, onPrint) {
      this.ast = ast;
      this.onPrint = onPrint || console.log;
      this.globalEnv = new Environment();
      this.functions = new Map();
      this.setupBuiltins();
    }

    setupBuiltins() {
      this.globalEnv.define('print', (val) => {
        const str = this.formatValue(val);
        this.onPrint(str);
        return null;
      });
      this.globalEnv.define('println', (val) => {
        const str = this.formatValue(val);
        this.onPrint(str);
        return null;
      });
      this.globalEnv.define('assert', (cond, msg) => {
        if (!cond) throw new Error(`Assertion failed${msg ? ': ' + msg : ''}`);
        return true;
      });
    }

    formatValue(v) {
      if (v === null || v === undefined) return 'null';
      if (typeof v === 'boolean') return v ? 'true' : 'false';
      return String(v);
    }

    evaluate() {
      // Collect functions and declarations
      for (const item of this.ast.items) {
        if (item.type === 'Function') {
          this.functions.set(item.name, item);
        }
      }

      if (!this.functions.has('main')) {
        throw new Error("No 'main()' function found. Every Prady program starts execution in 'fn main()'.");
      }

      const mainFn = this.functions.get('main');
      return this.executeFunction(mainFn, [], this.globalEnv);
    }

    executeFunction(fnNode, args, parentEnv) {
      const env = new Environment(this.globalEnv);

      // Bind parameters
      for (let i = 0; i < fnNode.params.length; i++) {
        const p = fnNode.params[i];
        env.define(p.name, args[i], true);
      }

      return this.executeBlock(fnNode.body, env);
    }

    executeBlock(blockNode, env) {
      for (const stmt of blockNode.statements) {
        const res = this.executeStatement(stmt, env);
        if (res && res.__isReturn) {
          return res.value;
        }
      }
      return null;
    }

    executeStatement(stmt, env) {
      switch (stmt.type) {
        case 'Let': {
          const val = stmt.init ? this.evalExpr(stmt.init, env) : null;
          env.define(stmt.name, val, stmt.isMut);
          return null;
        }
        case 'Assign': {
          const val = this.evalExpr(stmt.value, env);
          if (stmt.target.type === 'Identifier') {
            env.assign(stmt.target.name, val);
          } else {
            throw new Error('Invalid assignment target');
          }
          return null;
        }
        case 'AssignOp': {
          const val = this.evalExpr(stmt.value, env);
          if (stmt.target.type === 'Identifier') {
            const current = env.get(stmt.target.name);
            let next;
            if (stmt.op === '+=') next = current + val;
            else if (stmt.op === '-=') next = current - val;
            env.assign(stmt.target.name, next);
          }
          return null;
        }
        case 'Return': {
          const val = stmt.value ? this.evalExpr(stmt.value, env) : null;
          return { __isReturn: true, value: val };
        }
        case 'If': {
          const cond = this.evalExpr(stmt.condition, env);
          if (cond) {
            return this.executeBlock(stmt.thenBranch, new Environment(env));
          } else if (stmt.elseBranch) {
            if (stmt.elseBranch.type === 'If') {
              return this.executeStatement(stmt.elseBranch, env);
            } else {
              return this.executeBlock(stmt.elseBranch, new Environment(env));
            }
          }
          return null;
        }
        case 'While': {
          let guard = 0;
          while (this.evalExpr(stmt.condition, env)) {
            guard++;
            if (guard > 100000) throw new Error('Maximum loop iteration exceeded (potential infinite loop).');
            const res = this.executeBlock(stmt.body, new Environment(env));
            if (res && res.__isReturn) return res;
          }
          return null;
        }
        case 'ExprStmt': {
          this.evalExpr(stmt.expr, env);
          return null;
        }
        default:
          return null;
      }
    }

    evalExpr(expr, env) {
      if (!expr) return null;
      switch (expr.type) {
        case 'Literal':
          return expr.value;
        case 'Identifier':
          return env.get(expr.name);
        case 'Binary': {
          const left = this.evalExpr(expr.left, env);
          const right = this.evalExpr(expr.right, env);
          switch (expr.op) {
            case '+': return left + right;
            case '-': return left - right;
            case '*': return left * right;
            case '/': return Math.floor(left / right);
            case '%': return left % right;
            case '==': return left === right;
            case '!=': return left !== right;
            case '<': return left < right;
            case '<=': return left <= right;
            case '>': return left > right;
            case '>=': return left >= right;
            case '&&': return !!(left && right);
            case '||': return !!(left || right);
            default: throw new Error(`Unknown binary operator ${expr.op}`);
          }
        }
        case 'Unary': {
          const val = this.evalExpr(expr.expr, env);
          if (expr.op === '!') return !val;
          if (expr.op === '-') return -val;
          return val;
        }
        case 'Call': {
          if (expr.callee.type === 'Identifier') {
            const name = expr.callee.name;
            if (this.functions.has(name)) {
              const fnNode = this.functions.get(name);
              const args = expr.args.map((a) => this.evalExpr(a, env));
              return this.executeFunction(fnNode, args, env);
            }
            // Check builtin
            const maybeBuiltin = env.get(name);
            if (typeof maybeBuiltin === 'function') {
              const args = expr.args.map((a) => this.evalExpr(a, env));
              return maybeBuiltin(...args);
            }
            throw new Error(`Function '${name}' is not defined.`);
          }
          throw new Error('Unsupported callee expression');
        }
        default:
          return null;
      }
    }
  }

  // --- Public API ---
  window.PradyEngine = {
    compileAndRun: function (sourceCode, printCallback) {
      const startTime = performance.now();
      const lexer = new Lexer(sourceCode);
      const lexResult = lexer.tokenize();

      const lines = sourceCode.split('\n');
      const parser = new Parser(lexResult.tokens, lines);
      const ast = parser.parseProgram();

      const diagnostics = [...lexResult.errors.map(e => ({ level: 'error', ...e })), ...ast.diagnostics];

      if (diagnostics.length > 0) {
        const errorReports = diagnostics.map(d => {
          const lineText = lines[d.line - 1] || '';
          const caret = ' '.repeat(Math.max(0, d.col - 1)) + '^';
          return `error: ${d.message}\n  --> line ${d.line}:${d.col}\n   |\n${d.line} | ${lineText}\n   | ${caret}`;
        }).join('\n\n');

        return {
          success: false,
          tokens: lexResult.tokens,
          ast: ast,
          diagnostics,
          errorText: errorReports,
          durationMs: (performance.now() - startTime).toFixed(2),
        };
      }

      // Check for Architecture contracts
      let archReports = [];
      const archItem = ast.items.find(i => i.type === 'Architecture');
      if (archItem) {
        archReports.push(`[Architecture Validator] System '${archItem.name}' loaded.`);
        archReports.push(`  Defined layers: ${archItem.layers.join(' -> ')}`);
        for (const r of archItem.rules) {
          if (r.type === 'allow') archReports.push(`  ✓ Policy: ${r.from} allowed to depend on ${r.to}`);
          if (r.type === 'deny') archReports.push(`  ✓ Policy: ${r.from} strictly forbidden to import ${r.to}`);
        }
      }

      let runError = null;
      try {
        const evaluator = new Evaluator(ast, printCallback);
        evaluator.evaluate();
      } catch (err) {
        runError = err.message;
      }

      return {
        success: !runError,
        tokens: lexResult.tokens,
        ast: ast,
        diagnostics: [],
        architecture: archItem,
        archReports,
        runError,
        durationMs: (performance.now() - startTime).toFixed(2),
      };
    },
  };
})(window);
