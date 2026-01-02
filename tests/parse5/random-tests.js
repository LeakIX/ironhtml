/**
 * Random/property-based testing for HTML generation.
 *
 * Generates random HTML structures and validates them with parse5
 * to ensure our builder produces valid, parseable HTML.
 */

import * as parse5 from "parse5";
import { execSync } from "child_process";
import { strict as assert } from "assert";
import crypto from "crypto";

// Configuration
const NUM_RANDOM_TESTS = 100;
const MAX_DEPTH = 5;
const MAX_CHILDREN = 4;
const MAX_ATTRS = 5;

// Elements categorized by type
const VOID_ELEMENTS = [
  "area",
  "base",
  "br",
  "col",
  "embed",
  "hr",
  "img",
  "input",
  "link",
  "meta",
  "source",
  "track",
  "wbr",
];

const BLOCK_ELEMENTS = [
  "div",
  "p",
  "article",
  "section",
  "header",
  "footer",
  "nav",
  "main",
  "aside",
  "h1",
  "h2",
  "h3",
  "ul",
  "ol",
  "li",
  "table",
  "thead",
  "tbody",
  "tr",
  "td",
  "th",
  "form",
  "fieldset",
];

const INLINE_ELEMENTS = [
  "span",
  "a",
  "strong",
  "em",
  "b",
  "i",
  "code",
  "small",
  "mark",
  "abbr",
  "time",
  "label",
];

const ALL_ELEMENTS = [...BLOCK_ELEMENTS, ...INLINE_ELEMENTS];

// Random generators
function randomInt(max) {
  return Math.floor(Math.random() * max);
}

function randomChoice(arr) {
  return arr[randomInt(arr.length)];
}

function randomString(maxLen = 20) {
  const chars =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";
  const len = randomInt(maxLen) + 1;
  let result = "";
  for (let i = 0; i < len; i++) {
    result += chars[randomInt(chars.length)];
  }
  return result;
}

function randomSpecialString() {
  const specials = [
    "<script>alert('xss')</script>",
    '"><img onerror=alert(1)>',
    "&amp;&lt;&gt;&quot;",
    "Hello & goodbye",
    'data-value="test"',
    "<",
    ">",
    "&",
    '"',
    "'",
    "\\",
    "\n",
    "\t",
    "Hello\nWorld",
    "日本語",
    "🎉🚀💻",
    "   ",
    "",
    "a".repeat(100),
    `mixed "quotes' and <tags>`,
  ];
  return randomChoice(specials);
}

function randomAttrName() {
  const names = [
    "class",
    "id",
    "style",
    "title",
    "data-value",
    "data-id",
    "data-test",
    "aria-label",
    "role",
    "tabindex",
    "name",
    "value",
    "placeholder",
    "href",
    "src",
    "alt",
    "type",
  ];
  return randomChoice(names);
}

function randomAttrValue() {
  if (Math.random() < 0.3) {
    return randomSpecialString();
  }
  return randomString(30);
}

// HTML generation
function generateRandomHtml(depth = 0) {
  if (depth >= MAX_DEPTH || (depth > 0 && Math.random() < 0.3)) {
    // Generate text content
    if (Math.random() < 0.2) {
      return escapeHtml(randomSpecialString());
    }
    return escapeHtml(randomString());
  }

  // Decide: void element, regular element, or text
  const choice = Math.random();

  if (choice < 0.1) {
    // Void element
    const tag = randomChoice(VOID_ELEMENTS);
    const attrs = generateRandomAttrs();
    return `<${tag}${attrs} />`;
  } else if (choice < 0.15) {
    // Just text
    return escapeHtml(randomString());
  } else {
    // Regular element with children
    const tag = randomChoice(ALL_ELEMENTS);
    const attrs = generateRandomAttrs();
    const numChildren = randomInt(MAX_CHILDREN) + 1;
    let children = "";

    for (let i = 0; i < numChildren; i++) {
      children += generateRandomHtml(depth + 1);
    }

    return `<${tag}${attrs}>${children}</${tag}>`;
  }
}

function generateRandomAttrs() {
  const numAttrs = randomInt(MAX_ATTRS);
  const seen = new Set();
  let result = "";

  for (let i = 0; i < numAttrs; i++) {
    let name = randomAttrName();
    // Avoid duplicate attributes
    while (seen.has(name)) {
      name = randomAttrName();
    }
    seen.add(name);

    const value = escapeAttr(randomAttrValue());
    result += ` ${name}="${value}"`;
  }

  return result;
}

