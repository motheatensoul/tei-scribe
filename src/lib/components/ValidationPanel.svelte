<script lang="ts">
    import {
        validationStore,
    } from "$lib/stores/validation.svelte";
    import { validateXml } from "$lib/tauri";
    import { errorStore } from "$lib/stores/errors.svelte";

    // Icons
    import {
        CircleCheck,
        TriangleAlert,
        Loader,
        X as CloseIcon,
    } from "@lucide/svelte";

    let {
        xmlContent = "",
        onclose,
        onselecterror,
    }: {
        xmlContent?: string;
        onclose?: () => void;
        onselecterror?: (line: number, column: number) => void;
    } = $props();

    async function handleSchemaChange(e: Event) {
        const select = e.target as HTMLSelectElement;
        validationStore.selectSchema(select.value);
    }

    async function runValidation() {
        if (!xmlContent) return;

        const schemaId = validationStore.selectedSchemaId;
        validationStore.startValidation();

        try {
            const result = await validateXml(xmlContent, schemaId);
            validationStore.setResultKeepSchemas(result);
        } catch (e) {
            const msg = String(e);
            validationStore.setError(msg);
            errorStore.error("Validation", "XML validation failed", msg);
        }
    }

    // Run validation automatically when content or schema changes if autoPreview is on?
    // For now, let's keep it manual as it can be slow for large files
</script>

<div class="bg-base-100 text-base-content font-mono text-sm h-full flex flex-col">
    <div class="flex justify-between items-center p-3 border-b border-base-300">
        <h2 class="flex items-center gap-2 font-bold">
            XML Validation
            {#if validationStore.lastResult?.valid === true}
                <CircleCheck class="text-success" size={16} />
            {:else if validationStore.lastResult?.valid === false}
                <CloseIcon class="text-error" size={16} />
            {/if}
        </h2>
        <div class="flex gap-2 items-center">
            <select
                class="select select-xs select-bordered bg-base-100 text-base-content"
                value={validationStore.selectedSchemaId}
                onchange={handleSchemaChange}
                disabled={validationStore.isValidating}
            >
                {#each validationStore.availableSchemas as schema (schema.id)}
                    <option value={schema.id}>{schema.name}</option>
                {/each}
                {#if validationStore.availableSchemas.length === 0}
                    <option value="tei-p5">TEI P5</option>
                {/if}
            </select>
            <button
                class="btn btn-primary btn-sm"
                onclick={runValidation}
                disabled={validationStore.isValidating || !xmlContent}
            >
                {#if validationStore.isValidating}
                    <Loader class="animate-spin" size={14} />
                {:else}
                    Validate
                {/if}
            </button>
            <button
                class="btn btn-ghost btn-sm btn-circle"
                onclick={onclose}
                aria-label="Close"
            >
                <CloseIcon size={16} />
            </button>
        </div>
    </div>

    <div class="overflow-y-auto flex-1 p-2">
        {#if validationStore.isValidating}
            <div class="text-center py-8 flex items-center justify-center gap-2">
                <Loader class="animate-spin" size={20} />
                <span>Validating...</span>
            </div>
        {:else if validationStore.error}
            <div class="p-4 bg-error/20 rounded text-error">
                {validationStore.error}
            </div>
        {:else if validationStore.lastResult}
            {#if validationStore.lastResult.valid}
                <div class="p-4 flex items-center gap-2 text-success">
                    <CircleCheck size={20} />
                    <span
                        >Document is valid against {validationStore.lastResult.schemaName}</span
                    >
                </div>
            {:else}
                <div class="mb-2 flex gap-4 text-xs">
                    <span class="flex items-center gap-1 text-error">
                        <CloseIcon size={14} />
                        {validationStore.validationCounts.errors} error(s)
                    </span>
                    {#if validationStore.validationCounts.warnings > 0}
                        <span class="flex items-center gap-1 text-warning">
                            <TriangleAlert size={14} />
                            {validationStore.validationCounts.warnings} warning(s)
                        </span>
                    {/if}
                </div>

                {#each validationStore.lastResult.errors as error, i}
                    <button
                        class="w-full text-left mb-2 p-2 rounded border border-base-300 hover:bg-base-200 transition-colors group"
                        onclick={() =>
                            onselecterror?.(error.line || 1, error.column || 1)}
                    >
                        <div class="flex justify-between items-start mb-1">
                            <span
                                class="badge badge-sm {error.isWarning
                                    ? 'badge-warning'
                                    : 'badge-error'}"
                            >
                                {error.isWarning ? "Warning" : "Error"}
                            </span>
                            <span class="text-[10px] opacity-50"
                                >Line {error.line}, Col {error.column}</span
                            >
                        </div>
                        <div class="text-xs break-words">
                            {error.message}
                        </div>
                    </button>
                {/each}
            {/if}
        {:else}
            <div class="text-center py-12 opacity-50 italic">
                No validation results yet.<br />Click Validate to check the document.
            </div>
        {/if}
    </div>
</div>
