<script lang="ts">
    import { onMount, tick } from "svelte";
    import {
        inflectionStore,
        annotationStore,
    } from "$lib/stores/dictionary.svelte";
    import { dictionaryStore } from "$lib/stores/dictionary.svelte";
    import type { OnpEntry, InflectedForm } from "$lib/tauri";
    import { lookupLemma, addInflection } from "$lib/tauri";
    import {
        Check,
        Search,
        Loader,
        History as HistoryIcon,
        ArrowRight,
        Save,
        AlertCircle,
        Info
    } from "@lucide/svelte";

    let {
        facsimile,
        diplomatic,
        wordIndex,
        onclose,
        onsave,
    }: {
        facsimile: string;
        diplomatic: string;
        wordIndex: number;
        onclose?: () => void;
        onsave?: (wordIndex: number, lemma?: string, msa?: string) => void;
    } = $props();

    let searchQuery = $state("");
    let isSearching = $state(false);
    let results = $state<OnpEntry[]>([]);
    let selectedEntry = $state<OnpEntry | null>(null);
    let selectedInflection = $state<InflectedForm | null>(null);
    let searchError = $state<string | null>(null);

    // Custom override state
    let customLemma = $state("");
    let customMsa = $state("");
    let useCustom = $state(false);

    // Check if this word instance already has a session confirmation
    let sessionConfirmation = $derived(annotationStore.getLemmaMapping(wordIndex));

    // Sync searchQuery when diplomatic prop changes
    $effect.pre(() => {
        searchQuery = diplomatic;
    });

    // Load existing mappings on mount
    $effect(() => {
        if (wordIndex >= 0) {
             loadExistingMappings();
        }
    });

    async function loadExistingMappings() {
        if (sessionConfirmation) {
            searchQuery = sessionConfirmation.lemma;
        } else {
            handleSearch();
        }
    }

    async function handleSearch() {
        if (!searchQuery.trim()) return;

        isSearching = true;
        searchError = null;
        try {
            results = await lookupLemma(searchQuery);
        } catch (e) {
            searchError = String(e);
        } finally {
            isSearching = false;
        }
    }

    async function confirmSelection() {
        if (useCustom) {
            if (!customLemma.trim()) return;
            
            annotationStore.confirmLemma(wordIndex, customLemma, customMsa);
            onsave?.(wordIndex, customLemma, customMsa);
            onclose?.();
            return;
        }

        if (selectedEntry) {
            // Selected inflection or use some defaults from entry
            const msaString = selectedInflection?.analysis || selectedEntry.part_of_speech.join(', ') || "unknown";
            const normForm = selectedInflection?.normalized || undefined;

            try {
                // Save to local inflection mapping dictionary (persistent)
                await addInflection(
                    diplomatic,
                    selectedEntry.id,
                    selectedEntry.lemma,
                    msaString,
                    selectedEntry.part_of_speech[0] || "unknown",
                    undefined,
                    undefined,
                    normForm,
                );

                // Update inflection store if we have one
                if (selectedInflection) {
                    inflectionStore.addMapping(diplomatic, selectedInflection);
                }

                // Save to session store (by word index) for this specific instance
                annotationStore.confirmLemma(wordIndex, selectedEntry.lemma, msaString, normForm);

                onsave?.(wordIndex, selectedEntry.lemma, msaString);
                onclose?.();
            } catch (e) {
                searchError = `Failed to save: ${e}`;
            }
        }
    }

    function selectEntry(entry: OnpEntry) {
        selectedEntry = entry;
        selectedInflection = null;
    }
</script>

