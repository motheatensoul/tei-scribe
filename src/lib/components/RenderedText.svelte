<script lang="ts">
    import { entityStore, type EntityMap } from '$lib/stores/entities';
    import { getInflections } from '$lib/stores/dictionary';

    let {
        content = '',
        onwordclick,
    }: {
        content?: string;
        onwordclick?: (word: string, element: HTMLElement) => void;
    } = $props();

    interface TextToken {
        type: 'word' | 'punctuation' | 'linebreak' | 'pagebreak' | 'space' | 'text';
        displayText: string;  // What to show (facsimile with resolved entities)
        diplomatic?: string;  // For lemmatization lookup
        lineNumber?: string;
        pageNumber?: string;
    }

    // Access entity store reactively at top level
    let entities = $derived($entityStore.entities);

    let tokens = $derived(parseXml(content, entities));

    function parseXml(xml: string, entityDefs: EntityMap): TextToken[] {
        if (!xml.trim()) return [];

        try {
            // Strip XML declaration if present (breaks when wrapped)
            let cleanXml = xml.replace(/<\?xml[^?]*\?>\s*/gi, '');

            // Replace entity references with placeholders to avoid XML parsing errors
            const entityPattern = /&([a-zA-Z][a-zA-Z0-9]*);/g;
            const entityMap = new Map<string, string>();
            let entityCounter = 0;

            const sanitized = cleanXml.replace(entityPattern, (match, name) => {
                const placeholder = `__ENTITY_${entityCounter}__`;
                entityMap.set(placeholder, name);
                entityCounter++;
                return placeholder;
            });

            // Wrap in a root element with namespace declarations
            const wrapped = `<root xmlns:me="http://www.menota.org/ns/1.0">${sanitized}</root>`;
            const parser = new DOMParser();
            const doc = parser.parseFromString(wrapped, 'text/xml');

            const parseError = doc.querySelector('parsererror');
            if (parseError) {
                console.warn('XML parse error:', parseError.textContent);
                return [{ type: 'text', displayText: xml }];
            }

            const result: TextToken[] = [];
            const root = doc.documentElement;

            // Find the body content - try multiple approaches for namespaced XML
            let body: Element | null = null;

            // Try querySelector first
            body = root.querySelector('body');

            // If not found, try getElementsByTagName (works better with namespaces)
            if (!body) {
                const bodies = root.getElementsByTagName('body');
                if (bodies.length > 0) {
                    body = bodies[0];
                }
            }

            // Fall back to text element
            if (!body) {
                body = root.querySelector('text') || root.getElementsByTagName('text')[0];
            }

            // Final fallback to root
            const target = body || root;

            extractTokens(target, result, entityMap, entityDefs);

            // If no tokens extracted, show a message
            if (result.length === 0) {
                return [{ type: 'text', displayText: '(No content in body)' }];
            }

            return result;
        } catch (e) {
            console.error('Parse error:', e);
            return [{ type: 'text', displayText: `Parse error: ${e}` }];
        }
    }

    // Resolve entity placeholders to actual glyphs
    function resolveEntitiesToGlyphs(text: string, entityMap: Map<string, string>, entityDefs: EntityMap): string {
        let result = text;
        for (const [placeholder, entityName] of entityMap) {
            // Look up the entity in our entity definitions
            const entity = entityDefs[entityName];
            const glyph = entity?.char || `[${entityName}]`;
            result = result.replaceAll(placeholder, glyph);
        }
        return result;
    }

    function extractTokens(node: Node, result: TextToken[], entityMap: Map<string, string>, entityDefs: EntityMap): void {
        for (const child of node.childNodes) {
            if (child.nodeType === Node.TEXT_NODE) {
                const text = child.textContent || '';
                if (text.trim()) {
                    // Plain text outside of elements - resolve entities for display
                    result.push({
                        type: 'text',
                        displayText: resolveEntitiesToGlyphs(text, entityMap, entityDefs),
                    });
                }
            } else if (child.nodeType === Node.ELEMENT_NODE) {
                const el = child as Element;
                const tagName = el.localName;

                switch (tagName) {
                    case 'w': {
                        // Word element - get facsimile for display, diplomatic for lookup
                        const facsEl = el.querySelector('me\\:facs, facs');
                        const diplEl = el.querySelector('me\\:dipl, dipl');

                        // Get raw content (with placeholders)
                        const facsRaw = facsEl?.textContent || el.textContent || '';
                        const diplRaw = diplEl?.textContent || facsRaw;

                        // Facsimile: resolve to glyphs for display
                        const displayText = resolveEntitiesToGlyphs(facsRaw, entityMap, entityDefs).trim();

                        // Diplomatic: resolve for lookup
                        const diplomatic = resolveEntitiesToGlyphs(diplRaw, entityMap, entityDefs).trim();

                        if (displayText) {
                            result.push({
                                type: 'word',
                                displayText,
                                diplomatic,
                            });
                            result.push({ type: 'space', displayText: ' ' });
                        }
                        break;
                    }
                    case 'pc': {
                        // Punctuation - same logic
                        const facsEl = el.querySelector('me\\:facs, facs');
                        const facsRaw = facsEl?.textContent || el.textContent || '';
                        const displayText = resolveEntitiesToGlyphs(facsRaw, entityMap, entityDefs).trim();

                        if (displayText) {
                            result.push({ type: 'punctuation', displayText });
                            result.push({ type: 'space', displayText: ' ' });
                        }
                        break;
                    }
                    case 'lb': {
                        // Line break with optional number
                        const n = el.getAttribute('n');
                        result.push({
                            type: 'linebreak',
                            displayText: '',
                            lineNumber: n || undefined,
                        });
                        break;
                    }
                    case 'pb': {
                        // Page break with folio number
                        const n = el.getAttribute('n');
                        result.push({
                            type: 'pagebreak',
                            displayText: '',
                            pageNumber: n || undefined,
                        });
                        break;
                    }
                    case 'gap': {
                        const quantity = el.getAttribute('quantity') || '?';
                        result.push({ type: 'text', displayText: `[…${quantity}]` });
                        break;
                    }
                    case 'supplied': {
                        const text = resolveEntitiesToGlyphs(el.textContent || '', entityMap, entityDefs);
                        result.push({ type: 'text', displayText: `⟨${text}⟩` });
                        break;
                    }
                    case 'unclear': {
                        const text = resolveEntitiesToGlyphs(el.textContent || '', entityMap, entityDefs);
                        result.push({ type: 'text', displayText: `${text}?` });
                        break;
                    }
                    case 'del': {
                        const text = resolveEntitiesToGlyphs(el.textContent || '', entityMap, entityDefs);
                        result.push({ type: 'text', displayText: `⟦${text}⟧` });
                        break;
                    }
                    case 'add': {
                        const text = resolveEntitiesToGlyphs(el.textContent || '', entityMap, entityDefs);
                        result.push({ type: 'text', displayText: `\\${text}/` });
                        break;
                    }
                    case 'choice': {
                        // Show abbreviation form (facsimile)
                        const abbr = el.querySelector('abbr')?.textContent || '';
                        const text = resolveEntitiesToGlyphs(abbr, entityMap, entityDefs);
                        if (text) {
                            result.push({ type: 'text', displayText: text });
                            result.push({ type: 'space', displayText: ' ' });
                        }
                        break;
                    }
                    default:
                        // Recurse into other elements (div, p, etc.)
                        extractTokens(el, result, entityMap, entityDefs);
                }
            }
        }
    }

    function handleWordClick(token: TextToken, event: MouseEvent) {
        const target = event.currentTarget as HTMLElement;
        // Use diplomatic form for lemmatization lookup
        onwordclick?.(token.diplomatic || token.displayText, target);
    }

    function hasKnownLemma(word: string): boolean {
        const inflections = $getInflections(word);
        return inflections.length > 0;
    }

    function formatLemmaTooltip(token: TextToken): string | undefined {
        const word = token.diplomatic || token.displayText;
        const inflections = $getInflections(word);

        if (inflections.length === 0) {
            if (token.diplomatic && token.diplomatic !== token.displayText) {
                return `diplomatic: ${token.diplomatic}`;
            }
            return undefined;
        }

        const parts: string[] = [];
        for (const inf of inflections) {
            parts.push(`${inf.lemma} (${inf.analysis})`);
        }

        let tooltip = parts.join('; ');
        if (token.diplomatic && token.diplomatic !== token.displayText) {
            tooltip = `diplomatic: ${token.diplomatic}\n${tooltip}`;
        }
        return tooltip;
    }
