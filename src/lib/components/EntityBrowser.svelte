<script lang="ts">
    import { entityStore, entityNames, type Entity } from '$lib/stores/entities';

    let { oninsert, onclose }: { oninsert?: (text: string) => void; onclose?: () => void } = $props();

    let searchQuery = $state('');
    let selectedCategory = $state<string | null>(null);

    // Get unique categories
    const categories = $derived.by(() => {
        const cats = new Set<string>();
        for (const entity of Object.values($entityStore.entities)) {
            if (entity.category) cats.add(entity.category);
        }
        return Array.from(cats).sort();
    });

    // Filter entities based on search and category
    const filteredEntities = $derived.by(() => {
        const query = searchQuery.toLowerCase();
        return Object.entries($entityStore.entities)
            .filter(([name, entity]) => {
                const matchesSearch =
                    !query ||
                    name.toLowerCase().includes(query) ||
                    entity.description.toLowerCase().includes(query) ||
                    entity.char.includes(query);
                const matchesCategory = !selectedCategory || entity.category === selectedCategory;
                return matchesSearch && matchesCategory;
            })
            .sort((a, b) => a[0].localeCompare(b[0]));
    });

    function handleInsert(name: string) {
        oninsert?.(`:${name}:`);
    }
</script>

<div>
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold">Entity Browser</h2>
        <button class="btn btn-ghost btn-sm" onclick={onclose}>×</button>
    </div>

    <div class="flex gap-2 mb-4">
        <input
            type="text"
            placeholder="Search entities..."
            bind:value={searchQuery}
            class="input input-bordered input-sm flex-1"
        />
        <select bind:value={selectedCategory} class="select select-bordered select-sm min-w-32">
            <option value={null}>All categories</option>
            {#each categories as category}
                <option value={category}>{category}</option>
            {/each}
        </select>
    </div>

    {#if !$entityStore.loaded}
        <div class="text-center py-8 opacity-70">Loading entities...</div>
    {:else if $entityStore.error}
        <div class="text-center py-8 text-error">{$entityStore.error}</div>
    {:else}
        <div class="overflow-y-auto max-h-96 -mx-2">
            {#each filteredEntities as [name, entity]}
                <button
                    class="grid grid-cols-[2rem_1fr] grid-rows-2 gap-x-2 gap-y-0.5 w-full p-2 text-left rounded-lg hover:bg-base-200 cursor-pointer"
                    onclick={() => handleInsert(name)}
                    title={entity.description}
                >
                    <span class="row-span-2 text-2xl flex items-center justify-center" style="font-family: 'Junicode', serif;">{entity.char}</span>
                    <span class="font-mono text-sm text-primary font-medium">:{name}:</span>
                    <span class="text-xs opacity-70 truncate">{entity.description}</span>
                </button>
            {:else}
                <div class="text-center py-8 opacity-70">No entities found</div>
            {/each}
        </div>
    {/if}

    <div class="pt-4 border-t border-base-300 text-xs opacity-70">
        {filteredEntities.length} entities
    </div>
</div>