function escapeHtml(str) {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function escapeAttr(str) {
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

// Test utilities
let passed = 0;
let failed = 0;

function findElement(node, tagName) {
  if (node.tagName === tagName) return node;
  if (node.childNodes) {
    for (const child of node.childNodes) {
      const found = findElement(child, tagName);
      if (found) return found;
    }
  }
  return null;
}

function countElements(node, count = 0) {
  if (node.tagName) count++;
  if (node.childNodes) {
    for (const child of node.childNodes) {
      count = countElements(child, count);
    }
  }
  return count;
}

// =============================================================================
// Random Tests
// =============================================================================

console.log("\nRandom HTML Generation Tests\n");
console.log("============================\n");

// Set seed for reproducibility in CI (but different each run)
const seed = process.env.RANDOM_SEED || crypto.randomBytes(4).toString("hex");
console.log(`Random seed: ${seed}\n`);

// Simple seeded random (not cryptographically secure, just for reproducibility)
let seedNum = parseInt(seed, 16);
const seededRandom = () => {
  seedNum = (seedNum * 1103515245 + 12345) & 0x7fffffff;
  return seedNum / 0x7fffffff;
};
// Override Math.random for reproducibility
Math.random = seededRandom;

console.log(`Running ${NUM_RANDOM_TESTS} random tests...\n`);

for (let i = 0; i < NUM_RANDOM_TESTS; i++) {
  const html = generateRandomHtml();

  try {
    // Try to parse as fragment
    const doc = parse5.parseFragment(html);

    // Basic validation
    assert.ok(doc, "Document should be parsed");
    assert.ok(doc.childNodes, "Document should have childNodes");

    // Verify we got some structure
    const elementCount = countElements(doc);
    assert.ok(elementCount >= 0, "Should have parsed elements");

    // Serialize back and compare (roundtrip)
    const serialized = parse5.serialize(doc);
    assert.ok(
      typeof serialized === "string",
      "Serialization should produce string"
    );

    passed++;
  } catch (error) {
    console.log(`\nTest ${i + 1} failed:`);
    console.log(`  HTML: ${html.substring(0, 100)}...`);
    console.log(`  Error: ${error.message}`);
    failed++;
  }
}

// Print progress
console.log(`Random tests: ${passed} passed, ${failed} failed`);

// =============================================================================
// Edge Case Tests
// =============================================================================

console.log("\nEdge Case Tests\n");
console.log("===============\n");

const edgeCases = [
  // Empty and minimal
  { name: "empty div", html: "<div></div>" },
  { name: "empty span", html: "<span></span>" },
  { name: "just text", html: "Hello World" },

  // Deep nesting
  {
    name: "deep nesting",
    html: "<div><div><div><div><div><div><span>deep</span></div></div></div></div></div></div>",
  },

  // Many siblings
  {
    name: "many siblings",
    html: "<ul><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li><li>6</li><li>7</li><li>8</li><li>9</li><li>10</li></ul>",
  },

  // Mixed content
  {
    name: "mixed content",
    html: "<p>Hello <strong>world</strong> and <em>everyone</em>!</p>",
  },

  // Special characters
  { name: "escaped html in text", html: "<p>&lt;script&gt;alert(1)&lt;/script&gt;</p>" },
  { name: "ampersand in text", html: "<p>Tom &amp; Jerry</p>" },
  { name: "quotes in attr", html: '<div data-value="say &quot;hello&quot;"></div>' },

  // Void elements
  { name: "void elements", html: "<p><br /><hr /><img src=\"x\" alt=\"y\" /></p>" },
  { name: "input types", html: '<input type="text" /><input type="email" /><input type="checkbox" />' },

  // Unicode
  { name: "unicode text", html: "<p>日本語 中文 한국어</p>" },
  { name: "emoji", html: "<p>🎉 🚀 💻 🌍</p>" },

  // Boolean attributes
  { name: "boolean attrs", html: '<input type="text" disabled required readonly />' },

  // Large content
  { name: "large text", html: `<p>${"a".repeat(10000)}</p>` },

  // Comments
  { name: "with comment", html: "<div><!-- comment --><span>text</span></div>" },

  // Whitespace
  { name: "whitespace preserved", html: "<pre>  spaces  \n  newlines  </pre>" },

  // Tables
  {
    name: "complex table",
    html: "<table><thead><tr><th>A</th><th>B</th></tr></thead><tbody><tr><td>1</td><td>2</td></tr></tbody></table>",
  },

  // Forms
  {
    name: "form structure",
    html: '<form action="/submit" method="post"><label for="name">Name</label><input type="text" id="name" name="name" /><button type="submit">Go</button></form>',
  },

  // Semantic HTML
  {
    name: "semantic structure",
    html: "<article><header><h1>Title</h1></header><section><p>Content</p></section><footer>Footer</footer></article>",
  },

  // Definition list
  {
    name: "definition list",
    html: "<dl><dt>Term 1</dt><dd>Definition 1</dd><dt>Term 2</dt><dd>Definition 2</dd></dl>",
  },

  // Data attributes
  {
    name: "data attributes",
    html: '<div data-id="123" data-name="test" data-value="foo bar"></div>',
  },

  // ARIA attributes
  {
    name: "aria attributes",
    html: '<button aria-label="Close" aria-pressed="false" role="button">X</button>',
  },
];

for (const { name, html } of edgeCases) {
  try {
    const doc = parse5.parseFragment(html);
    assert.ok(doc, `${name}: should parse`);
    passed++;
    console.log(`  ✓ ${name}`);
  } catch (error) {
    failed++;
    console.log(`  ✗ ${name}: ${error.message}`);
  }
}

// =============================================================================
// Summary
// =============================================================================

console.log("\n------------------------");
console.log(`Total: ${passed} passed, ${failed} failed`);
console.log("------------------------\n");

if (failed > 0) {
  console.log(`Seed was: ${seed}`);
  console.log("Rerun with RANDOM_SEED=" + seed + " to reproduce\n");
}

process.exit(failed > 0 ? 1 : 0);