<div class="flex flex-col gap-4 max-h-[70vh]">
    <div class="flex justify-between items-start">
        <div>
            <h3 class="font-bold text-lg flex items-center gap-2">
                <Search size={18} class="text-primary" />
                Lemmatization
            </h3>
            <div class="text-sm opacity-70 mt-1">
                Word: <span class="font-serif italic text-base">"{facsimile}"</span>
                {#if diplomatic !== facsimile}
                    <span class="mx-1">→</span>
                    <span class="font-serif italic text-base opacity-60">"{diplomatic}"</span>
                {/if}
            </div>
        </div>
        <button class="btn btn-ghost btn-sm btn-circle" onclick={onclose}>✕</button>
    </div>

    {#if sessionConfirmation}
        <div class="alert alert-success py-2 text-sm shadow-sm">
            <Check size={16} />
            <div>
                Confirmed: <strong>{sessionConfirmation.lemma}</strong>
                <span class="opacity-70 ml-1">({sessionConfirmation.msa})</span>
            </div>
            <button 
                class="btn btn-ghost btn-xs underline" 
                onclick={() => annotationStore.unconfirmLemma(wordIndex)}
            >
                Clear
            </button>
        </div>
    {/if}

    <div class="divider my-0"></div>

    <div class="tabs tabs-bordered">
        <button 
            class="tab" 
            class:tab-active={!useCustom} 
            onclick={() => (useCustom = false)}
        >
            Dictionary Search
        </button>
        <button 
            class="tab" 
            class:tab-active={useCustom} 
            onclick={() => (useCustom = true)}
        >
            Custom Entry
        </button>
    </div>

    {#if !useCustom}
        <div class="form-control w-full">
            <label class="label" for="lemma-search">
                <span class="label-text">Search ONP dictionary</span>
                {#if !dictionaryStore.loaded}
                    <span class="label-text-alt text-warning">Dictionary not loaded</span>
                {/if}
            </label>
            <div class="join w-full">
                <input
                    id="lemma-search"
                    type="text"
                    class="input input-bordered join-item flex-1"
                    placeholder="Enter lemma..."
                    bind:value={searchQuery}
                    onkeydown={(e) => e.key === "Enter" && handleSearch()}
                    disabled={!dictionaryStore.loaded}
                />
                <button
                    class="btn btn-primary join-item"
                    onclick={handleSearch}
                    disabled={isSearching || !dictionaryStore.loaded}
                >
                    {#if isSearching}
                        <Loader class="animate-spin" size={18} />
                    {:else}
                        <Search size={18} />
                    {/if}
                </button>
            </div>
        </div>

        {#if searchError}
            <div class="alert alert-error text-xs py-2">
                <AlertCircle size={14} />
                <span>{searchError}</span>
            </div>
        {/if}

        <div class="flex gap-4 overflow-hidden h-64">
            <!-- Results List -->
            <div class="flex-1 overflow-y-auto border border-base-300 rounded bg-base-200/30">
                {#if results.length > 0}
                    <ul class="menu menu-sm p-0">
                        {#each results as entry}
                            <li>
                                <button
                                    class="rounded-none border-b border-base-300 last:border-0 {selectedEntry?.id === entry.id ? 'active' : ''}"
                                    onclick={() => selectEntry(entry)}
                                >
                                    <div class="flex flex-col items-start gap-0">
                                        <span class="font-bold text-sm">{entry.lemma}</span>
                                        <span class="text-[10px] opacity-70 italic line-clamp-1">{entry.id} ({entry.part_of_speech.join(', ')})</span>
                                    </div>
                                </button>
                            </li>
                        {/each}
                    </ul>
                {:else if !isSearching}
                    <div class="h-full flex items-center justify-center opacity-30 italic text-sm text-center p-4">
                        No results. Try searching for a lemma.
                    </div>
                {/if}
            </div>

            <!-- Inflections placeholder -->
            <div class="w-1/2 overflow-y-auto border border-base-300 rounded bg-base-200/30">
                {#if selectedEntry}
                    <div class="p-2 bg-base-300 text-[10px] font-bold uppercase tracking-wider sticky top-0 z-10">
                        Select Morphology for {selectedEntry.lemma}
                    </div>
                    <div class="p-4 text-center opacity-50 italic text-xs">
                        Manual morphology selection for ONP results is coming soon. Use Custom Entry for specific analysis.
                    </div>
                {:else}
                    <div class="h-full flex items-center justify-center opacity-30 italic text-sm text-center p-4">
                        Select a lemma to see forms.
                    </div>
                {/if}
            </div>
        </div>
    {:else}
        <!-- Custom Entry Form -->
        <div class="space-y-4 py-4">
            <div class="form-control">
                <label class="label" for="custom-lemma">
                    <span class="label-text">Lemma</span>
                </label>
                <input
                    id="custom-lemma"
                    type="text"
                    class="input input-bordered"
                    placeholder="e.g. maðr"
                    bind:value={customLemma}
                />
            </div>
            <div class="form-control">
                <label class="label" for="custom-msa">
                    <span class="label-text">Morphological Analysis (MSA)</span>
                </label>
                <input
                    id="custom-msa"
                    type="text"
                    class="input input-bordered font-mono text-sm"
                    placeholder="e.g. n.m.nom.sg."
                    bind:value={customMsa}
                />
            </div>
            <div class="bg-info/10 p-3 rounded text-xs flex gap-2">
                <Info size={16} class="shrink-0" />
                <p>Custom entries are only saved for this manuscript instance.</p>
            </div>
        </div>
    {/if}

    <div class="divider my-0"></div>

    <div class="flex justify-end gap-2">
        <button class="btn btn-ghost" onclick={onclose}>Cancel</button>
        <button 
            class="btn btn-primary" 
            onclick={confirmSelection}
            disabled={(!useCustom && !selectedEntry) || (useCustom && !customLemma)}
        >
            <Save size={18} /> Confirm
        </button>
    </div>
</div>
