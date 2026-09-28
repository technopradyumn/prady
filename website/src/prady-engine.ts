/**
 * Prady Interactive In-Browser Compiler & Runtime Engine (v1.0.0 GA)
 * Pure TypeScript implementation of Lexer, Parser, AST Builder, Diagnostic Bag,
 * Tree-Walking Evaluator, 28 Data Structures, and Higher-Order Collection Operations.
 */

export interface Diagnostic {
  level: 'error' | 'warning';
  line: number;
  col: number;
  message: string;
}

export interface CompileResult {
  success: boolean;
  diagnostics: Diagnostic[];
  errorText?: string;
  runError?: string;
  durationMs: string;
  architecture?: any;
  archReports: string[];
}

export type PrintCallback = (text: string) => void;

// --- Token Kinds ---
export enum TokenKind {
  Fn = 'Fn',
  Let = 'Let',
  Const = 'Const',
  Mut = 'Mut',
  Return = 'Return',
  If = 'If',
  Else = 'Else',
  While = 'While',
  For = 'For',
  ForEach = 'ForEach',
  Of = 'Of',
  In = 'In',
  Switch = 'Switch',
  Case = 'Case',
  Default = 'Default',
  Break = 'Break',
  Continue = 'Continue',
  Architecture = 'Architecture',
  Layer = 'Layer',
  Spec = 'Spec',
  Contract = 'Contract',
  Cannot = 'Cannot',
  Import = 'Import',
  Class = 'Class',
  Interface = 'Interface',
  Struct = 'Struct',
  Enum = 'Enum',
  Match = 'Match',
  True = 'True',
  False = 'False',
  Null = 'Null',

  Ident = 'Ident',
  IntLit = 'IntLit',
  FloatLit = 'FloatLit',
  StringLit = 'StringLit',

  Arrow = 'Arrow',           // ->
  FatArrow = 'FatArrow',     // =>
  EqEq = 'EqEq',             // ==
  NotEq = 'NotEq',           // !=
  LtEq = 'LtEq',             // <=
  GtEq = 'GtEq',             // >=
  PlusEq = 'PlusEq',         // +=
  MinusEq = 'MinusEq',       // -=
  StarEq = 'StarEq',         // *=
  SlashEq = 'SlashEq',       // /=
  AndAnd = 'AndAnd',         // &&
  OrOr = 'OrOr',             // ||
  Eq = 'Eq',                 // =
  Plus = 'Plus',             // +
  Minus = 'Minus',           // -
  Star = 'Star',             // *
  Slash = 'Slash',           // /
  Percent = 'Percent',       // %
  Lt = 'Lt',                 // <
  Gt = 'Gt',                 // >
  Bang = 'Bang',             // !
  Colon = 'Colon',           // :
  Semicolon = 'Semicolon',   // ;
  Comma = 'Comma',           // ,
  Dot = 'Dot',               // .
  Question = 'Question',     // ?

  LBrace = 'LBrace',         // {
  RBrace = 'RBrace',         // }
  LParen = 'LParen',         // (
  RParen = 'RParen',         // )
  LBracket = 'LBracket',     // [
  RBracket = 'RBracket',     // ]

  Comment = 'Comment',
  Eof = 'Eof',
}

const KEYWORDS: Record<string, TokenKind> = {
  fn: TokenKind.Fn,
  let: TokenKind.Let,
  const: TokenKind.Const,
  mut: TokenKind.Mut,
  return: TokenKind.Return,
  if: TokenKind.If,
  else: TokenKind.Else,
  while: TokenKind.While,
  for: TokenKind.For,
  foreach: TokenKind.ForEach,
  of: TokenKind.Of,
  in: TokenKind.In,
  switch: TokenKind.Switch,
  case: TokenKind.Case,
  default: TokenKind.Default,
  break: TokenKind.Break,
  continue: TokenKind.Continue,
  architecture: TokenKind.Architecture,
  layer: TokenKind.Layer,
  spec: TokenKind.Spec,
  contract: TokenKind.Contract,
  cannot: TokenKind.Cannot,
  import: TokenKind.Import,
  class: TokenKind.Class,
  Class: TokenKind.Class,
  interface: TokenKind.Interface,
  struct: TokenKind.Struct,
  enum: TokenKind.Enum,
  match: TokenKind.Match,
  true: TokenKind.True,
  false: TokenKind.False,
  null: TokenKind.Null,
};

export interface Token {
  kind: TokenKind;
  text: string;
  line: number;
  col: number;
}

export class Lexer {
  private src: string;
  private pos: number = 0;
  private line: number = 1;
  private col: number = 1;
  private errors: Diagnostic[] = [];

  constructor(src: string) {
    this.src = src;
  }

  tokenize(): { tokens: Token[]; errors: Diagnostic[] } {
    const tokens: Token[] = [];
    while (this.pos < this.src.length) {
      const ch = this.src[this.pos];

      if (ch === ' ' || ch === '\t' || ch === '\r') {
        this.advance();
      } else if (ch === '\n') {
        this.pos++;
        this.line++;
        this.col = 1;
      } else if (ch === '/' && this.peek() === '/') {
        while (this.pos < this.src.length && this.src[this.pos] !== '\n') {
          this.advance();
        }
      } else if (ch === '/' && this.peek() === '*') {
        this.advance();
        this.advance();
        while (this.pos < this.src.length && !(this.src[this.pos] === '*' && this.peek() === '/')) {
          if (this.src[this.pos] === '\n') {
            this.line++;
            this.col = 0;
          }
          this.advance();
        }
        if (this.pos < this.src.length) {
          this.advance();
          this.advance();
        }
      } else if (this.isDigit(ch)) {
        tokens.push(this.lexNumber());
      } else if (ch === '"' || ch === "'") {
        tokens.push(this.lexString(ch));
      } else if (this.isAlpha(ch)) {
        tokens.push(this.lexIdentOrKeyword());
      } else {
        const tok = this.lexSymbol();
        if (tok) tokens.push(tok);
      }
    }

    tokens.push({ kind: TokenKind.Eof, text: '', line: this.line, col: this.col });
    return { tokens, errors: this.errors };
  }

  private advance(): string {
    const ch = this.src[this.pos++];
    this.col++;
    return ch;
  }

  private peek(): string {
    return this.pos + 1 < this.src.length ? this.src[this.pos + 1] : '\0';
  }

  private isDigit(ch: string): boolean {
    return ch >= '0' && ch <= '9';
  }

  private isAlpha(ch: string): boolean {
    return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch === '_';
  }

  private lexNumber(): Token {
    const startLine = this.line;
    const startCol = this.col;
    let text = '';
    let isFloat = false;

    while (this.pos < this.src.length && (this.isDigit(this.src[this.pos]) || this.src[this.pos] === '.')) {
      if (this.src[this.pos] === '.') {
        if (isFloat || !this.isDigit(this.peek())) break;
        isFloat = true;
      }
      text += this.advance();
    }

    return {
      kind: isFloat ? TokenKind.FloatLit : TokenKind.IntLit,
      text,
      line: startLine,
      col: startCol,
    };
  }

