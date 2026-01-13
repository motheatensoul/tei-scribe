<script lang="ts">
    import { annotationStore } from "$lib/stores/annotations.svelte";
    import { dictionaryStore } from "$lib/stores/dictionary.svelte";
    import { X, BarChart2, PieChart, Hash, Tag } from "@lucide/svelte";
    import { onMount } from "svelte";

    let {
        onclose,
        previewContent
    }: {
        onclose?: () => void;
        previewContent: string;
    } = $props();

    // Word counts
    let wordCount = $state(0);
    let charCount = $state(0);
    
    // Calculate stats on mount and when preview changes
    $effect(() => {
        if (!previewContent) {
            wordCount = 0;
            charCount = 0;
            return;
        }

        // Simple estimation from text content
        const parser = new DOMParser();
        const doc = parser.parseFromString(previewContent, "text/xml");
        const text = doc.body.textContent || "";
        
        charCount = text.length;
        // Count words roughly by splitting on whitespace
        wordCount = text.trim().split(/\s+/).filter(s => s.length > 0).length;
    });

    // Annotation stats from store
    let annotationCounts = $derived(annotationStore.counts);
    let totalAnnotations = $derived(Object.values(annotationCounts).reduce((a, b) => a + b, 0));
    
    // Lemma stats
    let confirmedLemmas = $derived(annotationStore.confirmedLemmaIndices.size);
    let lemmaPercentage = $derived(wordCount > 0 ? Math.round((confirmedLemmas / wordCount) * 100) : 0);

    // Dictionary coverage (unique lemmas)
    let uniqueLemmas = $derived.by(() => {
        const lemmas = new Set<string>();
        const mappings = annotationStore.lemmaMappings;
        for (const key in mappings) {
            lemmas.add(mappings[key].lemma);
        }
        return lemmas.size;
    });

</script>

<div class="flex flex-col h-full bg-base-100 text-base-content">
    <div class="flex justify-between items-center p-4 border-b border-base-300">
        <h3 class="font-bold text-lg flex items-center gap-2">
            <BarChart2 class="text-primary" size={20} />
            Statistics
        </h3>
        <button class="btn btn-ghost btn-sm btn-circle" onclick={onclose}>
            <X size={20} />
        </button>
    </div>

    <div class="flex-1 overflow-y-auto p-4 space-y-6">
        <!-- General Text Stats -->
        <section>
            <h4 class="text-xs font-bold uppercase tracking-wider opacity-50 mb-3 flex items-center gap-2">
                <Hash size={12} /> Text Metrics
            </h4>
            <div class="stats shadow w-full stats-vertical lg:stats-horizontal">
                <div class="stat">
                    <div class="stat-title">Words</div>
                    <div class="stat-value">{wordCount}</div>
                    <div class="stat-desc">Approximate count</div>
                </div>
                <div class="stat">
                    <div class="stat-title">Characters</div>
                    <div class="stat-value text-secondary">{charCount}</div>
                    <div class="stat-desc">Excluding tags</div>
                </div>
            </div>
        </section>

        <div class="divider"></div>

        <!-- Annotation Stats -->
        <section>
            <h4 class="text-xs font-bold uppercase tracking-wider opacity-50 mb-3 flex items-center gap-2">
                <Tag size={12} /> Annotations
            </h4>
            
            <div class="grid grid-cols-2 gap-4 mb-4">
                <div class="card bg-base-200 p-4">
                    <div class="text-sm opacity-70">Total Annotations</div>
                    <div class="text-2xl font-bold">{totalAnnotations}</div>
                </div>
                <div class="card bg-base-200 p-4">
                    <div class="text-sm opacity-70">Lemmatized Words</div>
                    <div class="text-2xl font-bold text-primary">{confirmedLemmas}</div>
                    <div class="progress w-full h-1 mt-2 bg-base-300">
                        <div class="h-full bg-primary" style="width: {lemmaPercentage}%"></div>
                    </div>
                    <div class="text-xs mt-1 text-right">{lemmaPercentage}% of text</div>
                </div>
            </div>

            <h5 class="text-xs font-bold opacity-70 mb-2">Breakdown by Type</h5>
            <div class="overflow-x-auto">
                <table class="table table-xs w-full">
                    <thead>
                        <tr>
                            <th>Type</th>
                            <th class="text-right">Count</th>
                            <th class="text-right">%</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each Object.entries(annotationCounts) as [type, count]}
                            {#if count > 0}
                                <tr>
                                    <td class="capitalize">{type}</td>
                                    <td class="text-right font-mono">{count}</td>
                                    <td class="text-right font-mono opacity-70">
                                        {Math.round((count / totalAnnotations) * 100)}%
                                    </td>
                                </tr>
                            {/if}
                        {/each}
                    </tbody>
                </table>
            </div>
        </section>

        <div class="divider"></div>

        <!-- Vocabulary Stats -->
        <section>
            <h4 class="text-xs font-bold uppercase tracking-wider opacity-50 mb-3 flex items-center gap-2">
                <PieChart size={12} /> Vocabulary
            </h4>
            <div class="stats shadow w-full">
                <div class="stat">
                    <div class="stat-title">Unique Lemmas</div>
                    <div class="stat-value text-accent">{uniqueLemmas}</div>
                    <div class="stat-desc">Distinct dictionary entries used</div>
                </div>
            </div>
        </section>
    </div>
</div>
