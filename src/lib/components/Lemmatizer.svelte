<script lang="ts">
    import { inflectionStore } from '$lib/stores/dictionary';
    import { dictionaryStore } from '$lib/stores/dictionary';
    import {
        lookupLemma,
        searchLemmaPrefix,
        fetchOnpFullEntry,
        addInflection,
        removeInflection,
        lookupInflection,
        type OnpEntry,
        type OnpFullEntry,
        type InflectedForm,
    } from '$lib/tauri';

    let {
        word,
        onclose,
        onsave,
    }: {
        word: string;
        onclose?: () => void;
        onsave?: (wordform: string, lemma: string, analysis: string) => void;
    } = $props();

    // Search state - initialize with word prop
    let searchQuery = $state(word); // eslint-disable-line
    let searchResults = $state<OnpEntry[]>([]);
    let isSearching = $state(false);
    let searchTimeout: ReturnType<typeof setTimeout>;

    // Selected entry state
    let selectedEntry = $state<OnpEntry | null>(null);
    let fullEntry = $state<OnpFullEntry | null>(null);
    let isLoadingFull = $state(false);

    // Form state
    let analysis = $state('');

    // Existing mappings for this word
    let existingMappings = $state<InflectedForm[]>([]);

    // Load existing mappings on mount
    $effect(() => {
        loadExistingMappings();
    });

    async function loadExistingMappings() {
        try {
            existingMappings = await lookupInflection(word);
        } catch (e) {
            console.error('Failed to load existing mappings:', e);
        }
    }

    // Search as user types
    $effect(() => {
        if (searchQuery.length >= 1) {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => doSearch(), 200);
        } else {
            searchResults = [];
        }
    });

    async function doSearch() {
        if (!searchQuery.trim()) return;

        isSearching = true;
        try {
            // Try exact lemma lookup first
            let results = await lookupLemma(searchQuery);

            // If no exact match, try prefix search
            if (results.length === 0) {
                results = await searchLemmaPrefix(searchQuery, 20);
            }

            searchResults = results;
        } catch (e) {
            console.error('Search failed:', e);
            searchResults = [];
        } finally {
            isSearching = false;
        }
    }

    async function selectEntry(entry: OnpEntry) {
        selectedEntry = entry;
        fullEntry = null;
        isLoadingFull = true;

        try {
            fullEntry = await fetchOnpFullEntry(entry.id);
        } catch (e) {
            console.error('Failed to fetch full entry:', e);
        } finally {
            isLoadingFull = false;
        }
    }

    async function handleSave() {
        if (!selectedEntry || !analysis.trim()) return;

        try {
            await addInflection(
                word,
                selectedEntry.id,
                selectedEntry.lemma,
                analysis.trim(),
                selectedEntry.part_of_speech[0] || 'unknown'
            );

            // Update local store
            inflectionStore.addMapping(word, {
                onp_id: selectedEntry.id,
                lemma: selectedEntry.lemma,
                analysis: analysis.trim(),
                part_of_speech: selectedEntry.part_of_speech[0] || 'unknown',
            });

            onsave?.(word, selectedEntry.lemma, analysis.trim());
            onclose?.();
        } catch (e) {
            console.error('Failed to save inflection:', e);
        }
    }

    async function handleRemoveMapping(mapping: InflectedForm) {
        try {
            await removeInflection(word, mapping.onp_id, mapping.analysis);
            inflectionStore.removeMapping(word, mapping.onp_id, mapping.analysis);
            await loadExistingMappings();
        } catch (e) {
            console.error('Failed to remove mapping:', e);
        }
    }

    function formatPartOfSpeech(pos: string | string[]): string {
        const posStr = Array.isArray(pos) ? pos[0] : pos;
        const map: Record<string, string> = {
            commonNoun: 'noun',
            properNoun: 'prop.',
            verb: 'verb',
            adjective: 'adj.',
            adverb: 'adv.',
            adposition: 'prep.',
            numeral: 'num.',
            interjection: 'interj.',
            particle: 'part.',
            determiner: 'det.',
            coordinatingConjunction: 'conj.',
            subordinatingConjunction: 'subj.',
        };
        return map[posStr] || posStr;
    }
</script>