</script>

<div class="rendered-text p-4 font-serif text-lg leading-loose">
    {#each tokens as token, i}
        {#if token.type === 'word'}
            <button
                type="button"
                class="word-token"
                class:has-lemma={hasKnownLemma(token.diplomatic || token.displayText)}
                onclick={(e) => handleWordClick(token, e)}
                title={formatLemmaTooltip(token)}
            >
                {token.displayText}
            </button>
        {:else if token.type === 'punctuation'}
            <span class="punctuation">{token.displayText}</span>
        {:else if token.type === 'linebreak'}
            <!-- lb marks START of line, so br comes before (except at start or after pb) -->
            {#if i > 0 && tokens[i - 1]?.type !== 'pagebreak'}
                <br />
            {/if}
            <span class="line-number">{token.lineNumber || ''}</span>
        {:else if token.type === 'pagebreak'}
            <div class="pagebreak">
                <span class="page-indicator">{token.pageNumber || '?'}</span>
            </div>
        {:else if token.type === 'space'}
            {' '}
        {:else}
            <span class="plain-text">{token.displayText}</span>
        {/if}
    {/each}
</div>

<style>
    .rendered-text {
        font-family: 'Junicode', Georgia, serif;
    }

    .word-token {
        display: inline;
        padding: 0.125rem 0.25rem;
        margin: 0 -0.125rem;
        border-radius: 0.25rem;
        border: 1px solid transparent;
        background: transparent;
        cursor: pointer;
        transition: all 0.15s ease;
        font: inherit;
        color: inherit;
    }

    .word-token:hover {
        border-color: var(--color-base-300);
        background-color: color-mix(in oklch, var(--color-base-200) 50%, transparent);
    }

    .word-token:focus {
        outline: none;
        border-color: var(--color-primary);
        background-color: color-mix(in oklch, var(--color-primary) 10%, transparent);
    }

    .word-token.has-lemma {
        border-color: color-mix(in oklch, var(--color-success) 30%, transparent);
    }

    .word-token.has-lemma:hover {
        border-color: var(--color-success);
        background-color: color-mix(in oklch, var(--color-success) 15%, transparent);
    }

    .punctuation {
        color: var(--color-base-content);
    }

    .line-number {
        display: inline-block;
        min-width: 2rem;
        margin-right: 0.5rem;
        font-size: 0.75rem;
        color: var(--color-base-content);
        opacity: 0.4;
        font-family: monospace;
        text-align: right;
    }

    .pagebreak {
        display: block;
        margin: 1.5rem 0;
        text-align: center;
    }

    .page-indicator {
        display: inline-block;
        padding: 0.25rem 0.75rem;
        font-size: 0.875rem;
        color: var(--color-base-content);
        opacity: 0.4;
        font-family: monospace;
        border: 1px dashed currentColor;
        border-radius: 0.25rem;
    }

    .plain-text {
        white-space: pre-wrap;
    }
</style>
