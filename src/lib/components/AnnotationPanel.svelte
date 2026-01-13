<script lang="ts">
    import {
        annotationStore,
        getTypeLabel,
        getTypeIcon,
        type Annotation,
        type AnnotationType,
        type AnnotationValue,
    } from "$lib/stores/annotations.svelte";
    import { templateStore } from "$lib/stores/template.svelte";
    import { 
        Plus, 
        Trash2, 
        Info
    } from "@lucide/svelte";

    let {
        facsimile,
        diplomatic,
        wordIndex,
        spanEndIndex,
        onclose,
        onsave,
        onFindMatchingWords,
    }: {
        facsimile: string;
        diplomatic: string;
        wordIndex: number;
        spanEndIndex?: number | null;
        onclose?: () => void;
        onsave?: () => void;
        onFindMatchingWords?: (text: string) => number[];
    } = $props();

    // Is this a span annotation?
    let isSpan = $derived(spanEndIndex !== null && spanEndIndex !== undefined && spanEndIndex !== wordIndex);
    let startWord = $derived(isSpan ? Math.min(wordIndex, spanEndIndex!) : wordIndex);
    let endWord = $derived(isSpan ? Math.max(wordIndex, spanEndIndex!) : wordIndex);

    // Get annotations for this word
    let wordAnnotations = $derived(annotationStore.getForWord(wordIndex));

    // Form state for new annotation
    let selectedType = $state<AnnotationType>("semantic");
    let annotationValue = $state("");
    let semanticCategory = $state("person");
    let semanticSubcategory = $state("");
    let applyToAllMatching = $state(false);

    // Semantic categories constants
    const SEMANTIC_CATEGORIES = [
        { id: "person", label: "Person", subcategories: ["masculine-name", "feminine-name", "patronymic", "nickname", "title"] },
        { id: "place", label: "Place", subcategories: ["settlement", "region", "country", "toponym", "building"] },
        { id: "group", label: "Group/Organization", subcategories: ["family", "political", "religious"] },
        { id: "event", label: "Event", subcategories: ["battle", "assembly", "feast"] },
        { id: "object", label: "Object", subcategories: ["weapon", "ship", "manuscript", "clothing"] },
        { id: "date", label: "Date/Time", subcategories: ["year", "season", "holiday"] },
        { id: "ethnonym", label: "Ethnonym", subcategories: ["nationality", "tribe"] },
        { id: "other", label: "Other", subcategories: [] }
    ] as const;

    // Get matching word indices when bulk mode is available
    let matchingWordIndices = $derived.by(() => {
        if (!onFindMatchingWords || isSpan) return [];
        return onFindMatchingWords(diplomatic).filter(i => i !== wordIndex);
    });

    // Count of other matching words (excluding current)
    let matchingWordCount = $derived(matchingWordIndices.length);

    function handleAddAnnotation() {
        if (selectedType === "semantic" && !semanticCategory) return;
        if (selectedType === "note" && !annotationValue) return;

        const target = isSpan 
            ? { type: "span" as const, startWord, endWord } 
            : { type: "word" as const, wordIndex };

        let value: AnnotationValue;
        
        switch (selectedType) {
            case "semantic":
                value = { 
                    kind: "semantic", 
                    category: semanticCategory, 
                    subcategory: semanticSubcategory || undefined 
                };
                break;
            case "note":
                value = { kind: "note", text: annotationValue };
                break;
            default:
                value = { kind: "custom", value: annotationValue } as any;
        }

        const indices = [wordIndex];
        if (applyToAllMatching && !isSpan) {
            indices.push(...matchingWordIndices);
        }

        for (const idx of indices) {
            const ann: Annotation = {
                id: crypto.randomUUID(),
                type: selectedType,
                target: applyToAllMatching ? { type: "word", wordIndex: idx } : target,
                value,
                metadata: {
                    created: new Date().toISOString(),
                }
            };
            annotationStore.add(ann);
        }

        // Reset form
        annotationValue = "";
        onsave?.();
    }

    function removeAnnotation(id: string) {
        annotationStore.remove(id);
        onsave?.();
    }

    // Get subcategories for selected semantic category
    let availableSubcategories = $derived.by(() => {
        const cat = SEMANTIC_CATEGORIES.find(c => c.id === semanticCategory);
        return cat?.subcategories || [];
    });

    // Reset subcategory when category changes
    $effect(() => {
        if (semanticCategory) semanticSubcategory = "";
    });
</script>