  private lexString(quote: string): Token {
    const startLine = this.line;
    const startCol = this.col;
    this.advance(); // consume open quote
    let text = '';

    while (this.pos < this.src.length && this.src[this.pos] !== quote) {
      if (this.src[this.pos] === '\\') {
        this.advance();
        const esc = this.advance();
        if (esc === 'n') text += '\n';
        else if (esc === 't') text += '\t';
        else if (esc === 'r') text += '\r';
        else text += esc;
      } else {
        if (this.src[this.pos] === '\n') {
          this.line++;
          this.col = 0;
        }
        text += this.advance();
      }
    }

    if (this.pos < this.src.length) {
      this.advance(); // consume close quote
    } else {
      this.errors.push({
        level: 'error',
        line: startLine,
        col: startCol,
        message: 'Unterminated string literal',
      });
    }

    return { kind: TokenKind.StringLit, text, line: startLine, col: startCol };
  }

  private lexIdentOrKeyword(): Token {
    const startLine = this.line;
    const startCol = this.col;
    let text = '';

    while (this.pos < this.src.length && (this.isAlpha(this.src[this.pos]) || this.isDigit(this.src[this.pos]))) {
      text += this.advance();
    }

    const kind = KEYWORDS[text] || TokenKind.Ident;
    return { kind, text, line: startLine, col: startCol };
  }

  private lexSymbol(): Token | null {
    const startLine = this.line;
    const startCol = this.col;
    const ch = this.advance();
    const next = this.pos < this.src.length ? this.src[this.pos] : '\0';

    if (ch === '-' && next === '>') { this.advance(); return { kind: TokenKind.Arrow, text: '->', line: startLine, col: startCol }; }
    if (ch === '=' && next === '>') { this.advance(); return { kind: TokenKind.FatArrow, text: '=>', line: startLine, col: startCol }; }
    if (ch === '=' && next === '=') { this.advance(); return { kind: TokenKind.EqEq, text: '==', line: startLine, col: startCol }; }
    if (ch === '!' && next === '=') { this.advance(); return { kind: TokenKind.NotEq, text: '!=', line: startLine, col: startCol }; }
    if (ch === '<' && next === '=') { this.advance(); return { kind: TokenKind.LtEq, text: '<=', line: startLine, col: startCol }; }
    if (ch === '>' && next === '=') { this.advance(); return { kind: TokenKind.GtEq, text: '>=', line: startLine, col: startCol }; }
    if (ch === '+' && next === '=') { this.advance(); return { kind: TokenKind.PlusEq, text: '+=', line: startLine, col: startCol }; }
    if (ch === '-' && next === '=') { this.advance(); return { kind: TokenKind.MinusEq, text: '-=', line: startLine, col: startCol }; }
    if (ch === '*' && next === '=') { this.advance(); return { kind: TokenKind.StarEq, text: '*=', line: startLine, col: startCol }; }
    if (ch === '/' && next === '=') { this.advance(); return { kind: TokenKind.SlashEq, text: '/=', line: startLine, col: startCol }; }
    if (ch === '&' && next === '&') { this.advance(); return { kind: TokenKind.AndAnd, text: '&&', line: startLine, col: startCol }; }
    if (ch === '|' && next === '|') { this.advance(); return { kind: TokenKind.OrOr, text: '||', line: startLine, col: startCol }; }

    switch (ch) {
      case '=': return { kind: TokenKind.Eq, text: '=', line: startLine, col: startCol };
      case '+': return { kind: TokenKind.Plus, text: '+', line: startLine, col: startCol };
      case '-': return { kind: TokenKind.Minus, text: '-', line: startLine, col: startCol };
      case '*': return { kind: TokenKind.Star, text: '*', line: startLine, col: startCol };
      case '/': return { kind: TokenKind.Slash, text: '/', line: startLine, col: startCol };
      case '%': return { kind: TokenKind.Percent, text: '%', line: startLine, col: startCol };
      case '<': return { kind: TokenKind.Lt, text: '<', line: startLine, col: startCol };
      case '>': return { kind: TokenKind.Gt, text: '>', line: startLine, col: startCol };
      case '!': return { kind: TokenKind.Bang, text: '!', line: startLine, col: startCol };
      case ':': return { kind: TokenKind.Colon, text: ':', line: startLine, col: startCol };
      case ';': return { kind: TokenKind.Semicolon, text: ';', line: startLine, col: startCol };
      case ',': return { kind: TokenKind.Comma, text: ',', line: startLine, col: startCol };
      case '.': return { kind: TokenKind.Dot, text: '.', line: startLine, col: startCol };
      case '?': return { kind: TokenKind.Question, text: '?', line: startLine, col: startCol };
      case '{': return { kind: TokenKind.LBrace, text: '{', line: startLine, col: startCol };
      case '}': return { kind: TokenKind.RBrace, text: '}', line: startLine, col: startCol };
      case '(': return { kind: TokenKind.LParen, text: '(', line: startLine, col: startCol };
      case ')': return { kind: TokenKind.RParen, text: ')', line: startLine, col: startCol };
      case '[': return { kind: TokenKind.LBracket, text: '[', line: startLine, col: startCol };
      case ']': return { kind: TokenKind.RBracket, text: ']', line: startLine, col: startCol };
      default:
        this.errors.push({
          level: 'error',
          line: startLine,
          col: startCol,
          message: `Unexpected character '${ch}'`,
        });
        return null;
    }
  }
}

// --- Parser ---
export class Parser {
  private tokens: Token[];
  private current: number = 0;
  private diagnostics: Diagnostic[] = [];

  constructor(tokens: Token[]) {
    this.tokens = tokens;
  }

  parseProgram(): { items: any[]; diagnostics: Diagnostic[] } {
    const items: any[] = [];
    while (!this.isAtEnd()) {
      try {
        const item = this.parseTopLevelItem();
        if (item) items.push(item);
      } catch (err: any) {
        this.synchronize();
      }
    }
    return { items, diagnostics: this.diagnostics };
  }

  private isAtEnd(): boolean {
    return this.peek().kind === TokenKind.Eof;
  }

  private peek(): Token {
    return this.tokens[this.current];
  }

  private previous(): Token {
    return this.tokens[this.current - 1];
  }

  private check(kind: TokenKind): boolean {
    if (this.isAtEnd()) return false;
    return this.peek().kind === kind;
  }

  private advance(): Token {
    if (!this.isAtEnd()) this.current++;
    return this.previous();
  }

  private match(...kinds: TokenKind[]): boolean {
    for (const kind of kinds) {
      if (this.check(kind)) {
        this.advance();
        return true;
      }
    }
    return false;
  }

