<script lang="ts">
    import { onMount } from 'svelte';
    import { entityStore } from '$lib/stores/entities.svelte';
    import { inflectionStore } from '$lib/stores/dictionary.svelte';
    import { applyXsltTransform } from '$lib/utils/xslt';

    let {
        content = '',
        xslPath = '/xsl/simple.xsl',
        onwordclick,
    }: {
        content?: string;
        xslPath?: string;
        onwordclick?: (facsimile: string, diplomatic: string, wordIndex: number, element: HTMLElement) => void;
    } = $props();

    let renderedHtml = $state('');
    let error = $state<string | null>(null);
    let xslText = $state<string | null>(null);
    let xslProcessor = $state<XSLTProcessor | null>(null);
    let containerEl: HTMLDivElement;

    // Load the XSL stylesheet once on mount
    onMount(async () => {
        try {
            const response = await fetch(xslPath);
            if (!response.ok) {
                throw new Error(`Failed to load stylesheet: ${response.statusText}`);
            }
            xslText = await response.text();

            const parser = new DOMParser();
            const xslDoc = parser.parseFromString(xslText, 'application/xml');

            const parseError = xslDoc.querySelector('parsererror');
            if (parseError) {
                throw new Error(`XSL parse error: ${parseError.textContent}`);
            }

            const processor = new XSLTProcessor();
            processor.importStylesheet(xslDoc);
            xslProcessor = processor;
        } catch (e) {
            error = `Failed to load XSLT: ${e}`;
            console.error(error);
        }
    });

    // Transform XML when content or processor changes
    $effect(() => {
        if (!content.trim()) {
            renderedHtml = '';
            return;
        }

        async function transform() {
            try {
                error = null;
                // DEBUG: Log what we're receiving
                console.log('[XsltRenderer] Content length:', content.length);
                console.log('[XsltRenderer] First 200 chars:', content.substring(0, 200));
                console.log('[XsltRenderer] Starts with <:', content.trim().startsWith('<'));
                renderedHtml = await applyXsltTransform(content, xslText || '', entityStore.entities);
            } catch (e) {
                error = `Transform error: ${e}`;
                console.error(error);
                console.error('[XsltRenderer] Content that failed:', content.substring(0, 500));
                renderedHtml = '';
            }
        }

        if (xslText) {
            transform();
        }
    });

    // Set up word click handlers after rendering
    $effect(() => {
        if (!containerEl || !renderedHtml || !onwordclick) return;

        // Find all word elements with data attributes
        const wordElements = containerEl.querySelectorAll('[data-word-index]');

        wordElements.forEach((el) => {
            const element = el as HTMLElement;
            const wordIndex = parseInt(element.dataset.wordIndex || '-1', 10);
            const diplomatic = element.dataset.diplomatic || element.textContent || '';
            const facsimile = element.textContent || '';

            element.onclick = (e) => {
                e.preventDefault();
                onwordclick(facsimile, diplomatic, wordIndex, element);
            };

            // Add styling based on lemmatization state
            if (inflectionStore.hasInflection(diplomatic)) {
                 // We could check if it's confirmed in the session too, but for now just show if it's in dictionary
                 element.classList.add('has-suggestion');
            }
        });
    });
</script>

<div class="xslt-rendered p-4 font-serif text-lg leading-loose" bind:this={containerEl}>
    {#if error}
        <div class="alert alert-error">
            <span>{error}</span>
        </div>
    {:else if !xslProcessor}
        <div class="flex items-center justify-center h-32">
            <span class="loading loading-spinner loading-md"></span>
            <span class="ml-2">Loading stylesheet...</span>
        </div>
    {:else if !renderedHtml}
        <div class="text-base-content/50 italic">(No content to render)</div>
    {:else}
        {@html renderedHtml}
    {/if}
</div>

<style>
    .xslt-rendered {
        font-family: 'Junicode', Georgia, serif;
    }

    /* Word styling - mirrors RenderedText.svelte */
    .xslt-rendered :global(.word) {
        display: inline;
        padding: 0.125rem 0.25rem;
        margin: 0 -0.125rem;
        border-radius: 0.25rem;
        border: 1px solid transparent;
        background: transparent;
        cursor: pointer;
        transition: all 0.15s ease;
    }

    .xslt-rendered :global(.word:hover) {
        border-color: var(--color-base-300);
        background-color: color-mix(in oklch, var(--color-base-200) 50%, transparent);
    }

    .xslt-rendered :global(.word.is-confirmed) {
        border-color: var(--color-success);
        background-color: color-mix(in oklch, var(--color-success) 15%, transparent);
    }

    .xslt-rendered :global(.word.has-suggestion) {
        border-color: color-mix(in oklch, var(--color-warning) 30%, transparent);
        opacity: 0.75;
    }

    .xslt-rendered :global(.word.has-suggestion:hover) {
        border-color: var(--color-warning);
        background-color: color-mix(in oklch, var(--color-warning) 15%, transparent);
        opacity: 1;
    }
</style>