<div class="flex flex-col gap-4">
    <div class="flex justify-between items-start">
        <div>
            <h3 class="font-bold text-lg flex items-center gap-2">
                <Info size={18} class="text-primary" />
                Annotations
            </h3>
            <div class="text-sm opacity-70 mt-1">
                {#if isSpan}
                    Span: <span class="font-serif italic">"{facsimile}"</span> (Words {startWord}-{endWord})
                {:else}
                    Word: <span class="font-serif italic">"{facsimile}"</span>
                {/if}
            </div>
        </div>
        <button class="btn btn-ghost btn-sm btn-circle" onclick={onclose}>✕</button>
    </div>

    <div class="divider my-0"></div>

    <!-- Existing Annotations -->
    <div class="space-y-2 max-h-60 overflow-y-auto pr-2">
        {#if wordAnnotations.length === 0}
            <div class="text-center py-4 opacity-50 italic text-sm">
                No annotations for this word yet.
            </div>
        {:else}
            {#each wordAnnotations as ann (ann.id)}
                {@const Icon = getTypeIcon(ann.type)}
                <div class="bg-base-200 rounded p-3 flex justify-between items-center group">
                    <div class="flex gap-3 items-start">
                        <div class="mt-1">
                            <Icon size={14} class="text-primary" />
                        </div>
                        <div>
                            <div class="text-[10px] font-bold uppercase tracking-wider opacity-50">
                                {getTypeLabel(ann.type)}
                            </div>
                            <div class="text-sm">
                                {#if ann.value.kind === "semantic"}
                                    <span class="font-medium">{ann.value.category}</span>
                                    {#if ann.value.subcategory}
                                        <span class="opacity-60 ml-1">({ann.value.subcategory})</span>
                                    {/if}
                                {:else if ann.value.kind === "note"}
                                    {ann.value.text}
                                {:else if ann.value.kind === "lemma"}
                                    <span class="font-mono">{ann.value.lemma}</span>
                                    <span class="opacity-60 ml-2 italic text-xs">{ann.value.msa}</span>
                                {/if}
                            </div>
                        </div>
                    </div>
                    <button 
                        class="btn btn-ghost btn-xs btn-circle text-error opacity-0 group-hover:opacity-100 transition-opacity"
                        onclick={() => removeAnnotation(ann.id)}
                        title="Remove annotation"
                    >
                        <Trash2 size={14} />
                    </button>
                </div>
            {/each}
        {/if}
    </div>

    <div class="divider my-0"></div>

    <!-- Add New Annotation -->
    <div class="bg-base-200/50 rounded-lg p-4">
        <h4 class="text-xs font-bold uppercase tracking-widest opacity-50 mb-3">Add New Annotation</h4>
        
        <div class="grid grid-cols-1 gap-4">
            <div class="form-control">
                <label class="label py-1" for="ann-type">
                    <span class="label-text text-xs">Type</span>
                </label>
                <select id="ann-type" class="select select-bordered select-sm" bind:value={selectedType}>
                    <option value="semantic">Semantic</option>
                    <option value="note">Note</option>
                    <option value="paleographic">Paleographic</option>
                    <option value="reference">Reference</option>
                    <option value="custom">Custom</option>
                </select>
            </div>

            {#if selectedType === 'semantic'}
                <div class="grid grid-cols-2 gap-2">
                    <div class="form-control">
                        <label class="label py-1" for="sem-cat">
                            <span class="label-text text-xs">Category</span>
                        </label>
                        <select id="sem-cat" class="select select-bordered select-sm" bind:value={semanticCategory}>
                            {#each SEMANTIC_CATEGORIES as cat}
                                <option value={cat.id}>{cat.label}</option>
                            {/each}
                        </select>
                    </div>
                    {#if availableSubcategories.length > 0}
                        <div class="form-control">
                            <label class="label py-1" for="sem-subcat">
                                <span class="label-text text-xs">Subcategory</span>
                            </label>
                            <select id="sem-subcat" class="select select-bordered select-sm" bind:value={semanticSubcategory}>
                                <option value="">None</option>
                                {#each availableSubcategories as sub}
                                    <option value={sub}>{sub}</option>
                                {/each}
                            </select>
                        </div>
                    {/if}
                </div>
            {:else}
                <div class="form-control">
                    <label class="label py-1" for="ann-value">
                        <span class="label-text text-xs">Value / Content</span>
                    </label>
                    <textarea 
                        id="ann-value"
                        class="textarea textarea-bordered textarea-sm leading-tight h-20" 
                        placeholder={selectedType === 'note' ? "Enter note text..." : "Enter annotation value..."}
                        bind:value={annotationValue}
                    ></textarea>
                </div>
            {/if}

            {#if matchingWordCount > 0 && !isSpan}
                <div class="form-control">
                    <label class="label cursor-pointer justify-start gap-3 py-1">
                        <input type="checkbox" class="checkbox checkbox-primary checkbox-xs" bind:checked={applyToAllMatching} />
                        <span class="label-text text-xs">Apply to all matching words ({matchingWordCount} other instances)</span>
                    </label>
                </div>
            {/if}

            <button class="btn btn-primary btn-sm mt-2" onclick={handleAddAnnotation}>
                <Plus size={16} /> Add Annotation
            </button>
        </div>
    </div>
</div>