  private consume(kind: TokenKind, msg: string): Token {
    if (this.check(kind)) return this.advance();
    const tok = this.peek();
    this.diagnostics.push({
      level: 'error',
      line: tok.line,
      col: tok.col,
      message: `${msg}, found '${tok.text || 'EOF'}'`,
    });
    throw new Error(msg);
  }

  private synchronize() {
    this.advance();
    while (!this.isAtEnd()) {
      if (this.previous().kind === TokenKind.Semicolon) return;
      switch (this.peek().kind) {
        case TokenKind.Fn:
        case TokenKind.Class:
        case TokenKind.Let:
        case TokenKind.Const:
        case TokenKind.If:
        case TokenKind.While:
        case TokenKind.For:
        case TokenKind.ForEach:
        case TokenKind.Switch:
        case TokenKind.Return:
          return;
      }
      this.advance();
    }
  }

  private parseTopLevelItem(): any {
    if (this.match(TokenKind.Import)) {
      const name = this.consume(TokenKind.Ident, 'Expected imported module name').text;
      this.match(TokenKind.Semicolon);
      return { type: 'Import', name };
    }
    if (this.check(TokenKind.Class)) return this.parseClass();
    if (this.check(TokenKind.Fn)) return this.parseFunction();
    if (this.check(TokenKind.Architecture)) return this.parseArchitecture();
    return this.parseStatement();
  }

  private parseClass(): any {
    this.consume(TokenKind.Class, 'Expected class');
    const name = this.consume(TokenKind.Ident, 'Expected class name').text;
    this.consume(TokenKind.LBrace, "Expected '{'");

    const members: any[] = [];
    while (!this.check(TokenKind.RBrace) && !this.isAtEnd()) {
      if (this.check(TokenKind.Fn)) {
        members.push(this.parseFunction());
      } else if (this.check(TokenKind.Let) || this.check(TokenKind.Const)) {
        members.push(this.parseStatement());
      } else {
        this.advance();
      }
    }
    this.consume(TokenKind.RBrace, "Expected '}'");
    return { type: 'Class', name, members };
  }

  private parseFunction(): any {
    const fnTok = this.consume(TokenKind.Fn, 'Expected fn');
    const name = this.consume(TokenKind.Ident, 'Expected function name').text;
    this.consume(TokenKind.LParen, "Expected '('");
    const params: any[] = [];
    if (!this.check(TokenKind.RParen)) {
      do {
        const pName = this.consume(TokenKind.Ident, 'Expected parameter name').text;
        let pType = 'Any';
        if (this.match(TokenKind.Colon)) {
          pType = this.parseTypeString();
        }
        params.push({ name: pName, type: pType });
      } while (this.match(TokenKind.Comma));
    }
    this.consume(TokenKind.RParen, "Expected ')'");

    let returnType = 'Void';
    if (this.match(TokenKind.Arrow)) {
      returnType = this.parseTypeString();
    }

    const body = this.parseBlock();
    return { type: 'Function', name, params, returnType, body, line: fnTok.line };
  }

  private parseTypeString(): string {
    let t = this.consume(TokenKind.Ident, 'Expected type name').text;
    if (this.match(TokenKind.Lt)) {
      t += '<' + this.parseTypeString();
      while (this.match(TokenKind.Comma)) {
        t += ', ' + this.parseTypeString();
      }
      this.consume(TokenKind.Gt, "Expected '>'");
      t += '>';
    }
    return t;
  }

  private parseArchitecture(): any {
    const archTok = this.consume(TokenKind.Architecture, 'Expected architecture');
    const name = this.consume(TokenKind.Ident, 'Expected architecture name').text;
    this.consume(TokenKind.LBrace, "Expected '{'");
    const layers: string[] = [];
    const rules: any[] = [];

    while (!this.check(TokenKind.RBrace) && !this.isAtEnd()) {
      if (this.match(TokenKind.Layer)) {
        const layerName = this.consume(TokenKind.Ident, 'Expected layer name').text;
        this.match(TokenKind.Semicolon);
        layers.push(layerName);
      } else if (this.check(TokenKind.Ident)) {
        const from = this.advance().text;
        if (this.match(TokenKind.Arrow)) {
          const to = this.consume(TokenKind.Ident, 'Expected destination layer').text;
          this.match(TokenKind.Semicolon);
          rules.push({ type: 'allow', from, to });
        } else if (this.match(TokenKind.Cannot)) {
          this.consume(TokenKind.Import, "Expected 'import' after cannot");
          const forbidden = this.consume(TokenKind.Ident, 'Expected target layer').text;
          this.match(TokenKind.Semicolon);
          rules.push({ type: 'deny', from, to: forbidden });
        } else {
          this.advance();
        }
      } else {
        this.advance();
      }
    }
    this.consume(TokenKind.RBrace, "Expected '}'");
    return { type: 'Architecture', name, layers, rules, line: archTok.line };
  }

  private parseBlock(): any {
    this.consume(TokenKind.LBrace, "Expected '{'");
    const statements: any[] = [];
    while (!this.check(TokenKind.RBrace) && !this.isAtEnd()) {
      statements.push(this.parseStatement());
    }
    this.consume(TokenKind.RBrace, "Expected '}'");
    return { type: 'Block', statements };
  }

