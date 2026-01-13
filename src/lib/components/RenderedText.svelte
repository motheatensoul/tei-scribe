<script lang="ts">
    import { entityStore } from '$lib/stores/entities.svelte';
    import type { EntityMap } from '$lib/types/entities';
    import { resolveEntitiesToGlyphs } from '$lib/utils/entities';
    import { inflectionStore } from '$lib/stores/dictionary.svelte';
    import { annotationStore, type AnnotationType } from '$lib/stores/annotations.svelte';
    import { validationStore } from '$lib/stores/validation.svelte';

    let {
        content = '',
        onwordclick,
    }: {
        content?: string;
        onwordclick?: (facsimile: string, diplomatic: string, wordIndex: number, element: HTMLElement, isSpanExtend?: boolean) => void;
    } = $props();

    interface TextToken {
        type: 'word' | 'punctuation' | 'linebreak' | 'pagebreak' | 'space' | 'text';
        displayText: string;  // What to show (facsimile with resolved entities)
        diplomatic?: string;  // For lemmatization lookup
        wordIndex?: number;   // Global word index (0-based)
        lineNum?: string;     // For line breaks
        pageNum?: string;     // For page breaks
        isSupplied?: boolean; // From <supplied>
        isDeletion?: boolean; // From <del>
        isAddition?: boolean; // From <add>
        isAbbr?: boolean;     // From <abbr>
        expansion?: string;   // From <expan>
    }

    interface ParsedDocument {
        tokens: TextToken[];
        pages: { 
            pageNum: string; 
            startIndex: number; 
            endIndex: number; 
        }[];
    }

    // Access entity store reactively at top level
    let entities = $derived(entityStore.entities);
    // Access validation errors
    let validationState = $derived(validationStore);

    // Async parsing state to avoid blocking UI
    let document = $state<ParsedDocument | null>(null);
    let isRendering = $state(false);
    let lastContent = $state('');
    let currentPageIndex = $state(0);

    // Pre-compute confirmed word set for O(1) lookups during render
    let confirmedWordSet = $derived(
        new Set(Object.keys(annotationStore.lemmaMappings).map(Number))
    );

    // Get set of annotation types for a word for badge rendering
    function getWordAnnotationTypes(wordIndex: number): Set<AnnotationType> {
        const anns = annotationStore.getForWord(wordIndex);
        return new Set(anns.map((a: { type: AnnotationType }) => a.type));
    }

    // Re-parse when content or entities change
    $effect(() => {
        if (content !== lastContent) {
            lastContent = content;
            parseXmlContent(content);
        }
    });

    async function parseXmlContent(xml: string) {
        if (!xml.trim()) {
            document = null;
            return;
        }

        isRendering = true;
        try {
            const parser = new DOMParser();
            const xmlDoc = parser.parseFromString(xml, 'text/xml');
            
            if (xmlDoc.querySelector('parsererror')) {
                // Return null or partial if parse error
                isRendering = false;
                return;
            }

            const body = xmlDoc.querySelector('body');
            if (body) {
                document = await extractTokensAsync(body);
                // Reset to first page if we're past the end
                if (currentPageIndex >= (document?.pages.length || 0)) {
                    currentPageIndex = 0;
                }
            }
        } catch (e) {
            console.error('Render parsing error:', e);
        } finally {
            isRendering = false;
        }
    }

    // Helper to yield periodically to keep UI responsive
    const yieldToMain = () => new Promise(resolve => setTimeout(resolve, 0));

    // Resolve entity placeholders to actual glyphs
    // Restored via import from utils/entities.ts

    // Async version of extractTokens that yields periodically
    async function extractTokensAsync(
        body: Element,
    ): Promise<ParsedDocument> {
        const tokens: TextToken[] = [];
        const pages: ParsedDocument['pages'] = [];
        let wordIndex = 0;
        let currentLine: string | undefined;
        let currentPage: string | undefined;
        let currentPageStartIndex = 0;

        // Recursively process nodes
        const processNode = async (node: Node) => {
            if (node.nodeType === 3) { // Text node
                const text = node.textContent || '';
                if (text.trim() || text.includes(' ')) {
                    const parts = text.split(/(\s+)/);
                    for (const part of parts) {
                        if (part.trim()) {
                            // It's text, but in TEI words are in <w>
                            // This text is likely whitespace or punctuation between elements
                            // if not inside <w>
                            if (node.parentElement?.localName !== 'w') {
                                tokens.push({
                                    type: 'text',
                                    displayText: resolveEntitiesToGlyphs(part, new Map(), entities),
                                });
                            }
                        } else if (part.includes(' ')) {
                            tokens.push({ type: 'space', displayText: ' ' });
                        }
                    }
                }
                return;
            }

            if (node.nodeType !== 1) return; // Only process elements

            const el = node as Element;
            // Use localName to strip namespace prefix (e.g., 'me:dipl' -> 'dipl')
            const tagName = el.localName;

            if (tagName === 'pb') {
                const n = el.getAttribute('n') || '?';
                
                // Finalize previous page
                if (pages.length > 0 || tokens.length > currentPageStartIndex) {
                    pages.push({
                        pageNum: currentPage || '1',
                        startIndex: currentPageStartIndex,
                        endIndex: tokens.length
                    });
                }
                
                currentPage = n;
                currentPageStartIndex = tokens.length;
                
                tokens.push({
                    type: 'pagebreak',
                    displayText: n,
                    pageNum: n
                });
            } else if (tagName === 'lb') {
                const n = el.getAttribute('n');
                currentLine = n || undefined;
                tokens.push({
                    type: 'linebreak',
                    displayText: n || '',
                    lineNum: n || undefined
                });
            } else if (tagName === 'w') {
                // Word element - contains facsimile (text/supplied/abbr) 
                // and diplomatic (me:dipl)
                
                // Find dipl element by localName to handle namespace prefix
                const diplEl = Array.from(el.children).find(c => c.localName === 'dipl');
                const diplomatic = diplEl?.textContent || '';
                
                // Build facsimile representation from children (excluding dipl/norm)
                let facsimile = '';
                let isSupplied = false;
                let isDeletion = false;
                let isAddition = false;
                let isAbbr = false;
                let expansion = '';

                for (const child of Array.from(el.childNodes)) {
                    if (child.nodeType === 3) {
                        facsimile += child.textContent;
                    } else if (child.nodeType === 1) {
                        const childEl = child as Element;
                        // Use localName to strip namespace prefix
                        const childTag = childEl.localName;
                        
                        if (childTag === 'supplied') {
                            facsimile += childEl.textContent;
                            isSupplied = true;
                        } else if (childTag === 'del') {
                            facsimile += childEl.textContent;
                            isDeletion = true;
                        } else if (childTag === 'add') {
                            facsimile += childEl.textContent;
                            isAddition = true;
                        } else if (childTag === 'choice') {
                            const abbr = childEl.querySelector('abbr');
                            const expan = childEl.querySelector('expan');
                            if (abbr) facsimile += abbr.textContent;
                            if (expan) {
                                isAbbr = true;
                                expansion = expan.textContent || '';
                            }
                        }
                    }
                }

                tokens.push({
                    type: 'word',
                    wordIndex: wordIndex++,
                    displayText: resolveEntitiesToGlyphs(facsimile, new Map(), entities),
                    diplomatic: diplomatic || facsimile,
                    isSupplied,
                    isDeletion,
                    isAddition,
                    isAbbr,
                    expansion
                });
            } else if (tagName === 'pc') {
                tokens.push({
                    type: 'punctuation',
                    displayText: resolveEntitiesToGlyphs(el.textContent || '', new Map(), entities)
                });
            } else if (tagName !== 'dipl' && tagName !== 'norm' && tagName !== 'facs') {
                // Recursively process other elements (like <body>, <p>, etc.)
                // Skip Menota level containers as we handle them inside <w>
                // localName already strips the 'me:' prefix so this works for both
                for (const child of Array.from(el.childNodes)) {
                    await processNode(child);
                }
            }
        };

        // Process all nodes and yield occasionally
        const allChildNodes = Array.from(body.childNodes);
        for (let i = 0; i < allChildNodes.length; i++) {
            await processNode(allChildNodes[i]);
            if (i % 50 === 0) await yieldToMain();
        }

        // Finalize last page
        if (tokens.length > currentPageStartIndex) {
            pages.push({
                pageNum: currentPage || (pages.length === 0 ? '1' : '?'),
                startIndex: currentPageStartIndex,
                endIndex: tokens.length
            });
        }

        return { tokens, pages };
    }

    function formatLemmaTooltip(token: TextToken): string | undefined {
        const word = token.diplomatic || token.displayText;
        const inflections = inflectionStore.getInflections(word);
        
        if (inflections.length === 0) return undefined;
        
        return inflections.map(i => `${i.lemma} (${i.analysis})`).join('\n');
    }

    const currentPage = $derived(document?.pages[currentPageIndex]);
    const pageTokens = $derived(currentPage ? document?.tokens.slice(currentPage.startIndex, currentPage.endIndex) : []);
