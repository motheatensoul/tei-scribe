<script lang="ts">
    import { errorStore, errorCounts } from '$lib/stores/errors';

    let { onclose }: { onclose?: () => void } = $props();

    function formatTime(date: Date): string {
        return date.toLocaleTimeString();
    }

    function getLevelIcon(level: string): string {
        switch (level) {
            case 'error':
                return '✕';
            case 'warning':
                return '⚠';
            case 'info':
                return 'ℹ';
            default:
                return '•';
        }
    }
</script>

<div class="bg-neutral text-neutral-content font-mono text-sm" data-theme="coffee">
    <div class="flex justify-between items-center p-3 border-b border-base-content/20">
        <h2 class="flex items-center gap-2 font-bold">
            Logs
            {#if $errorCounts.error > 0}
                <span class="badge badge-error badge-sm">{$errorCounts.error}</span>
            {/if}
            {#if $errorCounts.warning > 0}
                <span class="badge badge-warning badge-sm">{$errorCounts.warning}</span>
            {/if}
        </h2>
        <div class="flex gap-2">
            <button class="btn btn-ghost btn-xs" onclick={() => errorStore.clear()}>Clear</button>
            <button class="btn btn-ghost btn-xs" onclick={onclose}>×</button>
        </div>
    </div>

    <div class="overflow-y-auto max-h-96 p-2">
        {#each $errorStore.slice().reverse() as error (error.id)}
            <div class="grid grid-cols-[1.5rem_4rem_8rem_1fr] gap-2 p-1.5 rounded hover:bg-base-content/10 items-start">
                <span class="text-center" class:text-error={error.level === 'error'} class:text-warning={error.level === 'warning'} class:text-info={error.level === 'info'}>
                    {getLevelIcon(error.level)}
                </span>
                <span class="opacity-50">{formatTime(error.timestamp)}</span>
                <span class="text-primary">[{error.source}]</span>
                <span>{error.message}</span>
                {#if error.details}
                    <pre class="col-span-4 ml-8 mt-1 p-2 bg-base-content/5 rounded text-xs opacity-70 whitespace-pre-wrap break-all">{error.details}</pre>
                {/if}
            </div>
        {:else}
            <div class="text-center py-8 opacity-50">No log entries</div>
        {/each}
    </div>
</div>