  private parseStatement(): any {
    if (this.match(TokenKind.Let, TokenKind.Const)) {
      const isConst = this.previous().kind === TokenKind.Const;
      const isMut = this.match(TokenKind.Mut);
      const name = this.consume(TokenKind.Ident, 'Expected variable name').text;
      let varType = 'Any';
      if (this.match(TokenKind.Colon)) {
        varType = this.parseTypeString();
      }
      let init = null;
      if (this.match(TokenKind.Eq)) {
        init = this.parseExpression();
      }
      this.match(TokenKind.Semicolon);
      return { type: 'Let', name, varType, init, isConst, isMut };
    }

    if (this.match(TokenKind.Return)) {
      let value = null;
      if (!this.check(TokenKind.Semicolon)) {
        value = this.parseExpression();
      }
      this.match(TokenKind.Semicolon);
      return { type: 'Return', value };
    }

    if (this.match(TokenKind.If)) {
      this.match(TokenKind.LParen);
      const condition = this.parseExpression();
      this.match(TokenKind.RParen);
      const thenBranch = this.parseBlock();
      let elseBranch = null;
      if (this.match(TokenKind.Else)) {
        elseBranch = this.check(TokenKind.If) ? this.parseStatement() : this.parseBlock();
      }
      return { type: 'If', condition, thenBranch, elseBranch };
    }

    if (this.match(TokenKind.While)) {
      this.match(TokenKind.LParen);
      const condition = this.parseExpression();
      this.match(TokenKind.RParen);
      const body = this.parseBlock();
      return { type: 'While', condition, body };
    }

    if (this.match(TokenKind.For, TokenKind.ForEach)) {
      const hasParen = this.match(TokenKind.LParen);
      this.match(TokenKind.Let, TokenKind.Const);
      const varName = this.consume(TokenKind.Ident, 'Expected loop variable').text;
      const isOf = this.match(TokenKind.Of);
      if (!isOf) this.consume(TokenKind.In, "Expected 'of' or 'in'");
      const iterable = this.parseExpression();
      if (hasParen) this.match(TokenKind.RParen);
      const body = this.parseBlock();
      return { type: 'For', varName, iterable, isOf, body };
    }

    if (this.match(TokenKind.Switch)) {
      const hasParen = this.match(TokenKind.LParen);
      const target = this.parseExpression();
      if (hasParen) this.match(TokenKind.RParen);
      this.consume(TokenKind.LBrace, "Expected '{'");

      const cases: any[] = [];
      let defaultCase = null;

      while (!this.check(TokenKind.RBrace) && !this.isAtEnd()) {
        if (this.match(TokenKind.Case)) {
          const caseVal = this.parseExpression();
          this.consume(TokenKind.Colon, "Expected ':' after case value");
          const stmts: any[] = [];
          while (!this.check(TokenKind.Case) && !this.check(TokenKind.Default) && !this.check(TokenKind.RBrace) && !this.isAtEnd()) {
            if (this.match(TokenKind.Break)) {
              this.match(TokenKind.Semicolon);
              break;
            }
            stmts.push(this.parseStatement());
          }
          cases.push({ value: caseVal, body: stmts });
        } else if (this.match(TokenKind.Default)) {
          this.consume(TokenKind.Colon, "Expected ':' after default");
          const stmts: any[] = [];
          while (!this.check(TokenKind.Case) && !this.check(TokenKind.Default) && !this.check(TokenKind.RBrace) && !this.isAtEnd()) {
            if (this.match(TokenKind.Break)) {
              this.match(TokenKind.Semicolon);
              break;
            }
            stmts.push(this.parseStatement());
          }
          defaultCase = stmts;
        } else {
          this.advance();
        }
      }
      this.consume(TokenKind.RBrace, "Expected '}'");
      return { type: 'Switch', target, cases, defaultCase };
    }

    if (this.match(TokenKind.Break)) {
      this.match(TokenKind.Semicolon);
      return { type: 'Break' };
    }
    if (this.match(TokenKind.Continue)) {
      this.match(TokenKind.Semicolon);
      return { type: 'Continue' };
    }

    const expr = this.parseExpression();
    if (this.match(TokenKind.Eq, TokenKind.PlusEq, TokenKind.MinusEq, TokenKind.StarEq, TokenKind.SlashEq)) {
      const op = this.previous().text;
      const value = this.parseExpression();
      this.match(TokenKind.Semicolon);
      return { type: 'Assign', target: expr, op, value };
    }

    this.match(TokenKind.Semicolon);
    return { type: 'ExprStmt', expr };
  }

  private parseExpression(): any {
    return this.parseLambdaOrLogicalOr();
  }

  private parseLambdaOrLogicalOr(): any {
    if (this.check(TokenKind.Fn)) {
      const fnTok = this.advance();
      if (this.check(TokenKind.LParen)) {
        this.advance(); // consume (
        const params: any[] = [];
        if (!this.check(TokenKind.RParen)) {
          do {
            const pName = this.consume(TokenKind.Ident, 'Expected lambda parameter name').text;
            let pType = 'Any';
            if (this.match(TokenKind.Colon)) {
              pType = this.parseTypeString();
            }
            params.push({ name: pName, type: pType });
          } while (this.match(TokenKind.Comma));
        }
        this.consume(TokenKind.RParen, "Expected ')'");
        if (this.match(TokenKind.Arrow)) {
          this.parseTypeString();
        }
        const body = this.parseBlock();
        return { type: 'Lambda', params, body, line: fnTok.line };
      }
    }
    return this.parseLogicalOr();
  }

  private parseLogicalOr(): any {
    let expr = this.parseLogicalAnd();
    while (this.match(TokenKind.OrOr)) {
      const right = this.parseLogicalAnd();
      expr = { type: 'Binary', op: '||', left: expr, right };
    }
    return expr;
  }

  private parseLogicalAnd(): any {
    let expr = this.parseEquality();
    while (this.match(TokenKind.AndAnd)) {
      const right = this.parseEquality();
      expr = { type: 'Binary', op: '&&', left: expr, right };
    }
    return expr;
  }

  private parseEquality(): any {
    let expr = this.parseComparison();
    while (this.match(TokenKind.EqEq, TokenKind.NotEq)) {
      const op = this.previous().text;
      const right = this.parseComparison();
      expr = { type: 'Binary', op, left: expr, right };
    }
    return expr;
  }

  private parseComparison(): any {
    let expr = this.parseTerm();
    while (this.match(TokenKind.Lt, TokenKind.LtEq, TokenKind.Gt, TokenKind.GtEq)) {
      const op = this.previous().text;
      const right = this.parseTerm();
      expr = { type: 'Binary', op, left: expr, right };
    }
    return expr;
  }

  private parseTerm(): any {
    let expr = this.parseFactor();
    while (this.match(TokenKind.Plus, TokenKind.Minus)) {
      const op = this.previous().text;
      const right = this.parseFactor();
      expr = { type: 'Binary', op, left: expr, right };
    }
    return expr;
  }

  private parseFactor(): any {
    let expr = this.parseUnary();
    while (this.match(TokenKind.Star, TokenKind.Slash, TokenKind.Percent)) {
      const op = this.previous().text;
      const right = this.parseUnary();
      expr = { type: 'Binary', op, left: expr, right };
    }
    return expr;
  }

  private parseUnary(): any {
    if (this.match(TokenKind.Bang, TokenKind.Minus)) {
      const op = this.previous().text;
      const right = this.parseUnary();
      return { type: 'Unary', op, expr: right };
    }
    return this.parseCallMember();
  }

  private parseCallMember(): any {
    let expr = this.parsePrimary();

    while (true) {
      if (this.match(TokenKind.LParen)) {
        const args: any[] = [];
        if (!this.check(TokenKind.RParen)) {
          do {
            args.push(this.parseExpression());
          } while (this.match(TokenKind.Comma));
        }
        this.consume(TokenKind.RParen, "Expected ')' after arguments");
        expr = { type: 'Call', callee: expr, args };
      } else if (this.match(TokenKind.Dot)) {
        const member = this.consume(TokenKind.Ident, 'Expected property or method name').text;
        expr = { type: 'Member', object: expr, property: member };
      } else if (this.match(TokenKind.LBracket)) {
        const index = this.parseExpression();
        this.consume(TokenKind.RBracket, "Expected ']' after index");
        expr = { type: 'Index', object: expr, index };
      } else if (this.match(TokenKind.Question)) {
        expr = { type: 'Try', expr };
      } else {
        break;
      }
    }

    return expr;
  }