<div class="flex flex-col gap-4">
    <div class="flex items-center justify-between">
        <h3 class="font-bold text-lg">Lemmatize: <span class="text-primary">{word}</span></h3>
        <button type="button" class="btn btn-sm btn-ghost" onclick={onclose} aria-label="Close">✕</button>
    </div>

    {#if existingMappings.length > 0}
        <div class="bg-base-200 rounded-lg p-3">
            <h4 class="text-sm font-medium mb-2">Existing mappings</h4>
            <div class="flex flex-wrap gap-2">
                {#each existingMappings as mapping}
                    <div class="badge badge-lg gap-2">
                        <span class="font-medium">{mapping.lemma}</span>
                        <span class="opacity-70">({mapping.analysis})</span>
                        <button
                            type="button"
                            class="btn btn-ghost btn-xs"
                            onclick={() => handleRemoveMapping(mapping)}
                            aria-label="Remove mapping"
                        >
                            ✕
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}

    <div class="form-control">
        <label class="label" for="lemma-search">
            <span class="label-text">Search ONP dictionary</span>
            {#if !$dictionaryStore.loaded}
                <span class="label-text-alt text-warning">Dictionary not loaded</span>
            {/if}
        </label>
        <input
            id="lemma-search"
            type="text"
            class="input input-bordered"
            placeholder="Enter lemma..."
            bind:value={searchQuery}
            disabled={!$dictionaryStore.loaded}
        />
    </div>

    {#if isSearching}
        <div class="flex items-center justify-center py-4">
            <span class="loading loading-spinner loading-sm"></span>
            <span class="ml-2 text-sm opacity-70">Searching...</span>
        </div>
    {:else if searchResults.length > 0}
        <div class="max-h-48 overflow-y-auto border border-base-300 rounded-lg">
            <ul class="menu menu-sm p-0" role="listbox" aria-label="Search results">
                {#each searchResults as entry}
                    <li>
                        <button
                            type="button"
                            role="option"
                            aria-selected={selectedEntry?.id === entry.id}
                            class="flex justify-between"
                            class:active={selectedEntry?.id === entry.id}
                            onclick={() => selectEntry(entry)}
                        >
                            <span class="font-medium">{entry.lemma}</span>
                            <span class="badge badge-sm badge-ghost">
                                {formatPartOfSpeech(entry.part_of_speech)}
                            </span>
                        </button>
                    </li>
                {/each}
            </ul>
        </div>
    {:else if searchQuery.length >= 1}
        <p class="text-sm opacity-70 text-center py-2">No results found</p>
    {/if}

    {#if selectedEntry}
        <div class="bg-base-200 rounded-lg p-4">
            <div class="flex items-start justify-between mb-2">
                <div>
                    <h4 class="font-bold text-lg">{selectedEntry.lemma}</h4>
                    <span class="badge badge-sm">{formatPartOfSpeech(selectedEntry.part_of_speech)}</span>
                </div>
                <a
                    href={`https://onp.ku.dk/${selectedEntry.id}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="btn btn-xs btn-ghost"
                    title="Open in ONP"
                >
                    ↗
                </a>
            </div>

            {#if isLoadingFull}
                <div class="flex items-center py-2">
                    <span class="loading loading-spinner loading-xs"></span>
                    <span class="ml-2 text-sm opacity-70">Loading definition...</span>
                </div>
            {:else if fullEntry && fullEntry.senses.length > 0}
                <div class="text-sm opacity-80 max-h-32 overflow-y-auto">
                    {#each fullEntry.senses.slice(0, 3) as sense}
                        <p class="mb-1">{sense.definition}</p>
                    {/each}
                    {#if fullEntry.senses.length > 3}
                        <p class="italic">...and {fullEntry.senses.length - 3} more senses</p>
                    {/if}
                </div>
            {/if}

            <div class="form-control mt-4">
                <label class="label" for="analysis-input">
                    <span class="label-text">Morphological analysis</span>
                </label>
                <input
                    id="analysis-input"
                    type="text"
                    class="input input-bordered input-sm"
                    placeholder="e.g., nom.sg.f, pret.ind.3sg"
                    bind:value={analysis}
                />
                <div class="label">
                    <span class="label-text-alt">Case, number, gender, tense, mood, person...</span>
                </div>
            </div>
        </div>
    {/if}

    <div class="modal-action">
        <button type="button" class="btn btn-ghost" onclick={onclose}>Cancel</button>
        <button
            type="button"
            class="btn btn-primary"
            onclick={handleSave}
            disabled={!selectedEntry || !analysis.trim()}
        >
            Save
        </button>
    </div>
</div>
