import {
  HighlightStyle,
  syntaxHighlighting,
  LRLanguage,
  LanguageSupport,
  foldNodeProp,
} from "@codemirror/language";
import { tags } from "@lezer/highlight";
import { styleTags } from "@lezer/highlight";
import { parser } from "./tei-dsl-parser.js";
import { syntaxTree } from "@codemirror/language";

/**
 * TEI-DSL syntax highlighting styles using CSS classes for theme compatibility
 */
export const teiDslHighlightStyle = HighlightStyle.define([
  // Keywords/commands like .abbr
  { tag: tags.keyword, class: "cmt-keyword" },
  // Content in brackets [text] {text}
  { tag: tags.string, class: "cmt-string" },
  // Delimiters like // ///
  { tag: tags.processingInstruction, class: "cmt-processing" },
  // Notes ^{...}
  { tag: tags.comment, class: "cmt-comment" },
  // Unclear ?{...}?
  { tag: tags.emphasis, class: "cmt-emphasis" },
  // Supplied <...>
  { tag: tags.inserted, class: "cmt-inserted" },
  // Deleted -{...}-
  { tag: tags.deleted, class: "cmt-deleted" },
  // Added +{...}+
  { tag: tags.changed, class: "cmt-changed" },
  // Entities :name:
  { tag: tags.atom, class: "cmt-atom" },
  // Word markers ~ |
  { tag: tags.separator, class: "cmt-separator" },
  // Gap [...]
  { tag: tags.special(tags.string), class: "cmt-gap" },
]);

/**
 * Configure the parser with syntax highlighting tags and fold regions
 */
const parserWithMetadata = parser.configure({
  props: [
    styleTags({
      // Breaks and continuations
      PageBreak: tags.processingInstruction,
      LineBreak: tags.processingInstruction,
      WordContinuationPageBreak: tags.separator,
      WordContinuationLineBreak: tags.separator,

      // Abbreviation
      "Abbreviation/...": tags.keyword,
      BracketContent: tags.string,
      BraceContent: tags.string,

      // Gap
      Gap: tags.special(tags.string),

      // Supplied
      Supplied: tags.inserted,
      SuppliedContent: tags.inserted,

      // Deletion
      Deletion: tags.deleted,
      DeletionContent: tags.deleted,

      // Addition
      Addition: tags.changed,
      AdditionContent: tags.changed,

      // Note
      Note: tags.comment,
      NoteContent: tags.comment,

      // Unclear
      Unclear: tags.emphasis,
      UnclearContent: tags.emphasis,

      // Entity
      Entity: tags.atom,

      // Word boundary and compound join
      WordBoundary: tags.separator,
      CompoundJoin: tags.separator,
    }),
    // Fold regions at page breaks - each page break folds content until next page break
    foldNodeProp.add({
      PageBreak: (node, state) => {
        // Get the end of the current line (after the page break marker)
        const lineEnd = state.doc.lineAt(node.to).to;

        // Search for the next PageBreak in the document
        const tree = syntaxTree(state);
        let nextPageBreakStart: number | null = null;

        tree.iterate({
          from: node.to,
          enter: (iterNode) => {
            if (iterNode.name === "PageBreak" && iterNode.from > node.to) {
              // Found next page break - get start of its line
              nextPageBreakStart = state.doc.lineAt(iterNode.from).from;
              return false; // Stop iteration
            }
          }
        });

        // If we found another page break, fold to just before it
        // Otherwise fold to end of document
        const foldEnd = nextPageBreakStart ?? state.doc.length;

        // Only create fold if there's content to fold (at least one line)
        if (foldEnd > lineEnd) {
          return { from: lineEnd, to: foldEnd };
        }
        return null;
      }
    }),
  ],
});

/**
 * TEI-DSL language definition using Lezer parser
 * Note: Type assertion used to handle version mismatch between
 * @lezer/lr and @codemirror/language's nested @lezer/lr dependency
 */
export const teiDslLanguage = LRLanguage.define({
  parser: parserWithMetadata as any,
  languageData: {
    commentTokens: { block: { open: "^{", close: "}" } },
  },
});

/**
 * Full language support including the parser and highlighting
 */
export const teiDsl = new LanguageSupport(teiDslLanguage);

/**
 * Syntax highlighting extension
 */
export const teiDslHighlighting = syntaxHighlighting(teiDslHighlightStyle);