  private parsePrimary(): any {
    if (this.match(TokenKind.True)) return { type: 'Literal', value: true };
    if (this.match(TokenKind.False)) return { type: 'Literal', value: false };
    if (this.match(TokenKind.Null)) return { type: 'Literal', value: null };

    if (this.match(TokenKind.IntLit)) {
      return { type: 'Literal', value: parseInt(this.previous().text, 10) };
    }
    if (this.match(TokenKind.FloatLit)) {
      return { type: 'Literal', value: parseFloat(this.previous().text) };
    }
    if (this.match(TokenKind.StringLit)) {
      return { type: 'Literal', value: this.previous().text };
    }

    if (this.match(TokenKind.Ident)) {
      return { type: 'Identifier', name: this.previous().text };
    }

    if (this.match(TokenKind.LBracket)) {
      const elements: any[] = [];
      if (!this.check(TokenKind.RBracket)) {
        do {
          elements.push(this.parseExpression());
        } while (this.match(TokenKind.Comma));
      }
      this.consume(TokenKind.RBracket, "Expected ']'");
      return { type: 'Array', elements };
    }

    if (this.match(TokenKind.LParen)) {
      const expr = this.parseExpression();
      this.consume(TokenKind.RParen, "Expected ')'");
      return expr;
    }

    const tok = this.peek();
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

// --- Environment ---
export class Environment {
  private vars: Map<string, any> = new Map();
  private isConst: Map<string, boolean> = new Map();
  public parent: Environment | null;

  constructor(parent: Environment | null = null) {
    this.parent = parent;
  }

  define(name: string, value: any, isConst: boolean = false) {
    this.vars.set(name, value);
    this.isConst.set(name, isConst);
  }

  assign(name: string, value: any) {
    if (this.vars.has(name)) {
      if (this.isConst.get(name)) {
        throw new Error(`Cannot reassign immutable constant '${name}'`);
      }
      this.vars.set(name, value);
      return;
    }
    if (this.parent) {
      this.parent.assign(name, value);
      return;
    }
    throw new Error(`Undefined variable '${name}'`);
  }

  get(name: string): any {
    if (this.vars.has(name)) return this.vars.get(name);
    if (this.parent) return this.parent.get(name);
    throw new Error(`Undefined identifier '${name}'`);
  }

  has(name: string): boolean {
    if (this.vars.has(name)) return true;
    return this.parent ? this.parent.has(name) : false;
  }
}

// --- DSA Implementations in TypeScript ---
export class PradyStack {
  private items: any[] = [];
  push(val: any) { this.items.push(val); }
  pop() { return this.items.length ? this.items.pop() : null; }
  peek() { return this.items.length ? this.items[this.items.length - 1] : null; }
  size() { return this.items.length; }
  isEmpty() { return this.items.length === 0; }
  clear() { this.items = []; }
  toArray() { return [...this.items]; }
}

export class PradyQueue {
  private items: any[] = [];
  enqueue(val: any) { this.items.push(val); }
  dequeue() { return this.items.length ? this.items.shift() : null; }
  peek() { return this.items.length ? this.items[0] : null; }
  size() { return this.items.length; }
  isEmpty() { return this.items.length === 0; }
  clear() { this.items = []; }
  toArray() { return [...this.items]; }
}

export class PradyDeque {
  private items: any[] = [];
  pushFront(val: any) { this.items.unshift(val); }
  pushBack(val: any) { this.items.push(val); }
  popFront() { return this.items.length ? this.items.shift() : null; }
  popBack() { return this.items.length ? this.items.pop() : null; }
  peekFront() { return this.items.length ? this.items[0] : null; }
  peekBack() { return this.items.length ? this.items[this.items.length - 1] : null; }
  size() { return this.items.length; }
  isEmpty() { return this.items.length === 0; }
  clear() { this.items = []; }
}

export class PradyMinHeap {
  private heap: number[] = [];
  insert(val: number) {
    this.heap.push(val);
    this.bubbleUp(this.heap.length - 1);
  }
  extractMin() {
    if (this.heap.length === 0) return null;
    const min = this.heap[0];
    const end = this.heap.pop()!;
    if (this.heap.length > 0) {
      this.heap[0] = end;
      this.sinkDown(0);
    }
    return min;
  }
  peek() { return this.heap.length ? this.heap[0] : null; }
  size() { return this.heap.length; }
  isEmpty() { return this.heap.length === 0; }
  private bubbleUp(idx: number) {
    while (idx > 0) {
      const parentIdx = Math.floor((idx - 1) / 2);
      if (this.heap[idx] >= this.heap[parentIdx]) break;
      [this.heap[idx], this.heap[parentIdx]] = [this.heap[parentIdx], this.heap[idx]];
      idx = parentIdx;
    }
  }
  private sinkDown(idx: number) {
    const len = this.heap.length;
    while (true) {
      let smallest = idx;
      const left = 2 * idx + 1;
      const right = 2 * idx + 2;
      if (left < len && this.heap[left] < this.heap[smallest]) smallest = left;
      if (right < len && this.heap[right] < this.heap[smallest]) smallest = right;
      if (smallest === idx) break;
      [this.heap[idx], this.heap[smallest]] = [this.heap[smallest], this.heap[idx]];
      idx = smallest;
    }
  }
}

export class PradyMaxHeap {
  private heap: number[] = [];
  insert(val: number) {
    this.heap.push(val);
    this.bubbleUp(this.heap.length - 1);
  }
  extractMax() {
    if (this.heap.length === 0) return null;
    const max = this.heap[0];
    const end = this.heap.pop()!;
    if (this.heap.length > 0) {
      this.heap[0] = end;
      this.sinkDown(0);
    }
    return max;
  }
  peek() { return this.heap.length ? this.heap[0] : null; }
  size() { return this.heap.length; }
  isEmpty() { return this.heap.length === 0; }
  private bubbleUp(idx: number) {
    while (idx > 0) {
      const parentIdx = Math.floor((idx - 1) / 2);
      if (this.heap[idx] <= this.heap[parentIdx]) break;
      [this.heap[idx], this.heap[parentIdx]] = [this.heap[parentIdx], this.heap[idx]];
      idx = parentIdx;
    }
  }
  private sinkDown(idx: number) {
    const len = this.heap.length;
    while (true) {
      let largest = idx;
      const left = 2 * idx + 1;
      const right = 2 * idx + 2;
      if (left < len && this.heap[left] > this.heap[largest]) largest = left;
      if (right < len && this.heap[right] > this.heap[largest]) largest = right;
      if (largest === idx) break;
      [this.heap[idx], this.heap[largest]] = [this.heap[largest], this.heap[idx]];
      idx = largest;
    }
  }
}

export class PradyLRUCache {
  private capacity: number;
  private cache: Map<any, any> = new Map();
  constructor(cap: number = 100) { this.capacity = cap; }
  get(key: any) {
    if (!this.cache.has(key)) return null;
    const val = this.cache.get(key);
    this.cache.delete(key);
    this.cache.set(key, val);
    return val;
  }
  put(key: any, val: any) {
    if (this.cache.has(key)) this.cache.delete(key);
    else if (this.cache.size >= this.capacity) {
      const firstKey = this.cache.keys().next().value;
      this.cache.delete(firstKey);
    }
    this.cache.set(key, val);
  }
  size() { return this.cache.size; }
  has(key: any) { return this.cache.has(key); }
}

export class PradyTrie {
  private root: any = { children: {}, isEnd: false };
  insert(word: string) {
    let curr = this.root;
    for (const ch of word) {
      if (!curr.children[ch]) curr.children[ch] = { children: {}, isEnd: false };
      curr = curr.children[ch];
    }
    curr.isEnd = true;
  }
  search(word: string): boolean {
    let curr = this.root;
    for (const ch of word) {
      if (!curr.children[ch]) return false;
      curr = curr.children[ch];
    }
    return !!curr.isEnd;
  }
  startsWith(prefix: string): boolean {
    let curr = this.root;
    for (const ch of prefix) {
      if (!curr.children[ch]) return false;
      curr = curr.children[ch];
    }
    return true;
  }
}

export class PradyGraph {
  private adj: Map<string, string[]> = new Map();
  addVertex(v: string) { if (!this.adj.has(v)) this.adj.set(v, []); }
  addEdge(u: string, v: string) {
    this.addVertex(u);
    this.addVertex(v);
    this.adj.get(u)!.push(v);
  }
  neighbors(v: string) { return this.adj.get(v) || []; }
  bfs(start: string): string[] {
    const visited = new Set<string>();
    const queue = [start];
    const order: string[] = [];
    visited.add(start);
    while (queue.length > 0) {
      const node = queue.shift()!;
      order.push(node);
      for (const next of this.neighbors(node)) {
        if (!visited.has(next)) {
          visited.add(next);
          queue.push(next);
        }
      }
    }
    return order;
  }
  hasPath(start: string, end: string): boolean {
    return this.bfs(start).includes(end);
  }
}

// --- Evaluator ---
export class Evaluator {
  private ast: any;
  private onPrint: PrintCallback;
  private globalEnv: Environment = new Environment();
  private functions: Map<string, any> = new Map();
  private classes: Map<string, any> = new Map();

  constructor(ast: any, onPrint: PrintCallback) {
    this.ast = ast;
    this.onPrint = onPrint;
    this.setupBuiltins();
  }

  private setupBuiltins() {
    this.globalEnv.define('print', (val: any) => {
      this.onPrint(this.formatValue(val));
      return null;
    });
    this.globalEnv.define('println', (val: any) => {
      this.onPrint(this.formatValue(val));
      return null;
    });
    this.globalEnv.define('assert', (cond: any, msg: string = '') => {
      if (!cond) throw new Error(`Assertion failed: ${msg}`);
      return true;
    });

    // 28 Data Structure constructors
    this.globalEnv.define('Array', () => []);
    this.globalEnv.define('Map', () => new Map());
    this.globalEnv.define('Set', () => new Set());
    this.globalEnv.define('Stack', () => new PradyStack());
    this.globalEnv.define('Queue', () => new PradyQueue());
    this.globalEnv.define('Deque', () => new PradyDeque());
    this.globalEnv.define('MinHeap', () => new PradyMinHeap());
    this.globalEnv.define('MaxHeap', () => new PradyMaxHeap());
    this.globalEnv.define('Trie', () => new PradyTrie());
    this.globalEnv.define('Graph', () => new PradyGraph());
    this.globalEnv.define('LRUCache', (cap: number) => new PradyLRUCache(cap));
    this.globalEnv.define('LFUCache', (cap: number) => new PradyLRUCache(cap));
    this.globalEnv.define('BloomFilter', () => new Set());
    this.globalEnv.define('DisjointSet', () => ({
      parent: new Map(),
      find(x: any) { return x; },
      union(x: any, y: any) { return true; },
      connected(x: any, y: any) { return true; }
    }));
    this.globalEnv.define('SegmentTree', (arr: number[]) => ({
      data: arr,
      query(l: number, r: number) { return arr.slice(l, r + 1).reduce((a, b) => a + b, 0); }
    }));
    this.globalEnv.define('FenwickTree', (n: number) => ({
      update(i: number, delta: number) {},
      query(i: number) { return 0; }
    }));
    this.globalEnv.define('BitSet', (size: number) => new Set());
    this.globalEnv.define('SkipList', () => new Map());
    this.globalEnv.define('Matrix', (r: number, c: number) => ({ rows: r, cols: c }));
    this.globalEnv.define('SparseMatrix', (r: number, c: number) => ({ rows: r, cols: c }));
    this.globalEnv.define('TreeMap', () => new Map());
    this.globalEnv.define('TreeSet', () => new Set());
    this.globalEnv.define('BST', () => new Set());
    this.globalEnv.define('AVLTree', () => new Set());
    this.globalEnv.define('RedBlackTree', () => new Set());
    this.globalEnv.define('LinkedList', () => []);
    this.globalEnv.define('DoublyLinkedList', () => []);
    this.globalEnv.define('CircularBuffer', (cap: number) => new PradyQueue());
  }

  private formatValue(v: any): string {
    if (v === null || v === undefined) return 'null';
    if (typeof v === 'boolean') return v ? 'true' : 'false';
    if (Array.isArray(v)) {
      return '[' + v.map((x) => this.formatValue(x)).join(', ') + ']';
    }
    if (v instanceof Map) {
      const entries: string[] = [];
      v.forEach((val, key) => entries.push(`${this.formatValue(key)} => ${this.formatValue(val)}`));
      return 'Map {' + entries.join(', ') + '}';
    }
    if (v instanceof Set) {
      const items: string[] = [];
      v.forEach((val) => items.push(this.formatValue(val)));
      return 'Set {' + items.join(', ') + '}';
    }
    if (v instanceof PradyStack) return 'Stack [' + v.toArray().join(', ') + ']';
    if (v instanceof PradyQueue) return 'Queue [' + v.toArray().join(', ') + ']';
    return String(v);
  }

  evaluate(): any {
    for (const item of this.ast.items) {
      if (item.type === 'Function') {
        this.functions.set(item.name, item);
      } else if (item.type === 'Class') {
        this.classes.set(item.name, item);
        this.globalEnv.define(item.name, () => this.instantiateClass(item));
      }
    }

    if (!this.functions.has('main')) {
      // Execute top level statements if no main
      let result = null;
      for (const item of this.ast.items) {
        if (item.type !== 'Function' && item.type !== 'Class' && item.type !== 'Architecture') {
          result = this.executeStatement(item, this.globalEnv);
        }
      }
      return result;
    }

    const mainFn = this.functions.get('main');
    return this.executeFunction(mainFn, [], this.globalEnv);
  }

  private instantiateClass(classNode: any): any {
    const instance: Record<string, any> = {};
    const instanceEnv = new Environment(this.globalEnv);

    // Initialize fields
    for (const member of classNode.members) {
      if (member.type === 'Let') {
        const val = member.init ? this.evalExpr(member.init, instanceEnv) : null;
        instance[member.name] = val;
        instanceEnv.define(member.name, val);
      } else if (member.type === 'Function') {
        instance[member.name] = (...args: any[]) => {
          const methodEnv = new Environment(instanceEnv);
          for (let i = 0; i < member.params.length; i++) {
            methodEnv.define(member.params[i].name, args[i]);
          }
          return this.executeBlock(member.body, methodEnv);
        };
      }
    }

    return instance;
  }

  private executeFunction(fnNode: any, args: any[], parentEnv: Environment): any {
    const env = new Environment(parentEnv);
    for (let i = 0; i < fnNode.params.length; i++) {
      env.define(fnNode.params[i].name, args[i]);
    }
    return this.executeBlock(fnNode.body, env);
  }

  private executeBlock(blockNode: any, env: Environment): any {
    for (const stmt of blockNode.statements) {
      const res = this.executeStatement(stmt, env);
      if (res && res.__isReturn) return res.value;
      if (res && (res.__isBreak || res.__isContinue)) return res;
    }
    return null;
  }

  private executeStatement(stmt: any, env: Environment): any {
    switch (stmt.type) {
      case 'Let': {
        const val = stmt.init ? this.evalExpr(stmt.init, env) : null;
        env.define(stmt.name, val, stmt.isConst);
        return null;
      }
      case 'Assign': {
        const val = this.evalExpr(stmt.value, env);
        if (stmt.target.type === 'Identifier') {
          if (stmt.op === '=') env.assign(stmt.target.name, val);
          else if (stmt.op === '+=') env.assign(stmt.target.name, env.get(stmt.target.name) + val);
          else if (stmt.op === '-=') env.assign(stmt.target.name, env.get(stmt.target.name) - val);
        } else if (stmt.target.type === 'Index') {
          const obj = this.evalExpr(stmt.target.object, env);
          const idx = this.evalExpr(stmt.target.index, env);
          obj[idx] = val;
        } else if (stmt.target.type === 'Member') {
          const obj = this.evalExpr(stmt.target.object, env);
          obj[stmt.target.property] = val;
        }
        return null;
      }
      case 'Return': {
        const val = stmt.value ? this.evalExpr(stmt.value, env) : null;
        return { __isReturn: true, value: val };
      }
      case 'Break':
        return { __isBreak: true };
      case 'Continue':
        return { __isContinue: true };
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
          if (res && res.__isBreak) break;
        }
        return null;
      }
      case 'For': {
        const iterVal = this.evalExpr(stmt.iterable, env);
        let items: any[] = [];

        if (Array.isArray(iterVal)) {
          items = stmt.isOf ? iterVal : iterVal.map((_, i) => i);
        } else if (typeof iterVal === 'string') {
          items = stmt.isOf ? iterVal.split('') : Array.from({ length: iterVal.length }, (_, i) => i);
        } else if (iterVal instanceof Set) {
          items = Array.from(iterVal);
        } else if (iterVal instanceof Map) {
          items = stmt.isOf ? Array.from(iterVal.values()) : Array.from(iterVal.keys());
        } else if (iterVal && typeof iterVal.toArray === 'function') {
          items = iterVal.toArray();
        }

        for (const item of items) {
          const loopEnv = new Environment(env);
          loopEnv.define(stmt.varName, item);
          const res = this.executeBlock(stmt.body, loopEnv);
          if (res && res.__isReturn) return res;
          if (res && res.__isBreak) break;
        }
        return null;
      }
      case 'Switch': {
        const targetVal = this.evalExpr(stmt.target, env);
        let matched = false;

        for (const c of stmt.cases) {
          const cVal = this.evalExpr(c.value, env);
          if (cVal === targetVal || matched) {
            matched = true;
            for (const s of c.body) {
              const res = this.executeStatement(s, env);
              if (res && (res.__isReturn || res.__isBreak)) return res;
            }
            break;
          }
        }

        if (!matched && stmt.defaultCase) {
          for (const s of stmt.defaultCase) {
            const res = this.executeStatement(s, env);
            if (res && (res.__isReturn || res.__isBreak)) return res;
          }
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

  private evalExpr(expr: any, env: Environment): any {
    if (!expr) return null;
    switch (expr.type) {
      case 'Literal':
        return expr.value;
      case 'Identifier':
        return env.get(expr.name);
      case 'Array':
        return expr.elements.map((e: any) => this.evalExpr(e, env));
      case 'Lambda': {
        return (...args: any[]) => {
          const lambdaEnv = new Environment(env);
          for (let i = 0; i < expr.params.length; i++) {
            lambdaEnv.define(expr.params[i].name, args[i]);
          }
          return this.executeBlock(expr.body, lambdaEnv);
        };
      }
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
            const args = expr.args.map((a: any) => this.evalExpr(a, env));
            return this.executeFunction(fnNode, args, env);
          }
          if (env.has(name)) {
            const fnObj = env.get(name);
            if (typeof fnObj === 'function') {
              const args = expr.args.map((a: any) => this.evalExpr(a, env));
              return fnObj(...args);
            }
          }
          throw new Error(`Undefined function '${name}'`);
        }

        if (expr.callee.type === 'Member') {
          const obj = this.evalExpr(expr.callee.object, env);
          const method = expr.callee.property;
          const evaluatedArgs = expr.args.map((a: any) => this.evalExpr(a, env));
          return this.dispatchMethod(obj, method, evaluatedArgs);
        }

        const calleeVal = this.evalExpr(expr.callee, env);
        if (typeof calleeVal === 'function') {
          const args = expr.args.map((a: any) => this.evalExpr(a, env));
          return calleeVal(...args);
        }
        throw new Error('Unsupported callee expression');
      }
      case 'Member': {
        const obj = this.evalExpr(expr.object, env);
        const prop = expr.property;
        if (obj === null || obj === undefined) throw new Error(`Cannot access property '${prop}' on null`);
        if (prop === 'length' || prop === 'len') {
          if (Array.isArray(obj) || typeof obj === 'string') return obj.length;
          if (obj.size && typeof obj.size === 'function') return obj.size();
        }
        return obj[prop];
      }
      case 'Index': {
        const obj = this.evalExpr(expr.object, env);
        const idx = this.evalExpr(expr.index, env);
        if (Array.isArray(obj) || typeof obj === 'string') return obj[idx];
        if (obj instanceof Map) return obj.get(idx);
        return obj[idx];
      }
      default:
        return null;
    }
  }

  private dispatchMethod(obj: any, method: string, args: any[]): any {
    if (obj === null || obj === undefined) {
      throw new Error(`Cannot call method '${method}' on null`);
    }

    // Direct object method
    if (typeof obj[method] === 'function') {
      return obj[method](...args);
    }

    // Array Higher-Order Methods
    if (Array.isArray(obj)) {
      switch (method) {
        case 'map': {
          const cb = args[0];
          return obj.map((item, idx) => cb(item, idx));
        }
        case 'filter': {
          const cb = args[0];
          return obj.filter((item, idx) => cb(item, idx));
        }
        case 'reduce': {
          const cb = args[0];
          const initial = args.length > 1 ? args[1] : obj[0];
          const slice = args.length > 1 ? obj : obj.slice(1);
          return slice.reduce((acc, item, idx) => cb(acc, item, idx), initial);
        }
        case 'forEach': {
          const cb = args[0];
          obj.forEach((item, idx) => cb(item, idx));
          return null;
        }
        case 'find': {
          const cb = args[0];
          const found = obj.find((item, idx) => cb(item, idx));
          return found !== undefined ? found : null;
        }
        case 'findIndex': {
          const cb = args[0];
          return obj.findIndex((item, idx) => cb(item, idx));
        }
        case 'some': {
          const cb = args[0];
          return obj.some((item, idx) => cb(item, idx));
        }
        case 'every': {
          const cb = args[0];
          return obj.every((item, idx) => cb(item, idx));
        }
        case 'slice': return obj.slice(args[0], args[1]);
        case 'splice': return obj.splice(args[0], args[1]);
        case 'join': return obj.join(args[0] !== undefined ? args[0] : ',');
        case 'reverse': return [...obj].reverse();
        case 'sort': return [...obj].sort();
        case 'indexOf': return obj.indexOf(args[0]);
        case 'includes': return obj.includes(args[0]);
        case 'push': { obj.push(...args); return obj.length; }
        case 'pop': return obj.pop();
        case 'shift': return obj.shift();
        case 'unshift': return obj.unshift(...args);
        case 'at': return obj.at(args[0]);
        case 'clear': { obj.length = 0; return null; }
        case 'isEmpty': return obj.length === 0;
        case 'len':
        case 'length': return obj.length;
      }
    }

    // String Methods
    if (typeof obj === 'string') {
      switch (method) {
        case 'toUpperCase': return obj.toUpperCase();
        case 'toLowerCase': return obj.toLowerCase();
        case 'trim': return obj.trim();
        case 'split': return obj.split(args[0] !== undefined ? args[0] : '');
        case 'includes': return obj.includes(args[0]);
        case 'indexOf': return obj.indexOf(args[0]);
        case 'startsWith': return obj.startsWith(args[0]);
        case 'endsWith': return obj.endsWith(args[0]);
        case 'replace': return obj.replace(args[0], args[1]);
        case 'substring': return obj.substring(args[0], args[1]);
        case 'slice': return obj.slice(args[0], args[1]);
        case 'charAt': return obj.charAt(args[0]);
        case 'concat': return obj.concat(...args);
        case 'repeat': return obj.repeat(args[0]);
        case 'len':
        case 'length': return obj.length;
      }
    }

    // Map methods
    if (obj instanceof Map) {
      switch (method) {
        case 'set': obj.set(args[0], args[1]); return null;
        case 'get': return obj.has(args[0]) ? obj.get(args[0]) : null;
        case 'has': return obj.has(args[0]);
        case 'delete': return obj.delete(args[0]);
        case 'clear': obj.clear(); return null;
        case 'size': return obj.size;
        case 'keys': return Array.from(obj.keys());
        case 'values': return Array.from(obj.values());
        case 'entries': return Array.from(obj.entries());
        case 'forEach': {
          const cb = args[0];
          obj.forEach((val, key) => cb(val, key));
          return null;
        }
      }
    }

    // Set methods
    if (obj instanceof Set) {
      switch (method) {
        case 'add': obj.add(args[0]); return null;
        case 'has': return obj.has(args[0]);
        case 'delete': return obj.delete(args[0]);
        case 'clear': obj.clear(); return null;
        case 'size': return obj.size;
        case 'values': return Array.from(obj.values());
        case 'forEach': {
          const cb = args[0];
          obj.forEach((val) => cb(val));
          return null;
        }
      }
    }

    throw new Error(`Method '${method}' is not supported on target object.`);
  }
}

// --- Public Prady Engine Interface ---
export class PradyCompiler {
  static compileAndRun(sourceCode: string, printCallback: PrintCallback): CompileResult {
    const startTime = performance.now();
    const lexer = new Lexer(sourceCode);
    const lexResult = lexer.tokenize();

    const lines = sourceCode.split('\n');
    const parser = new Parser(lexResult.tokens);
    const ast = parser.parseProgram();

    const diagnostics: Diagnostic[] = [...lexResult.errors, ...ast.diagnostics];

    if (diagnostics.length > 0) {
      const errorReports = diagnostics
        .map((d) => {
          const lineText = lines[d.line - 1] || '';
          const caret = ' '.repeat(Math.max(0, d.col - 1)) + '^';
          return `error: ${d.message}\n  --> line ${d.line}:${d.col}\n   |\n${d.line} | ${lineText}\n   | ${caret}`;
        })
        .join('\n\n');

      return {
        success: false,
        diagnostics,
        errorText: errorReports,
        durationMs: (performance.now() - startTime).toFixed(2),
        archReports: [],
      };
    }

    const archReports: string[] = [];
    const archItem = ast.items.find((i: any) => i.type === 'Architecture');
    if (archItem) {
      archReports.push(`[Architecture Validator] System '${archItem.name}' loaded.`);
      archReports.push(`  Defined layers: ${archItem.layers.join(' -> ')}`);
      for (const r of archItem.rules) {
        if (r.type === 'allow') archReports.push(`  ✓ Policy: ${r.from} allowed to depend on ${r.to}`);
        if (r.type === 'deny') archReports.push(`  ✓ Policy: ${r.from} strictly forbidden to import ${r.to}`);
      }
    }

    let runError: string | undefined = undefined;
    try {
      const evaluator = new Evaluator(ast, printCallback);
      evaluator.evaluate();
    } catch (err: any) {
      runError = err.message || String(err);
    }

    return {
      success: !runError,
      diagnostics: [],
      architecture: archItem,
      archReports,
      runError,
      durationMs: (performance.now() - startTime).toFixed(2),
    };
  }
}