</script>

<div class="h-full flex flex-col bg-base-100 text-base-content overflow-hidden rounded shadow-inner border border-base-300">
    <!-- Pagination Header -->
    {#if document && document.pages.length > 1}
        <div class="flex justify-between items-center px-4 py-2 bg-base-200 border-b border-base-300 shrink-0">
            <div class="flex gap-2 items-center">
                <button 
                    class="btn btn-xs btn-outline" 
                    disabled={currentPageIndex === 0}
                    onclick={() => currentPageIndex--}
                >
                    Prev
                </button>
                <span class="text-xs font-mono">
                    Page {currentPage?.pageNum || '?'} ({currentPageIndex + 1} / {document.pages.length})
                </span>
                <button 
                    class="btn btn-xs btn-outline" 
                    disabled={currentPageIndex === document.pages.length - 1}
                    onclick={() => currentPageIndex++}
                >
                    Next
                </button>
            </div>
            
            <div class="text-[10px] opacity-50 italic">
                {document.tokens.filter(t => t.type === 'word').length} words total
            </div>
        </div>
    {/if}

    <div class="flex-1 overflow-y-auto p-8 font-serif leading-relaxed text-xl selection:bg-primary/30" style="font-family: 'Junicode', serif;">
        {#if isRendering}
            <div class="flex flex-col items-center justify-center h-full opacity-50">
                <span class="loading loading-spinner loading-lg text-primary"></span>
                <p class="mt-4 font-sans text-sm italic">Rendering manuscript...</p>
            </div>
        {:else if !document}
            <div class="flex items-center justify-center h-full opacity-30 italic">
                No content to display
            </div>
        {:else}
            <div class="max-w-3xl mx-auto space-y-1">
                {#each pageTokens as token}
                    {#if token.type === 'word'}
                        {@const isConfirmed = confirmedWordSet.has(token.wordIndex!)}
                        {@const annotationTypes = getWordAnnotationTypes(token.wordIndex!)}
                        {@const hasSuggestions = inflectionStore.hasInflection(token.diplomatic || '')}
                        
                        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
                        <span 
                            class="word inline-block px-0.5 rounded cursor-pointer transition-colors relative group"
                            role="button"
                            tabindex="0"
                            class:is-confirmed={isConfirmed}
                            class:has-suggestion={!isConfirmed && hasSuggestions}
                            class:is-supplied={token.isSupplied}
                            class:is-deletion={token.isDeletion}
                            class:is-addition={token.isAddition}
                            onclick={(e) => onwordclick?.(token.displayText, token.diplomatic || token.displayText, token.wordIndex!, e.currentTarget as HTMLElement, e.shiftKey)}
                            title={formatLemmaTooltip(token)}
                        >
                            {token.displayText}
                            
                            <!-- Annotation Badges -->
                            {#if annotationTypes.size > 0}
                                <div class="absolute -top-2 left-0 flex gap-0.5 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity">
                                    {#each Array.from(annotationTypes) as type}
                                        <div 
                                            class="w-1.5 h-1.5 rounded-full" 
                                            class:bg-primary={type === 'lemma'}
                                            class:bg-secondary={type === 'semantic'}
                                            class:bg-accent={type === 'note'}
                                            class:bg-info={type === 'paleographic'}
                                            title={type}
                                        ></div>
                                    {/each}
                                </div>
                            {/if}
                        </span>
                    {:else if token.type === 'punctuation'}
                        <span class="punctuation">{token.displayText}</span>
                    {:else if token.type === 'space'}
                        <span class="space"> </span>
                    {:else if token.type === 'linebreak'}
                        <div class="flex items-center gap-4 my-2 opacity-30 select-none pointer-events-none no-print" style="font-family: sans-serif;">
                            <div class="h-px bg-current flex-1"></div>
                            <span class="text-[10px] font-mono whitespace-nowrap">Line {token.lineNum || ''}</span>
                            <div class="h-px bg-current flex-1"></div>
                        </div>
                    {:else if token.type === 'pagebreak'}
                        <div class="flex items-center gap-4 my-8 text-primary opacity-50 select-none pointer-events-none no-print" style="font-family: sans-serif;">
                            <div class="h-[2px] bg-current flex-1"></div>
                            <span class="text-xs font-bold font-mono whitespace-nowrap">PAGE {token.pageNum}</span>
                            <div class="h-[2px] bg-current flex-1"></div>
                        </div>
                    {:else}
                        {token.displayText}
                    {/if}
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .word {
        border-bottom: 1px solid transparent;
    }
    .word:hover {
        background-color: var(--color-base-200);
        border-bottom-color: var(--color-primary);
    }
    .is-confirmed {
        border-bottom: 2px solid var(--color-success);
    }
    .has-suggestion:not(.is-confirmed) {
        border-bottom: 1px dashed var(--color-primary);
    }
    .is-supplied {
        color: var(--color-info);
        font-style: italic;
    }
    .is-deletion {
        text-decoration: line-through;
        opacity: 0.6;
    }
    .is-addition {
        vertical-align: super;
        font-size: 0.85em;
        color: var(--color-secondary);
    }
    
    @media print {
        .no-print { display: none; }
        .word { border-bottom: none !important; }
    }
</style>
